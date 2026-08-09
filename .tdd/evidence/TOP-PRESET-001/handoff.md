work_item=TOP-PRESET-001
status=GREEN_OBSERVED; candidate complete and awaiting independent review/integration
requirement=PRESET-007
behavior=Editing the known preset name preserves every opaque segment's ID, order, and bytes exactly in a synthetic in-memory normalized document.

red:
- command: cargo test -p topology-preset editing_known_field_preserves_opaque_segment_exactly -- --exact --nocapture
- exit_status: 101
- evidence: red-command.txt, red.log, red-exit-status.txt
- accepted_reason: the focused test reached topology-preset and failed only for the intentionally missing OpaqueSegment, PresetDocument, and PresetMetadata exports.

green:
- command: cargo test -p topology-preset editing_known_field_preserves_opaque_segment_exactly -- --exact --nocapture
- exit_status: 0
- evidence: green-command.txt, green.log, green-exit-status.txt
- final_refactor_rerun: refactor-green-command.txt, refactor-green.log, refactor-green-exit-status.txt (exit 0 after test assertions were made independently literal)

implementation:
- crates/topology_preset/src/document.rs defines PresetMetadata, OpaqueSegment, and PresetDocument with ordered Vec storage and one set_name edit.
- crates/topology_preset/src/lib.rs re-exports the minimal API.
- crates/topology_preset/tests/opaque_segment.rs uses synthetic metadata and literal segment ID/order/bytes assertions.
- No serialization, protocol/SysEx, semantic interpretation, cross-device conversion, or vendor compatibility behavior was added.

sweeps:
- cargo test -p topology-preset: exit 0.
- cargo test -p topology-routing: exit 0.
- cargo fmt --all -- --check: exit 0.
- cargo clippy -p topology-preset --all-targets -- -D warnings: exit 0.
- The same fail-fast sweep was rerun after the test-only refactor; post-refactor-sweep.log records exact output and all four statuses 0.

claims_earned_by_this_worker:
- Observed RED_OBSERVED and GREEN_OBSERVED cycle evidence at Rust unit layer.
- Candidate behavior is eligible for UNIT_VERIFIED after independent review and integration rerun; this worker does not grant that label.

claims_not_earned:
- LOSSLESS_FILE_CODEC_VERIFIED / FILE_CODEC_VERIFIED.
- BYTE_FIXTURE_VERIFIED.
- SIMULATOR_VERIFIED.
- CAPTURE_VERIFIED.
- HARDWARE_VERIFIED or any device/firmware compatibility claim.

files_changed=See files-changed.txt. Worker touched only document/lib/test and .tdd/evidence/TOP-PRESET-001/**. crates/topology_preset/Cargo.toml is the parent-created harness and was not edited.
evidence=.tdd/evidence/TOP-PRESET-001/
commit_patch_reference=No worker commit; parent integration owns the commit. Candidate is the shared working-tree patch listed in files-changed.txt.
shared_changes_proposed=None. Root Cargo.toml/Cargo.lock membership remains parent-owned and untouched.
next_packets=TOP-PRESET-002, then TOP-UNDO-001 as declared by the packet.
blockers=None for executable unit cycle; independent review and integration rerun are mandatory gates, not behavior blockers.
