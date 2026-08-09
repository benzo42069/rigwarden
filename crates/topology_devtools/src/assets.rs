//! Production asset-manifest policy checks.

/// A manifest entry with an explicit policy scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetEntry {
    path: String,
    scope: AssetScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AssetScope {
    Production,
    TestOnly,
    Dynamic,
    ProceduralKnob,
}

impl AssetEntry {
    /// Declares a static production asset.
    pub fn production(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            scope: AssetScope::Production,
        }
    }

    /// Declares a test-only static asset fixture.
    pub fn test_only(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            scope: AssetScope::TestOnly,
        }
    }

    /// Declares a functional graphic rendered from live application data.
    pub fn dynamic(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            scope: AssetScope::Dynamic,
        }
    }

    /// Declares a permitted procedural knob renderer rather than bitmap art.
    pub fn procedural_knob(name: impl Into<String>) -> Self {
        Self {
            path: name.into(),
            scope: AssetScope::ProceduralKnob,
        }
    }
}

/// The declared assets for one manifest policy check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetManifest {
    entries: Vec<AssetEntry>,
}

impl AssetManifest {
    /// Creates a manifest from declared entries without scanning the repository.
    pub fn new(entries: impl IntoIterator<Item = AssetEntry>) -> Self {
        Self {
            entries: entries.into_iter().collect(),
        }
    }
}

/// Stable policy diagnostic for a declared production asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetPolicyError {
    pub code: String,
    pub path: String,
    pub message: String,
}

/// Rejects declared production SVG assets, case-insensitively.
///
/// Test-only fixtures and dynamically rendered functional graphics are outside
/// the production static-asset policy.
pub fn validate_manifest(manifest: &AssetManifest) -> Result<(), Vec<AssetPolicyError>> {
    let errors = manifest
        .entries
        .iter()
        .filter(|entry| {
            entry.scope == AssetScope::Production
                && entry.path.to_ascii_lowercase().ends_with(".svg")
        })
        .map(|entry| AssetPolicyError {
            code: "production_svg_forbidden".to_owned(),
            path: entry.path.clone(),
            message: format!("production asset `{}` uses forbidden SVG", entry.path),
        })
        .collect::<Vec<_>>();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
