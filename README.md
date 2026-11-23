# engine-compile-handl

Handl-to-Handl compiler — emits the Hughes arrow dialect of Handl itself.
Part of the `engine-compile-*` compiler-backend family: consumes Handl `Term`
arrow terms and lowers them back to canonical Handl source.

The crate lives in `modules/compile-handl` inside this workspace; runnable
examples are under `examples/`.

## Development

```bash
cargo run --manifest-path xtask/Cargo.toml -- gate   # fmt → clippy → test → doc
```

`xtask/` provides the repo's `xtask/` crate (`fmt`, `clippy`, `test`, `doc`,
`check`, `gate`); `ci.sh` is the gate entry. Deps resolve from git, so no
sibling checkouts are needed; a plain clone test-runs as-is.

## License

MIT.
