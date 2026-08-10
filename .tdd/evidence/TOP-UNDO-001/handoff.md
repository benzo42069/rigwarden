Work item: TOP-UNDO-001
Status: INTEGRATED

Behavior delivered:
- `Journal::begin_parameter_change` stores the caller-provided confirmed prior value and requested new value as pending.
- Pending changes do not appear in `completed_entries`.
- `Journal::confirm` moves exactly one pending entry into the completed undo list.
- `Journal::fail` removes a pending mutation without creating a completed entry.
- Completed entries expose stable target, exact prior value, and exact new value.

TDD evidence:
- Valid focused RED: `cargo test -p topology-undo undo_entry_uses_confirmed_previous_value -- --exact --nocapture`, exit 101, missing journal APIs after packet-authorized harness setup.
- Focused GREEN: same command, exit 0.
- Initial required sweep preserved: topology-undo 0; topology-command-engine 101 because concurrent TOP-SIM-002 `stale_response.rs` imports APIs not yet present; fmt 1 due this packet's two files; clippy 0.
- Formatting correction applied only to this packet; rerun focused test, topology-undo suite, fmt check, and clippy all exit 0.
- Final required rerun after TOP-SIM-002 froze its session API: topology-undo, topology-command-engine, fmt, and clippy all exit 0.

Files and scope:
- Added `crates/topology_undo/**` minimal crate/test.
- Added the packet-authorized topology-undo workspace member and lock edge.
- No filesystem persistence, branch handling, redo, protocol, simulator, app, native, or hardware behavior.
- Parent-owned pre-existing Cargo/Cargo.lock changes remain mixed in the shared worktree; integration must isolate/land the packet edge with the rest of the parent patch.

Claims earned now:
- Candidate Rust unit evidence for pending→confirmed exact prior/new and failed mutation not completed.
- `UNIT_VERIFIED` only after independent review plus integration rerun.

Claims not earned:
- PERSISTENCE_VERIFIED, SIMULATOR_VERIFIED, PLATFORM_DEVICE_VERIFIED, HARDWARE_VERIFIED, BYTE_FIXTURE_VERIFIED, release/distribution, or accessibility claims.

Blocker:
- None after the final adjacent sweep rerun. The initial concurrent command-engine failure is preserved above and was resolved by TOP-SIM-002's own candidate; this packet did not touch those files.

Follow-ups: TOP-UNDO-002 and TOP-E2E-001.

Integration: independent review is `REVIEW_APPROVED`. Parent reran the focused
undo test, undo package, adjacent command-engine package, workspace format
check, and undo Clippy with `-D warnings`; every command exited 0. The bounded
candidate and status updates are published to public main; see
`integration-sweep.md`.
