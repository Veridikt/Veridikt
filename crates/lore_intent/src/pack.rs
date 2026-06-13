//! Language pack as pure data (spec §8.6, §13, D-070/D-071). A `PackSpec`
//! is everything one generic adapter needs to scan, bind, and derive a
//! language *except* the tree-sitter grammar handle, which `lore_cli` passes
//! as a separate argument so this crate stays tree-sitter-free (D-070d). The
//! query source text rides along in `bind_scm` / `derive_scm`; the adapter
//! compiles it against the grammar at activation.

/// Cumulative capability tiers (§8.6.2). `scan` runs the scanner only; `bind`
/// adds grammar + `bind.scm` (full §7 binding); `derive` adds `derive.scm`,
/// import strategies, and mutator lists (files enter derivation scope).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Scan,
    Bind,
    Derive,
}

impl Tier {
    pub fn name(self) -> &'static str {
        match self {
            Tier::Scan => "scan",
            Tier::Bind => "bind",
            Tier::Derive => "derive",
        }
    }

    /// `derive` ⊇ `bind` ⊇ `scan`: does this tier reach at least `other`?
    pub fn at_least(self, other: Tier) -> bool {
        self.rank() >= other.rank()
    }

    fn rank(self) -> u8 {
        match self {
            Tier::Scan => 0,
            Tier::Bind => 1,
            Tier::Derive => 2,
        }
    }
}

/// One import-resolution strategy (§8.6.1, D-071), parameterized per pack.
/// Tried in `PackSpec.imports` order; the first that resolves wins, and when
/// none resolves the reference drops and is counted (§8.2 rule 3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportStrategy {
    /// `./`/`../` specifiers against the importing file's directory.
    Relative {
        extensions: Vec<String>,
        index_files: Vec<String>,
    },
    /// Dotted or path-shaped module names against `lore.toml [project] roots`.
    RootRelative {
        separator: String,
        extensions: Vec<String>,
        init_files: Vec<String>,
    },
    /// Same-directory sibling files (bare same-package references).
    PackageDir { extensions: Vec<String> },
    /// Strip the module prefix declared in a language manifest found by
    /// walking up from the importing file, then resolve the remainder as a
    /// directory path under the manifest's directory.
    ManifestPrefix {
        manifest_file: String,
        prefix_key: String,
    },
    /// A registered `lore_derive` `ImportStrategy` impl (the escape hatch);
    /// each custom name requires its own D-entry (D-071b).
    Custom { name: String },
}

/// A language pack as validated data (§13). Carries no tree-sitter types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackSpec {
    pub name: String,
    pub format: u32,
    pub tier: Tier,
    pub grammar_id: Option<String>,
    pub extensions: Vec<String>,
    pub comment_token: String,
    pub wrappers: Vec<String>,
    pub sibling_skips: Vec<String>,
    pub mutator_methods: Vec<String>,
    pub mutator_free_functions: Vec<String>,
    pub imports: Vec<ImportStrategy>,
    pub bind_scm: Option<String>,
    pub derive_scm: Option<String>,
}
