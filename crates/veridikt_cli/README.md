# veridikt

**Your comments claim what the code does. Veridikt checks whether they're still
telling the truth — and fails CI when they aren't.**

This crate is the `veridikt` command-line tool. For the full project, see the
[Veridikt repository](https://github.com/Veridikt/Veridikt).

## Install

```sh
cargo install veridikt
```

This installs the `veridikt` binary.

## What it does

Veridikt maintains **one intent graph with two layers** and checks them against
each other:

- **Derived layer — the *what*.** Functions, calls, and state reads/writes
  extracted from source by static analysis. True by construction, available with
  **zero annotations**.
- **Declared layer — the *why*.** `@veridikt` blocks in ordinary comments
  carrying what no analysis can recover: `purpose`, `because`, `assumes`,
  `unknown`, `owner`, and effect claims like `affects` and `reads`.
- **Reconciliation — the honesty mechanism.** Every declared claim is labelled
  against the derived layer: **Verified**, **Unverified**, **Contradicted**, or
  **Unverifiable**. Drift becomes a CI finding, not silent decay.

Veridikt never presents a guess as a fact: derived edges carry an explicit
confidence (`Exact`, `Resolved`, `Heuristic`), unresolvable calls are *counted,
never invented*, and `Unverifiable` is a first-class, honest answer. Annotation
and derivation work for **Python, TypeScript, Rust, Go, and Java**.

## Example

```python
# @veridikt
# purpose: "Apply a refund and record it"
# affects: Payment.ledger
def refund(payment_id: str, amount: Decimal) -> None:
    ledger.append(LedgerEntry(payment_id, -amount))
```

The day someone stops writing to `Payment.ledger`, the `affects` claim flips to
**Contradicted** and CI fails — the comment can't lie silently.

## Common commands

```sh
veridikt init      # propose a veridikt.toml manifest for your repo
veridikt lint      # reconcile declared vs derived; exit non-zero on drift (use in CI)
veridikt ask ...   # query the intent graph ("what writes to Payment.ledger?")
veridikt show ...  # explain a node: purpose, owner, effects, confidence
```

## License

Licensed under either of [MIT](../../LICENSE-MIT) or
[Apache-2.0](../../LICENSE-APACHE) at your option.
