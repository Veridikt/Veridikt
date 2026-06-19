//! Builtin language packs for Veridikt: the canonical manifests and tree-sitter
//! query files (`veridikt-lang.toml`, `queries/bind.scm`, `queries/derive.scm`)
//! for Python, TypeScript, Rust, Java, and Go.
//!
//! The pack *files* live under this crate (`packs/<lang>/`) so they ship in the
//! published crate and the `veridikt` CLI can embed them without any crate
//! reaching outside its own directory for them (D-086). The CLI maps this raw
//! data into its own `PackSource`; the `veridikt_annotations`/`veridikt_derive`
//! boundary tests pull the query strings from here too, so every consumer
//! points at one canonical copy.

use std::path::{Path, PathBuf};

/// The embedded sources of one builtin pack: its manifest text and query files
/// as `&'static str`. `bind_scm`/`derive_scm` would be `None` for a tier that
/// ships no such query; all current builtin packs supply both.
pub struct PackData {
    pub name: &'static str,
    pub manifest: &'static str,
    pub bind_scm: Option<&'static str>,
    pub derive_scm: Option<&'static str>,
}

macro_rules! pack {
    ($name:literal) => {
        PackData {
            name: $name,
            manifest: include_str!(concat!("../packs/", $name, "/veridikt-lang.toml")),
            bind_scm: Some(include_str!(concat!(
                "../packs/",
                $name,
                "/queries/bind.scm"
            ))),
            derive_scm: Some(include_str!(concat!(
                "../packs/",
                $name,
                "/queries/derive.scm"
            ))),
        }
    };
}

pub static PYTHON: PackData = pack!("python");
pub static TYPESCRIPT: PackData = pack!("typescript");
pub static RUST: PackData = pack!("rust");
pub static JAVA: PackData = pack!("java");
pub static GO: PackData = pack!("go");

/// All builtin packs, in the canonical order the CLI registers them.
pub static ALL: &[&PackData] = &[&PYTHON, &TYPESCRIPT, &RUST, &JAVA, &GO];

/// The embedded pack with this name, if it is a builtin.
pub fn get(name: &str) -> Option<&'static PackData> {
    ALL.iter().copied().find(|p| p.name == name)
}

/// Absolute path to the on-disk `packs/` directory, baked in at this crate's
/// compile time. Meaningful only inside the Veridikt repository, where the
/// conformance harness reads whole pack directories (including the `fixtures/`
/// suites) from disk. Published consumers must use the embedded data above —
/// fixtures are not needed at runtime (D-070e) and are excluded from the
/// published crate.
pub fn packs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("packs")
}
