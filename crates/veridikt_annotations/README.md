# veridikt_annotations

`@veridikt` annotation scanning, binding, and module scoping for
[Veridikt](https://github.com/Veridikt/Veridikt) (spec §7) — the **declared
layer**.

It scans ordinary source comments for `@veridikt` blocks, binds each block to the
declaration it annotates (via a tree-sitter `Binder` supplied by a language
pack), and resolves module scoping. The grammar crates themselves are dev-only
dependencies: packs supply concrete grammars to the CLI at runtime (D-070d).

Most users want the [`veridikt`](https://crates.io/crates/veridikt) CLI, not this
crate directly.

## License

Licensed under either of [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE) at your option.
