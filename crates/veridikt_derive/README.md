# veridikt_derive

The **derived layer** of [Veridikt](https://github.com/Veridikt/Veridikt)
(spec §8): static extraction from host source. Given the files in derivation
scope plus the declared state symbols, it produces derived nodes and edges
(Functions/Types, Calls/Affects/Reads) — **every edge carrying an explicit
confidence** (`Exact`, `Resolved`, `Heuristic`).

Unresolvable calls are dropped and *counted*, never guessed (G-7): the derived
layer is true by construction, so reconciliation can trust it. Extraction facts
are cached as JSON (D-064) to keep queries fast on large repos.

Most users want the [`veridikt`](https://crates.io/crates/veridikt) CLI, not this
crate directly.

## License

Licensed under either of [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE) at your option.
