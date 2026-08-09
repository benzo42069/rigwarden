//! Strict-TDD evidence-directory presence validation.

use std::path::Path;

/// Required files for a completed implementation evidence directory.
///
/// This validator checks presence only. It deliberately does not parse log
/// contents or infer whether a recorded test result is semantically valid.
const REQUIRED_COMPLETION_FILES: &[&str] = &[
    "environment.txt",
    "files-changed.txt",
    "green-command.txt",
    "green-exit-status.txt",
    "green.log",
    "handoff.md",
    "red-command.txt",
    "red-exit-status.txt",
    "red.log",
    "review.md",
    "sweep-commands.txt",
    "sweep-exit-statuses.txt",
    "sweep.log",
    "work-item.yaml",
];

/// A deterministic, machine-readable evidence validation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Stable error category used by callers and CI diagnostics.
    pub code: String,
    /// Required evidence filename associated with the diagnostic.
    pub path: String,
    /// Human-readable context for local debugging.
    pub message: String,
}

impl ValidationError {
    fn missing_file(path: &str) -> Self {
        let code = if path == "red.log" {
            "missing_red_log"
        } else {
            "missing_evidence_file"
        };
        Self {
            code: code.to_owned(),
            path: path.to_owned(),
            message: format!("required evidence file `{path}` is missing"),
        }
    }
}

/// Validate that a completed implementation packet has every required
/// evidence file.
///
/// Missing paths are returned in lexicographic order. A focused RED log is a
/// required file and is reported with the stable `missing_red_log` code. This
/// function intentionally does not parse metadata or log semantics yet.
pub fn validate_completed_evidence(
    directory: impl AsRef<Path>,
) -> Result<(), Vec<ValidationError>> {
    let directory = directory.as_ref();
    let mut missing = REQUIRED_COMPLETION_FILES
        .iter()
        .copied()
        .filter(|name| !directory.join(name).is_file())
        .collect::<Vec<_>>();
    missing.sort_unstable();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing
            .into_iter()
            .map(ValidationError::missing_file)
            .collect())
    }
}
