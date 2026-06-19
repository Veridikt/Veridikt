# veridikt_intent

Shared intent-model contracts for [Veridikt](https://github.com/Veridikt/Veridikt) — the
data types that make up the intent graph (spec §13).

This is the **AST contract** (G-3): every other Veridikt crate depends on these
types, so changing a field here is a breaking change for the whole workspace.
It deliberately has no dependencies and stays tree-sitter-free, so it can sit at
the bottom of the dependency graph.

Most users want the [`veridikt`](https://crates.io/crates/veridikt) CLI, not this
crate directly.

## License

Licensed under either of [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE) at your option.
