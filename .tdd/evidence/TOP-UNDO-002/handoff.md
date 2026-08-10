work_item: TOP-UNDO-002
status: INTEGRATED
requirement: UNDO-004

behavior_delivered:
- A Journal can start in a named preset branch and switch to another named preset branch.
- Switching context creates/retains a separate branch; `completed_entries()` exposes only the active branch.
- A prior branch remains inspectable through `branch_entries(name)`.
- Pending mutations retain the branch in which they began, so confirmation cannot append a pre-switch edit to the active branch by accident.
- Existing pending-to-confirmed and failed-mutation behavior remains covered by the dependency test.

tdd_cycle:
- red_command: `cargo test -p topology-undo preset_change_creates_isolated_journal_branch -- --exact --nocapture`
- red_status: 101; valid missing-API RED (`Journal::new_with_preset` absent), raw output in `red.log`.
- green_command: same focused command
- green_status: 0; `branch.rs` passes, raw output in `green.log`.
- required_sweeps: all four commands exited 0; exact commands/statuses/output in `sweep-commands.txt`, `sweep-exit-statuses.txt`, and `sweep.log`.

files_changed:
- `crates/topology_undo/src/journal.rs`
- `crates/topology_undo/tests/branch.rs`
- `.tdd/evidence/TOP-UNDO-002/**`

design_decisions:
- Preset context is the named branch identity for this leaf behavior.
- `Journal::new()` remains source-compatible and starts a `default` branch.
- Branch storage is in-memory only; no serialization, merge, UI, or persistence was added.
- Pending entries carry their originating branch name, preserving isolation if confirmation arrives after a context switch.

pitfalls_and_discoveries:
- Assigned base `5993c881af6d7a1915e624864738b7a3e6b5fa23` is not present in this shared worktree; actual starting commit was `536d8901ac91ecdbc15e09356800d9f46be401dd` on `main`.
- The shared worktree already contained parent/integration edits and untracked dependency crates; no reset or cleanup was performed.
- A first evidence-capture shell used zsh's read-only `status` variable after the test had already run; the raw cargo failure is intact and the status record was written from the observed 101 exit.

fixtures_and_sources:
- No protocol fixture or hardware is required. Inputs are synthetic in-memory journal values (`3.0 -> 4.5`, `4.5 -> 6.0`).
- Requirement/decision sources read: `UNDO-004`, DEC-047, and the TOP-UNDO-002 packet.

claims_earned_by_worker:
- `RED_OBSERVED` and `GREEN_OBSERVED` at Rust unit layer.
- Candidate is eligible for `UNIT_VERIFIED` only after independent review and parent integration-worktree rerun.

claims_not_earned:
- `UNIT_VERIFIED` (integration and independent review still pending).
- `PERSISTENCE_VERIFIED`, `SIMULATOR_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`, `HARDWARE_VERIFIED`, byte/file compatibility, accessibility, and release claims.

shared_changes_proposed: none. Root Cargo manifests/lockfiles, work-item index, traceability, apps, native, device packs, and docs remain integration-owned.
next_packet: Parent may route the declared follow-up `TOP-E2E-001` after this candidate is independently reviewed and integrated.
blockers: none for this executable Rust cycle; independent review and integration rerun are mandatory gates, not behavior blockers.
commit_patch_reference: parent published the bounded source/test/evidence paths after independent review and parent reruns; see integration-sweep.md.
