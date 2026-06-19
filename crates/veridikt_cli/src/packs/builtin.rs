//! The builtin packs, embedded at build time (spec §8.6, D-070d). The pack
//! manifest + query files live in the `veridikt_packs` crate (D-086); this maps
//! that embedded data into the CLI's `PackSource`. Fixtures are read from disk
//! by the conformance harness in CI, not needed at runtime (D-070e), so the
//! embedded `fixture_classes` assert the classes the harness verifies.
//!
//! `veridikt_cli` is the only crate that names grammar crates; `veridikt_annotations`
//! and `veridikt_derive` receive the grammar handle from the loader (D-070d).

use super::PackSource;

/// Build the embedded builtin `PackSource`s from the `veridikt_packs` data, in
/// its canonical registration order. The `fixture_classes` here are a trusted
/// assertion verified by the conformance harness (D-070e); the runtime loader
/// does not read fixtures from disk for embedded packs.
pub fn sources() -> Vec<PackSource> {
    veridikt_packs::ALL
        .iter()
        .map(|p| PackSource {
            name: p.name.into(),
            manifest_path: format!("packs/{}/veridikt-lang.toml", p.name).into(),
            manifest: p.manifest.into(),
            bind_scm: p.bind_scm.map(Into::into),
            derive_scm: p.derive_scm.map(Into::into),
            fixture_classes: vec!["scan".into(), "bind".into(), "derive".into()],
        })
        .collect()
}
