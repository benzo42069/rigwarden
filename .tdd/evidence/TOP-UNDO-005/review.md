review_status: REVIEW_APPROVED
review_phase: third_cycle_rereview
implementer: /root/undo005_impl
independent_reviewer: /root/undo005_final_audit (topology_security_reviewer)
reviewed_at: 2026-08-10
commit_reviewed: 536d8901ac91ecdbc15e09356800d9f46be401dd
decision: APPROVE_CANDIDATE_PENDING_INTEGRATION
historical_prior_review: The first and second reviews rejected earlier candidates for the HIGH symlink/arbitrary-file overwrite path and for a stale security GREEN fixture. Those decisions remain historical evidence; the third-cycle candidate and its strengthened test are audited below.

scope_audit:
  packet: TOP-UNDO-005
  reviewed_paths:
  - work-items/wave-02-core-vertical-slice/TOP-UNDO-005.yaml
  - crates/topology_undo/src/lib.rs
  - crates/topology_undo/src/persistence.rs
  - crates/topology_undo/src/journal.rs (dependency contract)
  - crates/topology_undo/tests/local_snapshot.rs
  - crates/topology_undo/tests/persistence.rs (adjacent codec tests)
  - docs/decisions/ADR-0006-local-journal-snapshot-foundation.md
  - .tdd/evidence/TOP-UNDO-005/*
  source_edits_by_reviewer: none
  packet_scope: source and test paths remain within the declared TOP-UNDO-005 write scope; no shared manifests, app, native, pack, index, or traceability files were changed.

independent_sweeps:
  focused_test: "cargo test -p topology-undo confirmed_entry_reloads_from_a_local_snapshot_after_restart_simulation -- --exact --nocapture" # exit 0
  undo_tests: "cargo test -p topology-undo" # exit 0
  preset_tests: "cargo test -p topology-preset" # exit 0
  package_format: "cargo fmt -p topology-undo -- --check" # exit 0
  undo_clippy: "cargo clippy -p topology-undo --all-targets -- -D warnings" # exit 0
  workspace_format: not_run
  workspace_format_reason: The prior reviewer observed a reproducible cargo fmt --all shared-worktree hang. The worker raw sweep records cargo fmt --all as green, but this final audit deliberately did not rerun the workspace-wide command; the package-specific format check above is the available independent result.

behavior_audit:
  sibling_replacement: PASS by code inspection and focused test; the successful path leaves only the destination and uses same-directory rename.
  bounded_reads: PASS for regular files; File::take caps the adapter at 1 MiB plus one byte before codec parsing and the codec has matching bounds.
  structured_errors: PASS for Snapshot and bounded Io variants; no panic path found in the reviewed malformed-input route.
  cleanup: PARTIAL; failed operations attempt removal, but cleanup errors are discarded and the temp name is shared/predictable.
  malformed_file: PASS for invalid magic through the focused test; oversized-file and I/O error variants lack direct tests.
  temporary_path_exclusion: PASS for a clean directory through the focused test; collision/symlink cases are not covered.
  transient_write_preservation: PASS; encode rejects pending mutation/restoration before creating the temp file and the focused test verifies the prior destination bytes remain.
  confirmation_truth: PASS within this local boundary; no pending state is encoded. No hardware, transport, retry, or partial-batch claim is made.

findings:
  - id: UNDO005-SEC-001
    severity: HIGH
    file: crates/topology_undo/src/persistence.rs:82-111
    title: Predictable temporary sibling follows symlinks and permits arbitrary local-file overwrite
    exploit_or_failure_path: A caller-selected destination in a directory writable by another local process yields a deterministic `.filename.tmp`. An attacker can pre-create or race that path as a symlink. File::create follows it and truncates/writes the symlink target; the subsequent rename then moves the symlink over the destination. A stale symlink after a crash has the same effect without an active race.
    impact: Arbitrary local file integrity/confidentiality loss and destination replacement with an attacker-controlled symlink. This is outside the intended journal target and can corrupt unrelated application state (and any state later used to drive hardware), even though the adapter has no direct transport capability.
    missing_tests: Pre-existing temp symlink, TOCTOU symlink insertion, stale symlink after failed save, and destination-symlink replacement tests.
    required_condition: Create a unique sibling with exclusive/no-follow semantics (or an equivalent platform-safe primitive), verify it is a regular file owned by the caller, and ensure cleanup cannot follow attacker-controlled paths before rename.

  - id: UNDO005-SEC-002
    severity: MEDIUM
    file: crates/topology_undo/src/persistence.rs:89-110
    title: Deterministic temp name is a same-process/concurrent-writer collision point
    exploit_or_failure_path: Two concurrent save_snapshot_to calls for the same destination both truncate/write `.filename.tmp`; their writes and renames can interleave, leaving a mixed or stale snapshot, or one call can remove the other's temp after an error.
    impact: Persistent journal corruption or a saved snapshot that does not correspond to the caller's confirmed state, misrepresenting the undo record. TOP-UNDO-005 excludes cross-process locking, but the public `&self` API still permits concurrent saves in one process.
    missing_tests: Concurrent saves with distinct journal contents, write/rename failure while another save is active, and stale-temp recovery.
    required_condition: Use unique temp names plus a single-writer/serialization contract, or explicitly make concurrent calls impossible and test the contract. Do not reuse another caller's temp during cleanup.

  - id: UNDO005-SEC-003
    severity: MEDIUM
    file: crates/topology_undo/src/persistence.rs:88-99
    title: Snapshot permissions inherit ambient File::create defaults
    exploit_or_failure_path: File::create uses platform defaults (typically mode 0644 under a common umask on Unix) for both the temporary file and final snapshot. Any other local account/process able to read the directory can read branch names, parameter identifiers, and values while writing or after replacement.
    impact: Local confidentiality leak of user journal/preset metadata. The packet excludes encryption, but it does not exclude least-privilege file permissions; the threat model treats local state as an asset.
    missing_tests: Assert restrictive permissions/ACLs on the destination and temporary file on supported platforms, and verify no readable temp remains after failures.
    required_condition: Apply OS-appropriate private-file permissions or route storage through a platform-scoped app-private/document-provider location, with an explicit cross-platform test/contract.

  - id: UNDO005-SEC-004
    severity: LOW
    file: crates/topology_undo/src/persistence.rs:121-136
    title: Byte bound does not prevent blocking reads from non-regular caller paths
    exploit_or_failure_path: File::open accepts a FIFO/device/special file. `take(MAX+1).read_to_end` bounds memory but can block indefinitely waiting for EOF, so a caller-controlled path can hang the journal load despite the advertised bounded read.
    impact: Availability/DoS in a local-file workflow; regular-file snapshots remain bounded and parser-safe.
    missing_tests: FIFO or special-file load, regular-file type check, and cancellation/time-budget behavior.
    required_condition: Restrict this adapter to regular files (or document and enforce an OS document-provider contract) and add a bounded cancellation/timeout policy where the platform permits.

verification_label_audit:
  UNIT_VERIFIED: NOT_GRANTED; all focused/package checks are green, but the independent security review failed and parent integration has not rerun the accepted sweep after a fix.
  BYTE_FIXTURE_VERIFIED: unavailable
  SIMULATOR_VERIFIED: unavailable
  PLATFORM_DEVICE_VERIFIED: unavailable
  HARDWARE_VERIFIED: unavailable

integration_conditions:
- Resolve UNDO005-SEC-001 before integration; this is the blocking security defect.
- Decide and document the same-process writer/permission/non-regular-file contracts (UNDO005-SEC-002 through UNDO005-SEC-004), adding the missing regression tests or an explicit reviewed non-goal where appropriate.
- Parent must rerun the focused test, both package suites, package format, and undo clippy in the integration worktree. Workspace-wide fmt remains a shared-worktree caveat and must not be represented as independently rerun by this review.

---

corrected_candidate_rereview:
  reviewer: /root/undo005_final_audit (topology_security_reviewer)
  reviewed_at: 2026-08-10
  source_state: corrected candidate only; no source, test, packet, or index edits by reviewer
  verdict: REVIEW_FAILED
  decision: REJECT_FOR_INTEGRATION
  historical_high_finding: The original predictable-temp symlink overwrite is closed by source inspection in the corrected implementation: the adapter now derives a process/counter sibling name, opens it with OpenOptions::create_new(true), and only attempts cleanup after successful creation. The committed correction evidence still does not exercise that generated name (see UNDO005-SEC-005), so this code-level result is not promoted to a security verification claim.

  independent_commands:
    correction_selector: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-review-target cargo test -p topology-undo preexisting_snapshot_temp_symlink_never_overwrites_external_target -- --exact --nocapture" # exit 0
    restart_selector: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-review-target cargo test -p topology-undo confirmed_entry_reloads_from_a_local_snapshot_after_restart_simulation -- --exact --nocapture" # exit 0
    undo_tests: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-review-target cargo test -p topology-undo" # exit 0; 2 local-snapshot tests included
    preset_tests: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-review-target cargo test -p topology-preset" # exit 0
    package_format: "cargo fmt -p topology-undo -- --check" # exit 0
    undo_clippy: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-review-target cargo clippy -p topology-undo --all-targets -- -D warnings" # exit 0
    workspace_format: "perl -e '$SIG{ALRM}=sub { exit 124 }; alarm 30; exec @ARGV' cargo fmt --all -- --check" # exit 142 (SIGALRM)
    workspace_format_caveat: The workspace-wide command again remained blocked by shared-worktree contention until the 30-second alarm; it is not an independent post-correction pass. The worker's pre-correction raw sweep records exit 0, but this review does not promote that stale result.

  behavior_audit:
    unique_create_new_temp: PASS by source inspection; pre-existing symlinks at the generated path should fail exclusive creation and failed creation does not trigger cleanup. The committed regression does not exercise that generated path.
    successful_sibling_rename: PASS by source inspection and restart GREEN; same-directory rename leaves the destination reloadable and no temporary artifact in the clean test directory.
    cleanup: PARTIAL; the `temporary_created` guard prevents removal of a pre-existing path, but cleanup errors remain intentionally discarded and post-open path replacement is not tested.
    bounded_reads: PASS for regular files; the `take(1 MiB + 1)` limit remains before codec parsing. FIFO/device blocking remains outside tested behavior.
    structured_errors: PASS for Snapshot and CreateTemporary/ReadSnapshot operation variants exercised or inspected.
    transient_write_preservation: PASS; encode rejects pending mutation/restoration before temp creation and restart test preserves the prior destination on rejected saves.

  findings:
    - id: UNDO005-SEC-005
      severity: HIGH
      file: crates/topology_undo/tests/local_snapshot.rs:95-104; crates/topology_undo/src/persistence.rs:154-161
      title: Security GREEN regression targets the removed legacy temp name
      exploit_or_failure_path: The regression creates `.journal.rwjs.tmp`, but corrected production code creates `.journal.rwjs.<process_id>.<counter>.tmp`. The selector therefore never presents a symlink at the path the corrected adapter opens. A vulnerable implementation that uses File::create on the new unique name would pass this GREEN unchanged. The recorded correction RED is valid for the old implementation, but the correction GREEN does not prove the new path's no-follow/exclusive behavior.
      impact: The original runtime fix is plausible and independently probed, but the committed security evidence cannot earn a strict-TDD security claim because the critical regression test is not coupled to the behavior under review.
      missing_tests: Generate the exact candidate temp path deterministically in a test-only seam, or pre-create a controlled set of generated sibling names and assert CreateTemporary leaves every external target/destination unchanged; add a post-open unlink/rename race test where the platform permits.
      required_condition: Correct the regression fixture/test (in a new reviewed cycle) before promoting `UNIT_VERIFIED` or integrating this security-sensitive correction.

    - id: UNDO005-SEC-006
      severity: MEDIUM
      file: crates/topology_undo/src/persistence.rs:95-126
      title: Post-open directory replacement can still move an attacker symlink over the destination
      exploit_or_failure_path: After create_new successfully opens the unique temp, a process able to rename entries in the parent directory can unlink that path and replace it with a symlink before fs::rename. The open descriptor receives the snapshot bytes, but rename then moves the attacker symlink over the destination. This no longer overwrites the symlink target, but can redirect a subsequent load to attacker-selected bytes.
      impact: Local journal integrity/confusion and possible data exposure in a shared writable directory. Cross-process locking is a packet non-goal; an app-private caller-selected directory or descriptor-based replacement contract is still required for a broader threat model.
      required_condition: Keep the local adapter scoped to an app-private/OS-managed regular-file location, or add a platform-safe directory/rename race defense in a follow-up storage packet.

    - id: UNDO005-SEC-007
      severity: MEDIUM
      file: crates/topology_undo/src/persistence.rs:97-100
      title: Snapshot and temp permissions remain ambient
      exploit_or_failure_path: OpenOptions::create_new inherits platform defaults (commonly 0644 under a permissive Unix umask), exposing semantic journal contents to other readers of the selected directory during and after replacement.
      impact: Local confidentiality leak of preset/parameter metadata; encryption is a declared non-goal, but privacy is not established by this packet.
      required_condition: Document/enforce an app-private or OS-scoped destination and add platform permission coverage before exposing arbitrary shared paths.

  claim_audit:
    RED_OBSERVED: original snapshot RED plus valid historical security RED remain recorded.
    GREEN_OBSERVED: restart and package GREENs are independently reproduced; security selector passes but is not accepted as sufficient proof because of UNDO005-SEC-005.
    UNIT_VERIFIED: NOT_GRANTED until the security regression targets the corrected temp path and parent integrates/reruns the required sweep.
    BYTE_FIXTURE_VERIFIED: unavailable
    SIMULATOR_VERIFIED: unavailable
    PLATFORM_DEVICE_VERIFIED: unavailable
    HARDWARE_VERIFIED: unavailable
    fsync_power_loss: unavailable and explicitly out of scope
    cross_process_locking: unavailable and explicitly out of scope

  integration_conditions:
  - Replace the stale legacy-name symlink regression with a test that exercises the current generated sibling path; rerun its RED/GREEN and the restart selector.
  - Preserve unique sibling naming, OpenOptions::create_new(true), and guarded cleanup; do not revert to deterministic File::create.
  - Parent must rerun both package suites, package format, and clippy after the corrected test lands. Workspace-wide fmt remains blocked by the shared-worktree contention caveat and must not be represented as independently green here.
  - Before broader use, document/enforce private regular-file storage or carry the post-open race/permissions work into the storage follow-up; no hardware, simulator, protocol, or durability claims are earned.

---

third_cycle_rereview:
  reviewer: /root/undo005_final_audit (topology_security_reviewer)
  reviewed_at: 2026-08-10
  source_state: third-cycle candidate with restored create_new source and strengthened exact PID/counter regression; no source, test, packet, or index edits by reviewer
  verdict: REVIEW_APPROVED
  decision: APPROVE_CANDIDATE_PENDING_INTEGRATION

  independent_commands:
    security_selector: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-cycle3-review-target cargo test -p topology-undo preexisting_snapshot_temp_symlink_never_overwrites_external_target -- --exact --nocapture" # exit 0
    restart_selector: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-cycle3-review-target cargo test -p topology-undo confirmed_entry_reloads_from_a_local_snapshot_after_restart_simulation -- --exact --nocapture" # exit 0
    undo_tests: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-cycle3-review-target cargo test -p topology-undo" # exit 0; both local snapshot tests included
    preset_tests: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-cycle3-review-target cargo test -p topology-preset" # exit 0
    package_format: "cargo fmt -p topology-undo -- --check" # exit 0
    undo_clippy: "CARGO_TARGET_DIR=/tmp/rigwarden-undo005-cycle3-review-target cargo clippy -p topology-undo --all-targets -- -D warnings" # exit 0
    workspace_format: "perl -e '$SIG{ALRM}=sub { exit 124 }; alarm 30; exec @ARGV' cargo fmt --all -- --check" # exit 142 (SIGALRM)
    workspace_format_caveat: The workspace-wide command again remained blocked by shared-worktree contention until the 30-second alarm. The worker's final cycle3 sweep records cargo fmt --all exit 0 against the restored source; this reviewer preserves that raw evidence but requires the parent integration rerun because an independent post-cycle3 workspace execution was not obtainable.

  security_red_green_audit:
    test_target: PASS; the regression now pre-creates `.journal.rwjs.<pid>.0.tmp` and `.1.tmp`, matching the production PID/counter naming. The isolated selector attacks `.0`; the full package sweep remains deterministic when the restart test consumes `.0` first.
    mutation_red: PASS; cycle3-mutation-red-v2 changed only `OpenOptions::create_new(true)` to `create(true)` and exit 101 showed RWJS bytes in the external target.
    restored_green: PASS; cycle3-green-v2 restored `create_new(true)`, exit 0, and asserted external bytes plus both symlink entries remained unchanged.
    high_finding_UNDO005_SEC_001: CLOSED for the pre-existing/first-open symlink overwrite path. `create_new` rejects an occupied path before a file is opened, and `temporary_created` remains false so cleanup cannot remove the pre-existing symlink.

  behavior_audit:
    sibling_replacement: PASS; successful saves use a same-directory unique temp and rename, leave only the destination in the clean directory, and reload the exact confirmed branch/value.
    bounded_reads: PASS for regular files; `take(1 MiB + 1)` precedes codec parsing. No unbounded allocation was found in this adapter.
    structured_errors: PASS; malformed snapshots and filesystem operations map to typed Snapshot/Io variants without panic in the exercised paths.
    malformed_file: PASS for invalid magic and codec negatives; oversized-file and filesystem-I/O variants remain less directly covered.
    transient_write_preservation: PASS; pending mutation/restoration is rejected before temp creation and the existing destination remains byte-for-byte intact.
    cleanup_preexisting_path: PASS; a failed create_new never sets `temporary_created`, so cleanup cannot remove an unowned/pre-existing symlink.
    cleanup_post_open_race: PARTIAL; after successful create_new, an attacker able to unlink and replace the path before rename or an error can cause cleanup to remove the replacement directory entry. This is not the original target-overwrite path, but no inode-identity/descriptor race defense is present.

  remaining_findings:
    - id: UNDO005-SEC-008
      severity: MEDIUM
      file: crates/topology_undo/src/persistence.rs:95-126
      title: Post-open path replacement can move an attacker symlink or delete a replacement entry
      exploit_or_failure_path: After create_new succeeds, a process with parent-directory write access can unlink the temp path and replace it before fs::rename or a failed-operation cleanup. Rename may move the attacker symlink over the destination; cleanup may remove the replacement directory entry rather than the originally created inode.
      impact: Shared-directory local journal integrity/confusion and possible deletion of another process's replacement file. The corrected code no longer writes through a pre-existing symlink. Cross-process locking is a packet non-goal, so this remains a scoped storage-hardening condition rather than a reopened HIGH finding.
      required_condition: Keep caller-selected storage app-private/OS-managed, or add descriptor/inode-aware replacement and cleanup in a later storage packet. Do not claim shared-hostile-directory safety.

    - id: UNDO005-SEC-009
      severity: MEDIUM
      file: crates/topology_undo/src/persistence.rs:97-100
      title: Snapshot and temp permissions remain ambient
      exploit_or_failure_path: OpenOptions::create_new inherits platform defaults (commonly 0644 under a permissive Unix umask), exposing semantic journal names and values to readers of a shared selected directory.
      impact: Local confidentiality leak of preset/parameter metadata. Encryption is explicitly out of scope, but private storage/least privilege is not established by this packet.
      required_condition: Document/enforce an app-private or OS-scoped destination and add platform permission coverage before exposing arbitrary shared paths.

    - id: UNDO005-SEC-010
      severity: LOW
      file: crates/topology_undo/src/persistence.rs:129-141
      title: Bounded byte reads do not prevent blocking special-file loads
      exploit_or_failure_path: File::open accepts a FIFO/device/special file; read_to_end can wait indefinitely for EOF even though the byte count is capped.
      impact: Local availability/DoS only; regular-file memory bounds and parser checks remain intact.
      required_condition: Restrict the adapter to regular OS-managed files or add a cancellation/time-budget contract in the storage follow-up.

  claim_audit:
    RED_OBSERVED: original snapshot RED, historical security RED, and cycle3 mutation RED are valid and preserved.
    GREEN_OBSERVED: cycle3 exact-path security GREEN, restart GREEN, package suites, package format, and Clippy independently reproduced; worker cycle3 workspace fmt evidence is preserved, while this review's timed attempt was blocked.
    UNIT_VERIFIED: candidate-level approval only; parent must integrate and rerun the accepted packet sweep before publishing `UNIT_VERIFIED`.
    BYTE_FIXTURE_VERIFIED: unavailable
    SIMULATOR_VERIFIED: unavailable
    PLATFORM_DEVICE_VERIFIED: unavailable
    HARDWARE_VERIFIED: unavailable
    fsync_power_loss: unavailable and explicitly out of scope
    cross_process_locking: unavailable and explicitly out of scope

  integration_conditions:
  - Parent lands the final corrected source/test patch and reruns the focused security selector, restart selector, both package suites, package format, workspace format, and undo Clippy in the integration worktree.
  - Preserve unique PID/counter sibling naming, OpenOptions::create_new(true), and guarded cleanup; do not revert to deterministic File::create.
  - Treat UNDO005-SEC-008 through UNDO005-SEC-010 as scoped storage-hardening conditions; enforce app-private/OS-managed regular-file paths before broader shared-path exposure.
  - Only `UNIT_VERIFIED` is available after the integration rerun. Byte-fixture, simulator, platform, hardware, fsync, and cross-process-locking claims remain unavailable.
