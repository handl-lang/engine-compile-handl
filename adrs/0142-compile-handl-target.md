# ADR-0142: `compile-handl` target

## Context

[ADR-0114](../0114-handl-compile-plugin-architecture.md) states the compile
port (`Compiler` / `CompilerRegistry`).
[ADR-0074](../0074-arrow-syntax-v0_1-surface-redesign.md) fixes the arrow
authoring surface, and [ADR-0084](../0084-arrow-macros.md) generalises the
desugaring that a Handl-to-Handl lowering builds on.

## Decision

`handl` owns the Handl-to-Handl compiler: `handl-plugin-compile-handl` (target
`handl`), which lowers the canonical surface to the Hughes arrow dialect —
here as `modules/compile-handl`.

This repo also owns `examples/` — the corpus this target is exercised against.
Every file there is plain surface syntax the compiler must accept, so the
corpus is the target's conformance input, not decoration.

`platform` keeps the port and the registry seam.

## Consequences

- `hnl compile handl` goes through the registry; the target name is unchanged.
  The repo is named `handl/` (the platform repo is `platform/`) so `hnl` mode
  is unambiguous — see ADR-0128 §"Repo naming".
- Adding an example is adding compiler coverage; the two artefacts evolve
  together.
- A surface-syntax change (ADR-0074/0084) is now a multi-repo change across
  `platform` (grammar data) and `handl` (the compiler) plus a rev bump.

## Status

Accepted — extracted in ADR-0128 Phase 7.
