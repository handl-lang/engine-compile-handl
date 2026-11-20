# compile-handl

Handl-to-Handl compiler — emits the Hughes arrow dialect of Handl itself.
Part of the `compile-*` compiler-backend family: consumes Handl `Term`
arrow terms and lowers them back to canonical Handl source.

The crate lives in `modules/compile-handl` inside this workspace; runnable
examples are under `examples/`.

## Development

```bash
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
```

`cargo xtask` wraps the repo's `xtask/` crate (`fmt`, `clippy`, `test`, `doc`,
`check`, `gate`); `ci.sh` is the gate entry. Deps resolve from git, so no
sibling checkouts are needed; a plain clone test-runs as-is.

## License

MIT.
