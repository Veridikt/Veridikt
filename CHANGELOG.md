# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project follows
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] — Phase 1: the language-agnostic tool

First public release. Phase 1 is feature-complete and has passed the T10
thesis-validation gate (D-039) on both an external OSS repo and an internal
project.

### Added

- **Intent graph** with two layers — a derived layer extracted from source by
  static analysis, and a declared layer from `@veridikt` comment blocks — and
  reconciliation that labels every declared claim **Verified**, **Unverified**,
  **Contradicted**, or **Unverifiable**.
- **Language support** for Python, TypeScript, Rust, Go, and Java via language
  packs (tree-sitter grammars).
- **CLI**: `veridikt init`, `lint`, `ask`, `show`, and friends; `--json` on every
  command; exit codes suitable for CI.
- **MCP server**: four read-only tools so agents can query the intent graph,
  with server-side usage guidance (D-079/D-080).
- **Confidence labels** (`Exact`, `Resolved`, `Heuristic`) on derived edges;
  unresolvable calls are counted, never invented (G-7).
- Parse cache (`.veridikt-cache/`) for fast warm queries on large repos.

### Crates

- `veridikt` — the command-line tool (this is what `cargo install veridikt`
  installs).
- `veridikt_intent`, `veridikt_annotations`, `veridikt_derive`, `veridikt_graph`
  — the library crates behind it.

[Unreleased]: https://github.com/Veridikt/Veridikt/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Veridikt/Veridikt/releases/tag/v0.2.0
