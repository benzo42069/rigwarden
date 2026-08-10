# TOP-UNDO-002 independent review

Reviewer: `/root/undo002_review` (`topology_reviewer`, OpenAI `gpt-5.6-luna` / `max`)
Review timestamp: `2026-08-10T06:30:43Z`
Review basis: candidate source and evidence in `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`; no production edits by this reviewer.
Decision: `REVIEW_APPROVED` (candidate only; immutable parent integration rerun remains required)

## Findings

- The packet is `READY`, and both declared dependencies (`TOP-UNDO-001`, `TOP-PRESET-002`) are `INTEGRATED` in `work-items/index.yaml`. The packet copy hash matches the source packet (`476df8ff00ecfab6e17aa04beee76d36d6d9448a9c9b5079f08e9dd1016ae443`).
- The recorded RED is real and intended. `.tdd/evidence/TOP-UNDO-002/red.log` shows the exact focused selector reached `topology-undo` and failed only because `Journal::new_with_preset` was intentionally absent (exit `101`). The test was created before the candidate source (`branch.rs` mtime `01:21:18`, RED `01:21:51`, `journal.rs` mtime `01:22:31` local), and the diagnostic names the requested branch API. There is no selector, syntax, fixture, dependency, or unrelated-container failure.
- `crates/topology_undo/tests/branch.rs:5-33` uses independent, distinct literal context IDs (`preset-a` and `preset-b`) and independent values (`3.0 -> 4.5`, `4.5 -> 6.0`). It asserts the active branch is B, B's completed history contains only the B edit, and A remains inspectable with its original edit. The test would fail if histories were mixed, A were discarded, context switching were ignored, or completed entries exposed the wrong branch. It does not derive expectations through production helpers, weaken thresholds, or skip requirements.
- `crates/topology_undo/src/journal.rs:49-97` adds only in-memory named branch storage/current-branch selection/inspection required by the packet. `journal.rs:104-131` captures `branch_name` at mutation start and appends on confirmation to that origin branch, so a pending A mutation cannot be appended to active B after a context switch. I independently reproduced this sequence (pending A; switch to B; confirm B, then A; each branch remains length one) against the candidate rlib; it exited `0`. This is directly coupled to branch isolation, not persistence, merge, UI, protocol, simulator, or hardware behavior.
- The focused committed test does not itself switch context while a mutation is pending; a mutant that always appended confirmations to the current branch would still pass `branch.rs`. Therefore pending-entry affinity is source-reviewed and independently reproduced here, but it is not a separately earned test claim. A future packet that exposes pending confirmation as a public behavior must add its own RED/GREEN assertion; adding that assertion after this cycle would be tests-after.
- The candidate inventory is bounded to `crates/topology_undo/src/journal.rs`, `crates/topology_undo/tests/branch.rs`, and `.tdd/evidence/TOP-UNDO-002/**`. The shared worktree has all dependency crates untracked because the parent created the harness; that mixed state is recorded in `files-changed.txt` and is an integration-isolation condition, not a worker shared-file violation. No root manifest/lockfile, app, native, device-pack, index, or traceability file is attributed to this candidate.

## Independent sweep audit

I reran the packet selector and every requested sweep from the shared candidate worktree. Each command exited `0`:

```text
cargo test -p topology-undo preset_change_creates_isolated_journal_branch -- --exact --nocapture
cargo test -p topology-undo
cargo test -p topology-preset
cargo fmt --all -- --check
cargo clippy -p topology-undo --all-targets -- -D warnings
```

The undo package reports the branch and dependency confirmation tests; the preset package reports all three tests; no required test is skipped; formatter is clean; Clippy with warnings denied emits no warnings. The recorded fail-fast sweep in `sweep.log`/`sweep-exit-statuses.txt` independently contains the same all-zero statuses.

## Verification-label audit and integration gate

This approval is limited to the Rust unit (L1) packet behavior: named preset branch isolation and retained old-branch inspection. `RED_OBSERVED` and `GREEN_OBSERVED` are supported by the preserved logs. Pending-entry origin affinity is a reviewed implementation safeguard, not a separately verified claim because the committed focused test does not cover a cross-switch pending confirmation. `UNIT_VERIFIED` for the packet becomes claimable only after the parent lands exactly the bounded source/test/evidence paths in an immutable integration commit and reruns the focused command plus every packet sweep from that commit.

The candidate does not earn `PERSISTENCE_VERIFIED`, `SIMULATOR_VERIFIED`, `PLATFORM_SIMULATOR_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`, `HARDWARE_VERIFIED`, byte/file compatibility, accessibility, or release claims. There is currently no immutable candidate/integration commit (`commit: none`); do not mark `INTEGRATED` or promote the unit label from this review alone.

## Reproduction steps

From `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`:

1. Run the focused selector above; it passes on the candidate. The preserved pre-GREEN `red.log` is the expected missing-API failure with exit `101`.
2. Run the four sweep commands above; each must exit `0` with `-D warnings` Clippy output clean.
3. For pending affinity, construct `Journal::new_with_preset("preset-a")`, begin A, switch to `preset-b`, begin/confirm B, then confirm A. Assert `completed_entries().len() == 1` on B and `branch_entries("preset-a").unwrap().len() == 1` afterward. The independent `rustc`/rlib reproduction used for this review exited `0`.
4. Before integration, isolate only `journal.rs`, `branch.rs`, and this evidence directory from the shared untracked harness, create the integration commit, and rerun steps 1-2. Only then promote `UNIT_VERIFIED`/`INTEGRATED`.
