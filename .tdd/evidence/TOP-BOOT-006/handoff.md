Work item: TOP-BOOT-006
Candidate status: GREEN_OBSERVED (independent review and integration pending)

Behavior delivered:
- Exported `topology_devtools::fixture` and added `validate_yaml` for the
  redistribution declaration in a fixture provenance YAML record.
- Records with `redistribution.permitted: false` are rejected with stable code
  `redistribution_permission_required` and path `redistribution.permitted`.
- Records that omit a non-empty `redistribution.basis` are rejected with the
  same stable code and path `redistribution.basis`.
- A complete permitted declaration (`permitted: true` plus a non-empty basis)
  is accepted.
- The source category is not used to infer permission; the focused test uses a
  `simulator_fixture` source category while proving denied permission remains
  denied.

Required TDD evidence:
- RED: `cargo test -p topology-devtools fixture_without_redistribution_permission_is_rejected -- --exact --nocapture`, exit 101. The test reached the intended package and failed only because the fixture module/API was absent.
- GREEN: the same focused command, exit 0. Denied, missing-basis, and complete-permission cases are exercised in one focused test.
- Required sweeps: final `cargo test -p topology-devtools`, `cargo fmt --all -- --check`, and `cargo clippy -p topology-devtools --all-targets -- -D warnings` each exit 0. The first sweep attempt's formatting failure is preserved in `sweep.log`; formatting was corrected and the full fail-fast sweep rerun passed.

Files changed:
- `crates/topology_devtools/src/lib.rs` — module export granted by the parent amendment.
- `crates/topology_devtools/src/fixture.rs` — minimum redistribution validator and stable diagnostics.
- `crates/topology_devtools/tests/fixture_validation.rs` — focused denied/missing-basis/permitted test.
- `.tdd/evidence/TOP-BOOT-006/*` — packet copy and cycle evidence.

Design decisions and scope:
- No Cargo dependency, root manifest, or lockfile change.
- The validator intentionally does not inspect fixture bytes, interpret legal
  language, verify signatures, or grant permission based on source category.
- The parser handles the block/inline `redistribution` mapping needed by this
  packet and returns structured code/path diagnostics.
- Parent's mechanical scope amendment (recorded at 2026-08-09T11:15:00-05:00,
  `claim_changed: false`) is copied into `work-item.yaml`.

Pitfalls and prior blocker:
- Before amendment, `src/lib.rs` export was absent from the packet write scope;
  execution stopped without production behavior and the blocker was reported.
- The amendment granted only the required module export, after which the valid
  RED/GREEN cycle ran.

Claims earned by this candidate:
- Candidate `UNIT_VERIFIED` for the redistribution permission rule, subject to
  independent review and integration rerun.

Claims unavailable:
- `BYTE_FIXTURE_VERIFIED` (no protocol-byte compatibility test).
- `CAPTURE_VERIFIED` (no real capture or provenance corpus consumed).
- `HARDWARE_VERIFIED`, `RELEASE_VERIFIED`, and CLI integration claims.

Shared-file changes proposed: none.

Next packet: TOP-BOOT-007 (after this packet is independently reviewed and
integrated; TOP-BOOT-009 remains dependent on both validators).

Blockers: none after the recorded parent amendment. Independent review and
integration are required before the packet can be marked `INTEGRATED`.
