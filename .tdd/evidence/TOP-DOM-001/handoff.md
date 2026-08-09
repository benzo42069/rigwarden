work_item: TOP-DOM-001
status: REVIEW_APPROVED
requirement: DEV-003

behavior delivered:
- Added `DeviceFamilyId`, a validated Rust newtype that preserves nonblank input exactly.
- Added comparable structured `DeviceFamilyIdError::BlankOrWhitespace`.
- Empty and whitespace-only values are rejected without panic.
- Exported both types from `topology_domain`.

strict-TDD evidence:
- RED: `cargo test -p topology-domain blank_device_family_id_is_rejected -- --exact --nocapture` exited 101 solely because the intended APIs were absent; see `red.log`.
- GREEN: the same focused command exited 0; see `green.log`.
- Post-format/documentation focused rerun exited 0; see `post-refactor-focused.log`.
- `cargo test -p topology-domain` passed, including the bootstrap harness.
- `cargo clippy -p topology-domain --all-targets -- -D warnings` passed.
- Packet-scope `cargo fmt --manifest-path crates/topology_domain/Cargo.toml -- --check` passed.

required-sweep note:
- The first workspace-wide format check found and corrected only this packet's `device.rs` formatting.
- A later fail-fast workspace-wide format check stopped on unrelated concurrent edits in `crates/topology_devtools/src/work_item.rs` (blank-line and line-wrapping differences). That path is outside this packet and was not edited. Raw output and statuses are preserved in `sweep.log` and `sweep-exit-statuses.txt` as `BLOCKED_CONCURRENT`.
- Parent integration must rerun the exact required workspace sweeps after the concurrent worker's formatting is complete.

design boundaries:
- No regex, dependency, case folding, trimming of valid identifiers, vendor registry, model type, firmware type, protocol behavior, or hardware behavior was added.
- `trim().is_empty()` is used only for blank/whitespace detection; the original string is retained for valid values.

claims earned by this worker:
- `RED_OBSERVED` and `GREEN_OBSERVED` for the L1 Rust unit behavior.
- Packet-scope unit behavior is demonstrated; independent review is approved and the parent integration rerun is required before `UNIT_VERIFIED` is claimed.

claims unavailable:
- `SIMULATOR_VERIFIED`
- `HARDWARE_VERIFIED`

shared-file changes proposed: none
next executable packet: TOP-DOM-002 (after this packet is reviewed/integrated)
review: `REVIEW_APPROVED` by `/root/dom001_review`; the parent reran the required workspace-wide sweeps after concurrent formatting completed.
integration: reviewed source/evidence were published at `bfaf0ae08568a13975ed234e20236b8117cf3aa4`; focused tests, workspace formatter, workspace clippy, and workspace tests exited 0 before promotion to `INTEGRATED`.
