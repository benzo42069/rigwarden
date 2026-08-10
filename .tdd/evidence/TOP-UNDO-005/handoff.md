status: CANDIDATE_RE_REVIEW_PENDING
work_item: TOP-UNDO-005
behavior_delivered: A confirmed journal snapshot can be written to a caller-selected local path through a sibling temporary file and rename replacement, then loaded into a fresh Journal after the source is dropped. Malformed snapshots and bounded filesystem failures return structured errors; pending mutations/restorations are rejected before any replacement and leave the previous file unchanged.
red: cargo test -p topology-undo confirmed_entry_reloads_from_a_local_snapshot_after_restart_simulation -- --exact --nocapture exited 101 on the missing JournalPersistenceError/save_snapshot_to/load_snapshot_from API. See red.log and red-exit-status.txt.
green: The same focused command exited 0 after the minimum adapter and Journal wrappers were added. See green.log and green-exit-status.txt.
files_changed: crates/topology_undo/src/lib.rs, crates/topology_undo/src/persistence.rs, crates/topology_undo/tests/local_snapshot.rs, and TOP-UNDO-005 evidence only. Candidate patch reference: .tdd/evidence/TOP-UNDO-005/patch.diff. The correction cycle changed only persistence.rs and local_snapshot.rs; no shared files or commit were created.
design_decisions: Keep the reviewed byte codec as the sole serializer; bound local reads at 1 MiB plus one byte; write/flush a unique process/counter sibling temporary file with create_new and rename it over the destination; ignore cleanup errors after a failed write because the primary structured I/O error is preserved; do not fsync or claim power-loss durability.
pitfalls: This local agent's git object view starts at 536d890 and does not contain the packet's declared public dependency object 8b66ae456; the shared working tree contains the integrated dependency files as untracked overlay. Parent must inspect and land the logical patch rather than rely on this agent's local commit ancestry.
claims_earned_after_review_and_integration: UNIT_VERIFIED only.
claims_not_earned: BYTE_FIXTURE_VERIFIED, SIMULATOR_VERIFIED, PLATFORM_DEVICE_VERIFIED, HARDWARE_VERIFIED, fsync/power-loss durability, cross-process locking, SQLite/migration/corruption-repair guarantees.
shared_file_changes_proposed: none.
next_packet: TOP-E2E-000.
correction_cycle:
  amendment: The packet amendment required an independent pre-existing temporary-path symlink regression after review found that predictable `.filename.tmp` plus File::create could follow a symlink and overwrite an external target.
  security_red: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-target cargo test -p topology-undo preexisting_snapshot_temp_symlink_never_overwrites_external_target -- --exact --nocapture" — exit 101. The old implementation overwrote the external target with RWJS bytes. Evidence: security-red-command.txt, security-red.log, security-red-exit-status.txt.
  security_green: The same selector — exit 0 after the minimum fix. A unique process/counter sibling name is opened with OpenOptions::create_new(true); cleanup runs only after this call creates the file, so a pre-existing/raced symlink is never followed or removed. Evidence: security-green-command.txt, security-green.log, security-green-exit-status.txt.
  restart_regression: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-target cargo test -p topology-undo confirmed_entry_reloads_from_a_local_snapshot_after_restart_simulation -- --exact --nocapture" — exit 0 after the security correction. Evidence: correction-restart-green-command.txt, correction-restart-green.log, correction-restart-green-exit-status.txt.
  post_correction_sweeps:
    topology_undo_tests: exit 0 (final package suite after the strengthened test).
    topology_preset_tests: exit 0 (final package suite after the strengthened test).
    workspace_format: exit 0 (final restored source and strengthened test).
    undo_clippy: exit 0 (final restored source and strengthened test).
    evidence: cycle3-sweep-commands.txt, cycle3-sweep.log, cycle3-sweep-exit-statuses.txt.
  review_state: Preserve review.md's prior REVIEW_FAILED decision and H1 finding. This candidate is frozen for a fresh independent security review; no UNIT_VERIFIED claim is earned until that review and parent integration rerun pass.
third_cycle:
  amendment: The 05:05 parent amendment required the Unix regression to pre-create the exact current PID/counter sibling (`.journal.rwjs.<pid>.0.tmp`) in its isolated selector, then prove a deliberate create_new-to-create mutation RED before restoring the safe implementation.
  baseline: exact-name characterization baseline exited 0; `.0.tmp` is the isolated first generated path, with `.1.tmp` additionally covered so the full package suite remains deterministic if the restart test consumes `.0.tmp` first. Evidence: cycle3-baseline-v2-command.txt, cycle3-baseline-v2.log, cycle3-baseline-v2-exit-status.txt.
  mutation_red: the temporary `OpenOptions::create(true)` mutation exited 101 and the external target contained RWJS bytes. Evidence: cycle3-mutation-red-v2-command.txt, cycle3-mutation-red-v2.log, cycle3-mutation-red-v2-exit-status.txt.
  restored_green: restored `OpenOptions::create_new(true)` exited 0 with the external target and both symlinks unchanged. Evidence: cycle3-green-v2-command.txt, cycle3-green-v2.log, cycle3-green-v2-exit-status.txt.
  restart_green: original restart selector exited 0 after restoration. Evidence: cycle3-restart-green-v2-command.txt, cycle3-restart-green-v2.log, cycle3-restart-green-v2-exit-status.txt.
  sweep_status: all required sweeps exited 0 in the final rerun against the restored source; raw output is preserved in cycle3-sweep.log and command/status metadata in cycle3-sweep-commands.txt and cycle3-sweep-exit-statuses.txt.
  final_sweep_commands:
    - "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-cycle3-target cargo test -p topology-undo" — exit 0.
    - "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-cycle3-target cargo test -p topology-preset" — exit 0.
    - "cargo fmt --all -- --check" — exit 0.
    - "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-cycle3-target cargo clippy -p topology-undo --all-targets -- -D warnings" — exit 0.
blockers: Fresh independent security re-review and parent integration rerun remain pending. All focused, package, format, and clippy checks for the final restored candidate are green. No environment or fixture blocker for this cycle.
