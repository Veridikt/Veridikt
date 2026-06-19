# veridikt_packs

The builtin language packs for [Veridikt](https://github.com/Veridikt/Veridikt):
the canonical `veridikt-lang.toml` manifests and tree-sitter query files
(`queries/bind.scm`, `queries/derive.scm`) for **Python, TypeScript, Rust, Java,
and Go**.

The pack files physically live in this crate (`packs/<lang>/`) so they ship in
the published crate and the `veridikt` CLI can embed them — no other crate has
to reach outside its own directory for pack data. The crate exposes each pack's
manifest and query text as `&'static str`, plus `packs_dir()` for the in-repo
conformance harness.

Most users want the [`veridikt`](https://crates.io/crates/veridikt) CLI, not this
crate directly.

## License

Licensed under either of [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE) at your option.
