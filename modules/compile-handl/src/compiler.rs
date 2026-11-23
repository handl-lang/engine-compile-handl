//! The `HandlCompiler` implementation.

use handl_language::Term;
use handl_plugin_compile::{CompileError, Compiler, CompilerConfig, CompilerInput, CompilerOutput};
use hnl_engine_codegen::{ArtifactRef, Change, CodegenError, EmitContext, Emitter};

use crate::config::HandlConfig;
use crate::emit::{HughesEmitter, unsupported_variants};
use crate::output::HandlOutput;

/// Handl-to-Handl (Hughes arrow dialect) compiler.
#[derive(Debug, Default, Clone, Copy)]
pub struct HandlCompiler;

impl Compiler for HandlCompiler {
    fn compile(
        &self,
        input: &CompilerInput,
        config: &dyn CompilerConfig,
    ) -> Result<Box<dyn CompilerOutput>, CompileError> {
        let cfg = config
            .as_any()
            .downcast_ref::<HandlConfig>()
            .ok_or_else(|| CompileError::InvalidInput("expected HandlConfig".to_string()))?;

        if cfg.to != "hughes" {
            return Err(CompileError::UnsupportedTarget(cfg.to.clone()));
        }

        let src = HughesEmitter.emit(&input.term);
        Ok(Box::new(HandlOutput::new(src)))
    }

    fn target(&self) -> &str {
        "handl"
    }
}

/// `Emitter<Term>` over the desired-state codegen kernel (PL-004 T-092):
/// the same render as [`HandlCompiler::compile`], expressed as a
/// `Change` set (`main.hnl`) with no side effects. The legacy `Compiler`
/// impl stays until `handl-plugin-compile` migrates to the kernel
/// pipeline.
///
/// `verify` is the honest-decline gate: `HughesEmitter` silently emits
/// `"<term>"` for Term variants it cannot express, so any placeholder
/// in the desired state — or any unsupported variant in the IR — is a
/// `CodegenError::Verify`, never a silent degrade.
impl Emitter<Term> for HandlCompiler {
    fn materialize(&self, ir: &Term, req: &EmitContext) -> Result<Vec<Change>, CodegenError> {
        let to = req.config.get("to").map(String::as_str).unwrap_or("hughes");
        if to != "hughes" {
            return Err(CodegenError::Unsupported(format!(
                "handl compile target {to:?} (only \"hughes\" is supported)"
            )));
        }
        Ok(vec![Change::text(
            ArtifactRef::new("main.hnl"),
            HughesEmitter.emit(ir),
        )])
    }

    fn verify(&self, ir: &Term, changes: &[Change]) -> Result<(), CodegenError> {
        let placeholder = changes.iter().any(|c| {
            c.content()
                .is_some_and(|b| b.windows(b"<term>".len()).any(|w| w == b"<term>"))
        });
        if !placeholder {
            return Ok(());
        }
        let unsupported = unsupported_variants(ir);
        let detail = if unsupported.is_empty() {
            // `App`/`Seq` in arrow position also degrade to `"<term>"`;
            // `unsupported_variants` only names never-emittable variants.
            "term contains an App/Seq in arrow position".to_string()
        } else {
            format!("unsupported Term variant(s): {}", unsupported.join(", "))
        };
        Err(CodegenError::Verify(format!(
            "handl emitter produced \"<term>\" placeholder ({detail})"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn supported() -> Term {
        Term::op_native(
            "add",
            vec![
                Term::op_native("id", vec![]),
                Term::op_native_lit("elem", vec![], json!(0)),
            ],
        )
    }

    #[test]
    fn materialize_emits_main_hnl_matching_render() {
        let term = supported();
        let changes = HandlCompiler
            .materialize(&term, &EmitContext::new())
            .expect("materialize");
        assert_eq!(changes.len(), 1);
        match &changes[0] {
            Change::Put {
                artifact, content, ..
            } => {
                assert_eq!(artifact.as_str(), "main.hnl");
                assert_eq!(
                    String::from_utf8(content.clone()).unwrap(),
                    HughesEmitter.emit(&term)
                );
            }
            other => panic!("expected Put, got {other:?}"),
        }
        HandlCompiler
            .verify(&term, &changes)
            .expect("supported term verifies");
    }

    #[test]
    fn verify_rejects_unsupported_variants() {
        // `First` is not in the Hughes emitter's vocabulary — it emits
        // "<term>" and verify must decline honestly.
        let term = Term::First {
            inner: Box::new(Term::op_native("id", vec![])),
        };
        let changes = HandlCompiler
            .materialize(&term, &EmitContext::new())
            .expect("materialize still produces desired state");
        let err = HandlCompiler
            .verify(&term, &changes)
            .expect_err("unsupported variant must fail verify");
        match err {
            CodegenError::Verify(msg) => {
                assert!(
                    msg.contains("<term>"),
                    "message names the placeholder: {msg}"
                );
                assert!(msg.contains("First"), "message lists variants: {msg}");
            }
            other => panic!("expected Verify error, got {other}"),
        }
    }

    #[test]
    fn materialize_rejects_unknown_target() {
        let term = supported();
        let req = EmitContext::new().with_config("to", "other");
        let err = HandlCompiler
            .materialize(&term, &req)
            .expect_err("unknown target must decline");
        assert!(matches!(err, CodegenError::Unsupported(_)));
    }
}
