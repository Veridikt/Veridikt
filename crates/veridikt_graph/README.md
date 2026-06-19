# veridikt_graph

The [Veridikt](https://github.com/Veridikt/Veridikt) intent graph (spec §6, §13):
node table, both adjacency maps, resolution, applicability, the `depends_on`
surface, hygiene checks, strict promotion, reconciliation, and the query engine
behind `veridikt ask`/`show`.

It consumes data produced by `veridikt_annotations` (the declared layer) and
`veridikt_derive` (the derived layer) and reconciles them — labelling every
declared claim **Verified**, **Unverified**, **Contradicted**, or
**Unverifiable**. It never depends on those crates directly (§13 dependency
direction).

Most users want the [`veridikt`](https://crates.io/crates/veridikt) CLI, not this
crate directly.

## License

Licensed under either of [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE) at your option.
