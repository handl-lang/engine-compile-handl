# handl examples

Small, self-contained `.hnl` programs. They are two things at once:

- the **runnable reference** for the language surface — every file is a single
  expression you can run with `hnl run`; and
- the **corpus** the `compile-handl` target is exercised against (this repo
  owns `modules/compile-handl`), so each file is written in surface syntax the
  compiler must accept.

Moved here from `platform/examples/handl` (ADR-0128).

## Layout

```
basics/        the surface: operators, conditionals, match, records, arrows
recursion/     feedback with `while` (the kernel's Loop) — 12 programs
algorithms/    classic algorithms as single expressions
lists/         head/tail folds over arrays
modules/       kits, `.handl.toml` and imports (`lib-utils/`)
```

## Run them

```sh
hnl run basics/arithmetic.hnl
hnl fmt recursion/factorial.hnl
hnl eval '1 + 2 * 3'

# everything at once
for f in */*.hnl; do printf '=== %s\n' "$f"; hnl run "$f"; done
```

`hnl run` / `hnl eval` use the canonical kit directly, so no `.handl.toml` is
needed for these files. Output is JSON (bare strings print without quotes).

## What each group shows

### `basics/`

| file | shows |
|---|---|
| `arithmetic.hnl` | precedence, grouping, `**`, `/`, `%`, `<=>`, unary `-` |
| `conditionals.hnl` | `if`/`else` and nested `match` |
| `match.hnl` | `match` over literals, strings, bools, binders, wildcards, nested arms |
| `destructure.hnl` | array (`[h, ...t]`, `[]`) and object (`{ name: n }`) patterns |
| `records.hnl` | nested records/arrays, dot access, computed fields |
| `closures.hnl` | `~>` with multiple params, immediate invocation, higher-order combinators |
| `composition.hnl` | `>>>` — the kernel's `Seq` combinator as surface syntax |

### `recursion/`

Feedback is `while` (ADR-0074): the body closure receives the current state
and returns `{ out: v }` to finish or `{ state: v }` to continue. With **param
destructuring** the body names state fields directly:

```hnl
while { n: 5, acc: 1 } (
  [n, acc] ~> if n == 0 { out: acc } else { state: { n: n - 1, acc: acc * n } }
)
```

`factorial`, `fibonacci`, `tribonacci`, `power`, `fast-exp`, `gcd`,
`sum-to-n`, `collatz`, `fizzbuzz`, `binomial`, `hanoi`, and `ackermann`
(the last uses a self-applying `proc`, since it is not tail-recursive).

### `algorithms/`

`binary-search`, `is-prime`, `largest-factor`, `digital-root`, `popcount`,
`sort_demo`.

### `lists/`

`list-length`, `list-sum`, `list-max`, `list-contains` — folds and searches
over arrays using head/tail in `while` + `if`.

### `modules/`

`lib-utils/` is the kit/configuration example: it ships a `.handl.toml` that
loads the `hello` kit, a `.handl-no-fmt.toml` showing `exclude`, and two
one-line programs. See its README for the `.handl.toml` walkthrough.

## Patterns to steal

**`if` is an expression** — it desugars to `match cond { true: a, _: b }`.

**State spread** — `{ ...s, field: val }` copies `s` and overrides fields
(later entries win).

**Arrow composition** — `5 >>> (proc (x) -> x + 1) >>> (proc (y) -> y * 2)`.

**Self-applying `proc`** for non-tail recursion:

```hnl
(proc (self) -> self(self, 2, 3))(proc (self, m, n) -> match m { 0: n + 1, _: ... })
```

## Known limits of the bare canonical kit

`hnl run`/`hnl eval` use the canonical kit only. Not available there:
`first`/`choice`/`app` as surface keywords (`~>`, `>>>` and `while` are
wired); array spread of a *computed* value; bracket indexing (`xs[0]` — use
`xs.0`); string concatenation with `+`; and the library modules
(`str.*`, `math.*`, `json.*`, `http_server.*`, …), which need a kit loaded via
`.handl.toml` through the plugin host — see `modules/`.

## Docs

Project docs live on the `project` branch, mounted at `project/` (ADR-0129).
