Work item: TOP-BOOT-007
Candidate status: GREEN_OBSERVED (independent review and integration pending)

Behavior delivered:
- `topology_devtools::evidence::validate_completed_evidence` checks the
  required evidence-record file set for a completed implementation packet.
- A missing `red.log` returns a stable `missing_red_log` diagnostic at path
  `red.log`, so green-only evidence cannot be accepted.
- All other missing required paths return `missing_evidence_file` diagnostics
  and are returned in deterministic lexicographic order.
- A directory containing every required file is accepted.
- The validator checks file presence only; it does not parse log semantics,
  interpret RED intent, or perform review approval.

TDD evidence:
- Focused RED: `cargo test -p topology-devtools
  completed_evidence_without_red_log_is_rejected -- --exact --nocapture`, exit
  101. The test reached the intended crate and failed only because the
  packet-granted evidence module/API was absent.
- Focused GREEN: same command, exit 0. The test rejects the green-only
  directory, accepts the complete fixture, and asserts sorted missing paths.
- Post-refactor focused rerun: same selector, exit 0.
- Required fail-fast sweeps: `cargo test -p topology-devtools`,
  `cargo fmt --all -- --check`, and `cargo clippy -p topology-devtools
  --all-targets -- -D warnings`, each exit 0 under `set -euo pipefail`.

Files changed:
- `crates/topology_devtools/src/evidence.rs`
- `crates/topology_devtools/src/lib.rs`
- `crates/topology_devtools/tests/evidence_validation.rs`
- `.tdd/evidence/TOP-BOOT-007/*`

Design decisions:
- The required file list mirrors the strict-TDD evidence-record required files
  and is sorted before diagnostics are returned.
- `red.log` receives a dedicated stable diagnostic code while other missing
  files use a generic stable code.
- No Cargo dependency, root manifest, lockfile, protocol behavior, or log
  parser was added.

Claims earned by this candidate:
- Candidate `UNIT_VERIFIED` for deterministic completed-evidence file presence,
  subject to independent review and integration rerun.

Claims unavailable:
- `TDD_SEMANTIC_REVIEW_VERIFIED` until an independent reviewer approves.
- `HARDWARE_VERIFIED`, `RELEASE_VERIFIED`, and protocol/fixture compatibility
  claims.

Shared-file changes proposed: none.
Next executable packet: TOP-BOOT-008 (after independent review/integration).
Blockers: none.
