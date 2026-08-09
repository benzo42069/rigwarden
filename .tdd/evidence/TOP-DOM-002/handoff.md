# TOP-DOM-002 handoff

status: `IMPLEMENTED_CANDIDATE` (integration and independent review remain with parent)
work_item: `TOP-DOM-002`
requirements: `DEV-003`, `DEV-005`

## Behavior delivered

- Added `FirmwareId`, an opaque validated Rust newtype.
- Rejected empty and whitespace-only firmware text with comparable `FirmwareIdError::BlankOrWhitespace`.
- Removed only outer whitespace from accepted text; preserved the vendor-defined inner text `1.02 beta 3` exactly.
- Added `AsRef<str>` and `Display` for typed access/diagnostics.
- Added no semantic-version parsing, ordering, compatible ranges, profile resolution, protocol behavior, or hardware behavior.

## Strict-TDD evidence

- RED: `cargo test -p topology-domain firmware_id_preserves_nonblank_vendor_text -- --exact --nocapture` exited `101` because the intended `FirmwareId`/`FirmwareIdError` APIs were absent; see `red.log` and `red-exit-status.txt`.
- GREEN: the same focused command exited `0`; see `green.log` and `green-exit-status.txt`.
- Required fail-fast sweeps all exited `0`: package tests, workspace formatter check, and package clippy with warnings denied; see `sweep.log` and `sweep-exit-statuses.txt`.

## Files changed

- `crates/topology_domain/src/firmware.rs`
- `crates/topology_domain/src/lib.rs` (firmware module/export only; existing TOP-DOM-001 device export retained)
- `crates/topology_domain/tests/firmware_id.rs`
- `.tdd/evidence/TOP-DOM-002/*`

No forbidden/shared file was changed. Root manifests, lockfiles, app/native paths, device packs, index, and traceability remain untouched by this worker.

## Design decisions and pitfalls

- Firmware identifiers are opaque vendor text because the approved research map does not establish a universal grammar; no `semver` dependency or comparison implementation was introduced.
- Outer whitespace handling is explicit and test-covered; internal punctuation, spaces, and leading-zero text are preserved.
- The shared main worktree already contained uncommitted TOP-DOM-001 changes and unrelated packet work. Those changes were preserved and not folded into this behavior's implementation.

## Claims

Earned by this worker:

- `RED_OBSERVED` and `GREEN_OBSERVED` for the L1 Rust unit behavior.
- Candidate `UNIT_VERIFIED` after parent integration rerun and independent review.

Not earned/unavailable:

- `FIRMWARE_RANGE_VERIFIED` (explicit non-goal; no range evidence or behavior)
- `SIMULATOR_VERIFIED`
- `HARDWARE_VERIFIED`

## Handoff

- Shared-file changes proposed: none.
- Next executable follow-ups: `TOP-DOM-003`, then `TOP-REG-001` as packet dependencies permit.
- Blockers: none for this executable L1 cycle; integration owner must apply/review the candidate and rerun the required sweep in the integration worktree.
- Commit/patch reference: shared-worktree candidate diff; no isolated commit was created because the worker had no isolated branch and the integration owner controls landing.
