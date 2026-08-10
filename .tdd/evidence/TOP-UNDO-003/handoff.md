work_item: TOP-UNDO-003
status: INTEGRATED
requirement_ids: UNDO-003, UNDO-008

behavior_delivered:
- `Journal::prepare_undo` creates one typed, inspectable proposal from the most recent completed entry in the active preset branch.
- `UndoProposal::target` exposes the semantic target and `UndoProposal::restoration_value` exposes the exact confirmed prior value.
- The completed entry remains in the branch while the proposal is pending.
- `Journal::confirm_undo` consumes only the matching prepared entry after caller confirmation; stale or duplicate proposals return `JournalError::UnknownUndoProposal`.
- Proposals are scoped to their owning `Journal` instance with a private owner token; a proposal from another journal cannot consume a matching-looking entry.
- No transport, protocol bytes, persistence, redo, batch, UI, FFI, simulator, or hardware behavior was added.

tdd_cycle:
- RED command: `cargo test -p topology-undo confirmed_undo_restores_exact_prior_value_only_after_confirmation -- --exact --nocapture`
- RED exit: 101; accepted because the focused test reached `topology-undo` and failed only on the intentionally missing `prepare_undo` and `confirm_undo` APIs. Raw output: `red.log`.
- GREEN command: same selector after the minimum journal implementation.
- GREEN exit: 0; one focused restoration test passed. Raw output: `green.log`.
- Independent review found a cross-Journal identity collision, so parent approved a tightly coupled amendment at `2026-08-10T02:10:37-05:00`; the amended packet copy matches the source packet hash.
- Owner-scope RED: `cargo test -p topology-undo undo_proposal_cannot_confirm_against_another_journal -- --exact --nocapture`, corrected setup exit 101; the first setup's exit 0 is explicitly marked invalid in `red-owner-invalid-reason.txt`.
- Owner-scope GREEN: same selector exit 0 after the private per-Journal owner token check; `green-owner.log` records the raw output.
- Original focused test was rerun after the owner correction (`green-final.log`, exit 0).
- Required sweeps all exited 0 in fail-fast order after the owner correction; exact commands/statuses/raw output are in `sweep-commands.txt`, `sweep-exit-statuses.txt`, and `sweep.log`.

files_changed:
- `crates/topology_undo/src/journal.rs`
- `crates/topology_undo/tests/undo_restoration.rs`
- `.tdd/evidence/TOP-UNDO-003/**`
- `crates/topology_undo/src/lib.rs`, manifests, lockfiles, app/native/device-pack paths, work-item index, and traceability were not edited by this packet.

design_decisions:
- The pending restoration record pairs the proposal's private stable ID/target/value with its source branch and entry index, so confirmation cannot silently consume a different entry.
- The proposal also carries a private `Arc<()>` owner token, and confirmation requires pointer identity with the receiving journal's token; equal IDs/fields from another journal are rejected.
- Only one restoration may be pending at a time; this keeps the packet bounded to one restoration and avoids batch/redo semantics.
- Pending restoration retains the originating branch, consistent with the existing named-branch journal behavior.
- Assertions guard internal branch/index invariants; external stale/duplicate/foreign handles are rejected before those guards. No public constructor or mutable fields permit forging a mismatched proposal.

fixtures_and_sources:
- No protocol fixture, simulator, native platform, or hardware is required. Test inputs are independent literal semantic values (`amp-1/gain`, `3.0`, `4.5`) in an in-memory journal.
- Requirement and contract sources read: `docs/00-READING-ORDER.md`, required project docs, `work-items/README.md`, `work-items/index.yaml`, `TOP-UNDO-003.yaml`, and all strict-TDD references.

claims_earned_by_worker:
- `RED_OBSERVED`
- `GREEN_OBSERVED`
- `UNIT_VERIFIED` for the integrated in-memory Rust restoration behavior only.

claims_unavailable:
- persistent storage or crash recovery
- `SIMULATOR_VERIFIED`
- `PLATFORM_DEVICE_VERIFIED`
- `HARDWARE_VERIFIED`
- byte/file compatibility, protocol, accessibility, FFI, and release claims.

blockers:
- No executable-cycle blocker. Independent review and parent integration rerun passed; the worker did not self-approve or perform the integration.

shared_changes_proposed: none
required_followup: TOP-E2E-000
patch_reference: shared-worktree candidate; no commit created so the parent can isolate and land only the packet-authorized paths.
