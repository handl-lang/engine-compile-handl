# Task 0128 — `handl/` extraction (ADR-0128 Phase 7)

Expand ADR-0128 Phase 7 for this repo. The docs are already on
this `project` branch; the code lands with the extraction.

## Docs owned here

(none)

## Extraction

- [x] Follow ADR-0128 Phase 7: create the repo's workspace,
      move the crates/packages named there, wire the git dep + `[patch]`.
      **Done:** `modules/compile-handl` (subtree-split history of
      `crates/plugin-compile-handl`, package name unchanged);
      `plugin-ext-compile` consumes it via a pinned git dep. The
      roundtrip tests' `playground/*.hnl` fixtures are vendored under
      `tests/fixtures/` (playground is its own repo now).
- [x] Keep `main` a clean build tree (`/project/` ignored).
