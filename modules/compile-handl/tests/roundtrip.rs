//! Roundtrip tests for the Hughes arrow emitter.
//!
//! These tests lower the canonical `playground/*.hnl` files to `Term`, emit
//! them back to a handl-hughes source string, then parse *that* string with
//! the canonical parser and lower it again — the emitted text must produce
//! the original `Term`.
//!
//! The handl-hughes dialect *is* canonical handl surface syntax (ADR-0079
//! unified the arrow DSL onto the canonical language): `~>` lambdas,
//! `>>>`/`***`/`&&&` combinators, `arr <op>`, `{k: v}` objects and `expr.i`
//! field access. The parse side therefore goes through the same canonical
//! `lex`/`PrattEngine`/`lower_ast` pipeline the `.hnl` files do, so both
//! sides share one lowering contract.

use std::path::{Path, PathBuf};

use handl_kit_arrow_lower::lower_ast;
use handl_kit_data_prelude::language_from_yaml;
use handl_language::{Ast, AstRef, Language, Term};
use handl_plugin_compile_handl::HughesEmitter;
use hnl_engine_lex_pattern::lex;
use hnl_engine_parse::{ParserEngine, ParserRequest};
use hnl_engine_parse_pratt::PrattEngine;

fn canonical_lang() -> Language {
    language_from_yaml().expect("canonical language should assemble")
}

fn parse_canonical(src: &str, lang: &Language) -> Ast {
    let tokens = lex(lang, src).expect("canonical lex");
    let engine = PrattEngine::new();
    let req = ParserRequest::new(tokens);
    engine.parse(lang, &req).expect("canonical parse")
}

fn lower_src(src: &str, lang: &Language) -> Term {
    let ast = parse_canonical(src, lang);
    let root = AstRef::new(ast.len() - 1);
    lower_ast(&ast, lang, root).expect("lower to term")
}

fn lower_file(path: &Path, lang: &Language) -> (Term, String) {
    let src = std::fs::read_to_string(path).expect("read playground file");
    let term = lower_src(&src, lang);
    let emitted = HughesEmitter::default().emit(&term);
    (term, emitted)
}

fn playground(name: &str) -> PathBuf {
    // ADR-0128: `playground/` is its own repo now — the two roundtrip
    // fixtures are vendored under `tests/fixtures/` so this crate's
    // suite is self-contained.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Post-`d8f1e0b7` canonicalization. `x >>> f` with a *value-class* rhs
/// (a param, env slot, or projection) now lowers to `Op{flow, [l, r]}` —
/// the `eval.flow` dispatch — instead of `Seq`; arrow operands keep their
/// `Arr` quote inside the flow args (evaluated as arrow values at
/// runtime). Both denote the same sequential composition, so the
/// structural comparison normalizes `flow` back to `Seq`, unquoting
/// `Arr`-wrapped operands exactly as the lowerer's `unquote_term` does.
fn normalize(term: &Term) -> Term {
    match term {
        Term::Op { name, args, .. } if name == "flow" && args.len() == 2 => reassoc(Term::seq(
            unquote(normalize(&args[0])),
            unquote(normalize(&args[1])),
        )),
        Term::Seq { .. } => reassoc(map_children(term)),
        _ => map_children(term),
    }
}

/// Right-reassociate an axis-0 `Seq` chain — `>>>` is associative, so
/// `Seq{Seq{a,b},c}` and `Seq{a,Seq{b,c}}` are the same composition.
fn reassoc(term: Term) -> Term {
    let mut parts = Vec::new();
    let mut cur = term;
    while let Term::Seq { axis, fst, snd } = cur {
        if !axis.is_default_axis() {
            cur = Term::Seq { axis, fst, snd };
            break;
        }
        parts.push(*fst);
        cur = *snd;
    }
    // `parts` collected left-to-right; `cur` is the rightmost leaf.
    // Rebuild right-leaning: a >>> (b >>> (c >>> …)).
    let mut out = cur;
    for l in parts.into_iter().rev() {
        out = Term::seq(l, out);
    }
    out
}

/// Unquote an `Arr` whose payload serializes a `Term` (the arrow-value
/// quote `lower_ast` produces for operand positions).
fn unquote(term: Term) -> Term {
    if let Term::Arr { value, .. } = &term {
        if let Ok(inner) = serde_json::from_value::<Term>(value.clone()) {
            return normalize(&inner);
        }
    }
    term
}

/// Recursively normalize a term's immediate subterms.
fn map_children(term: &Term) -> Term {
    match term {
        Term::Seq { axis, fst, snd } => Term::Seq {
            axis: axis.clone(),
            fst: Box::new(normalize(fst)),
            snd: Box::new(normalize(snd)),
        },
        Term::Op {
            dim,
            name,
            args,
            source,
            lit,
        } => Term::Op {
            dim: dim.clone(),
            name: name.clone(),
            args: args.iter().map(normalize).collect(),
            source: source.clone(),
            lit: lit.clone(),
        },
        Term::App { callee, args } => Term::App {
            callee: callee.clone(),
            args: args.iter().map(normalize).collect(),
        },
        _ => term.clone(),
    }
}

fn assert_roundtrip(name: &str) {
    let lang = canonical_lang();
    let (original, emitted) = lower_file(&playground(name), &lang);
    eprintln!("emitted:\n{}", emitted);
    eprintln!("original: {:?}", original);
    let parsed = lower_src(&emitted, &lang);
    let (pn, on) = (normalize(&parsed), normalize(&original));
    if pn != on {
        std::fs::write("/tmp/normalized-parsed.txt", format!("{pn:#?}")).ok();
        std::fs::write("/tmp/normalized-orig.txt", format!("{on:#?}")).ok();
    }
    assert_eq!(pn, on);
}

#[test]
fn roundtrip_composition_hughes() {
    assert_roundtrip("composition-hughes.hnl");
}

#[test]
fn roundtrip_composition_emidio() {
    assert_roundtrip("composition-emidio.hnl");
}
