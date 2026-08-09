work_item=TOP-PRESET-002
status=GREEN_OBSERVED; candidate complete and awaiting independent review/integration
requirements=PRESET-007, PRESET-011, LIB-010
behavior=Serialize and parse an initial synthetic RigWarden offline container while retaining explicit schema version, device metadata, one normalized graph edge, one known value, and opaque segment ID/order/bytes; reject truncated input with a structured error and no panic.

red:
- command: cargo test -p topology-preset offline_container_preserves_normalized_and_opaque_data -- --exact --nocapture
- exit_status: 101
- evidence: red-command.txt, red.log, red-exit-status.txt
- accepted_reason: the focused test reached topology-preset and failed only for the intentionally missing container API exports.

green:
- command: cargo test -p topology-preset offline_container_preserves_normalized_and_opaque_data -- --exact --nocapture
- exit_status: 0
- evidence: green-command.txt, green.log, green-exit-status.txt
- implementation: `crates/topology_preset/src/container.rs` provides the `RWOC` schema-v1 internal container, bounded length/count checks, structured `ContainerError`, and deterministic little-endian field encoding; `lib.rs` exports it.
- test: `container_roundtrip.rs` asserts literal normalized fields and opaque bytes and exercises truncation/no-panic handling.

refactor:
- The focused test was strengthened after minimum GREEN with independent literal assertions for every declared normalized field and opaque bytes.
- command: cargo test -p topology-preset offline_container_preserves_normalized_and_opaque_data -- --exact --nocapture
- exit_status: 0
- evidence: refactor-green-command.txt, refactor-green.log, refactor-green-exit-status.txt
- Final schema-byte assertion now checks independent literal `[0x01, 0x00]`; focused reruns exit `0` in `refactor-green-2.log` and `refactor-green-3.log`.

sweeps:
- cargo test -p topology-preset: exit 0 (both container tests and integrated opaque-segment test pass).
- cargo fmt --all -- --check: exit 0.
- cargo clippy -p topology-preset --all-targets -- -D warnings: exit 0.
- Evidence: sweep-commands.txt, sweep.log, sweep-exit-statuses.txt.
- The same fail-fast sweep was rerun after the final assertion; `sweep-final-commands.txt`, `sweep-final.log`, and `sweep-final-exit-statuses.txt` record all three exit `0` statuses.
- A final post-literal-assertion rerun is recorded in `sweep-final-2-commands.txt`, `sweep-final-2.log`, and `sweep-final-2-exit-statuses.txt`; all three exit `0`.

claims_earned_by_this_worker:
- Observed `RED_OBSERVED` and `GREEN_OBSERVED` at the Rust file-codec test layer.
- Candidate is eligible for `UNIT_VERIFIED` and `FILE_CODEC_VERIFIED` only after independent review and integration-worktree rerun, per packet; this worker does not grant the labels.

claims_not_earned:
- `BYTE_FIXTURE_VERIFIED` (no independent/provenance-approved bytes; synthetic internal format only).
- `SIMULATOR_VERIFIED`, `CAPTURE_VERIFIED`, `COMMUNITY_CONFIRMED`, `HARDWARE_VERIFIED`, platform verification, vendor-file/SysEx compatibility, or cross-device conversion.
- Long-term/final container-format guarantee; compression/encryption/database storage.

files_changed=See files-changed.txt. Worker touched only the packet source/test paths and `.tdd/evidence/TOP-PRESET-002/**`; parent-created harness/dependency files remain pre-existing.
evidence=.tdd/evidence/TOP-PRESET-002/
commit_patch_reference=No worker commit; parent integration owns the commit/patch and must rerun focused plus required sweeps from its immutable integration worktree.
base_note=Packet requested public base `ef305c8f8f42e66e9419f0de6322c9f85b6265ca`, but the shared worktree was already at parent/integration commit `536d8901ac91ecdbc15e09356800d9f46be401dd`; no reset or destructive operation was performed.
shared_changes_proposed=None. Root Cargo.toml/Cargo.lock, index, and traceability remain integration-owned and untouched.
next_packet=TOP-UNDO-002 (after its declared dependency chain is integrated); no packet amendment proposed.
blockers=Independent review and integration rerun are mandatory gates, not behavior blockers. Physical hardware/vendor fixtures are explicitly out of scope.
