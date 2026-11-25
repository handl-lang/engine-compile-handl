# Agent guide — tool-compile-handl

Handl-to-Handl compiler — emits the Hughes arrow dialect of Handl
itself. `tool-compile-*` compiler-backend family: consumes Handl `Term`
arrow terms and lowers them back to canonical Handl source. Runnable
examples under `examples/`. Rust 2024.

## Layout

| path | role |
| --- | --- |
| `modules/compile-handl/` | the crate — `handl-plugin-compile-handl` (`src/lib.rs`, `compiler.rs`, `emit.rs`, `config.rs`, `output.rs`; `tests/roundtrip.rs`) |
| `examples/` | runnable compile examples |
| `xtask/` | vendored repo gate |
| `workflows/gate.yaml`, `bupha.toml`, `.cargo/` | CI wiring + `cargo xtask` alias |

## Contract

- Virtual-workspace root `Cargo.toml` (`members = ["modules/compile-handl"]`);
  `[patch]` tables map engine/kernel/tool-* git deps to sibling checkouts.
- Round-trip is the contract: `tests/roundtrip.rs` pins Term → `.hnl`
  → Term fidelity — extend emit coverage there, not ad-hoc.

## Verification

```bash
cargo xtask gate   # run at the workspace root
```


<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
