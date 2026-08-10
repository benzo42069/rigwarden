work_item: TOP-UNDO-004
requirement_ids: UNDO-002
status: GREEN_OBSERVED

behavior_delivered:
- `Journal::encode_snapshot` emits a versioned, deterministic bounded snapshot of all confirmed named-branch history and the active branch.
- `Journal::decode_snapshot` validates literal magic/schema, bounded counts/lengths, UTF-8, checked byte access, duplicate branches, active-branch membership, and trailing bytes before constructing a fresh journal.
- Confirmed `f64` values are stored and restored with exact IEEE-754 bit representations.
- Pending mutations and pending restorations are rejected with distinct `JournalSnapshotError` variants; transient state is never serialized as confirmed history.
- No filesystem, SQLite, network, protocol, simulator, Flutter, FFI, or hardware behavior was added.

tdd_cycle:
- RED command: `cargo test -p topology-undo confirmed_entry_survives_deterministic_save_reload -- --exact --nocapture`
- RED exit: `101`; accepted because the focused test reached `topology-undo` and failed only on intentionally missing `Journal::encode_snapshot`/`Journal::decode_snapshot` APIs. Raw output: `red.log` (identical rerun preserved as `red-rerun.log`).
- GREEN command: same selector; final tightened test output is `green-final.log`, exit `0` in `green-final-exit-status.txt`.
- A transient implementation compile typo was captured unchanged as `green-compile-failure.log`; it was corrected before the accepted GREEN rerun.
- Required final sweeps all exited `0`; command order and raw output are `sweep-commands.txt`, `sweep.log`, and `sweep-exit-statuses.txt`.

files_changed:
- `crates/topology_undo/src/journal.rs`
- `crates/topology_undo/src/lib.rs`
- `crates/topology_undo/tests/persistence.rs`
- `.tdd/evidence/TOP-UNDO-004/**`
- No forbidden/shared paths were edited by this packet.

design_decisions:
- Snapshot format is internal and versioned: literal `RWJS` magic, little-endian schema/count/length fields, active branch, BTreeMap-sorted branch names, and target/value records.
- Bounds are enforced on total input/output bytes, string lengths, branch counts, and per/total entry counts before allocation or append.
- Fresh decoded journals reset pending state and allocation IDs; only confirmed semantic history is persisted.
- The test uses independent literal magic/schema bytes and exact `f64::to_bits()` comparisons rather than deriving expected values from codec code.

claims_earned_by_worker:
- `RED_OBSERVED`
- `GREEN_OBSERVED`

claims_available_after independent review and integration:
- `UNIT_VERIFIED` for this bounded Rust journal snapshot behavior only.

claims_unavailable:
- `BYTE_FIXTURE_VERIFIED` (internal journal bytes are not a vendor/protocol fixture)
- `SIMULATOR_VERIFIED`
- `PLATFORM_DEVICE_VERIFIED`
- `HARDWARE_VERIFIED`
- filesystem durability, crash recovery, SQLite, migration, compaction, encryption, corruption repair, redo, accessibility, FFI, and release claims

blockers:
- No executable-cycle blocker.
- Independent security review and parent integration rerun remain pending; implementer does not self-approve and no worker commit was created.

shared_changes_proposed: none
required_followup: TOP-UNDO-005 after parent integration
patch_reference: shared-worktree candidate; parent must isolate/land only the packet-authorized paths and rerun the focused selector plus all required sweeps.
