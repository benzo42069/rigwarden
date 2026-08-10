# TOP-UNDO-003 independent review

Reviewer: `/root/undo003_review` (`topology_reviewer`, OpenAI `gpt-5.6-luna` / `max`)
Review timestamp: `2026-08-10T07:16:04Z`
Candidate basis: amended shared-worktree candidate at starting commit
`536d8901ac91ecdbc15e09356800d9f46be401dd`; no source/test edits by this reviewer.

Decision: `REVIEW_APPROVED` for the amended candidate only. Parent must still
isolate the authorized paths, land an immutable integration commit, and rerun
the focused selector plus every packet sweep before changing the packet/index
status or promoting `UNIT_VERIFIED`.

## Findings and RED/GREEN audit

- The packet is `READY`; `TOP-UNDO-001` and `TOP-UNDO-002` are `INTEGRATED` in
  `work-items/index.yaml`. The source packet and evidence copy are identical
  (`ec1cf8d4b23b1283d2fbf02d66296fd1db44ff08046c91395c03efc0441be268`). The
  parent-recorded amendment keeps the correction within exact-entry undo
  behavior and does not add persistence, transport, or platform scope.
- The original RED is real and intended. `red-command.txt` and `red.log` show
  the focused selector reached `topology-undo` and failed only on the
  deliberately absent `Journal::prepare_undo` and `Journal::confirm_undo`
  methods (exit `101`). The test existed before the candidate implementation;
  no selector, fixture, dependency, syntax, or unrelated-suite failure is
  involved.
- The owner-scope cycle also has a valid, focused RED. The corrected selector
  in `red-owner-corrected-command.txt` reaches the test and fails at
  `first.confirm_undo(foreign_proposal).is_err()` in
  `red-owner-corrected.log` (exit `101`): two independently initialized
  journals produced colliding IDs/fields and the receiving journal accepted
  the foreign proposal. The earlier owner setup exited `0` because it had no
  pending restoration in the receiving journal; `red-owner-invalid-reason.txt`
  explicitly excludes that attempt, while preserving its raw log.
- `crates/topology_undo/tests/undo_restoration.rs:3-29` asserts the requested
  target, exact prior value (`3.0`), retained completed entry while pending,
  and consumption only after confirmation. Lines `32-55` independently use
  two equal-looking journals and assert a foreign proposal is rejected while
  both entries remain. Literal semantic inputs are independent of the
  implementation algorithm; no simulator, transport, persistence, fixture,
  skipped case, or weakened threshold is present.
- `crates/topology_undo/src/journal.rs:192-240` is bounded to the packet. The
  proposal copies the target/prior value, retains the branch/index until
  confirmation, and now carries a private `Arc<()>` owner token. Confirmation
  checks `Arc::ptr_eq` and the proposal fields before touching the branch;
  foreign, stale, or duplicate proposals return `UnknownUndoProposal` and the
  pending proposal is restored. The original focused test was rerun after this
  correction (`green-final.log`, exit `0`), and the owner test is green in
  `green-owner.log` (exit `0`).
- The prior frozen candidate was correctly rejected for the cross-Journal
  identity collision. That finding is preserved here; the owner-token
  amendment closes the reproduced failure rather than weakening the test.

## Assertion and panic audit

- The `expect` at `journal.rs:227` is safe for the public API: constructors and
  `switch_preset` always insert branches, and no public operation removes a
  branch. The `expect` at `:229` is also guarded by the state machine: while a
  proposal is pending, public mutation can only append entries, so the stored
  index remains valid; the only removal is the same `confirm_undo` call after
  these checks.
- The `assert_eq!` guards at `:231-238` therefore represent private-state
  invariants, not user-controlled validation. I independently exercised a
  pending proposal across a branch switch plus an append, and exercised a
  foreign-proposal rejection followed by confirmation of the owning proposal;
  both smoke sequences exited `0` without a panic. No safe public path can
  remove or rewrite the asserted entry before confirmation. Non-finite `f64`
  policy is not introduced or claimed by this packet.

## Scope and required sweeps

The changed candidate paths are exactly:

- `crates/topology_undo/src/journal.rs`
- `crates/topology_undo/tests/undo_restoration.rs`
- `.tdd/evidence/TOP-UNDO-003/**`

`files-changed.txt` records no edits to `lib.rs`, manifests/lockfiles, apps,
native, device packs, index, or traceability. The shared worktree is dirty and
the candidate has no immutable commit; this is an integration gate, not a
scope violation.

Independent reruns from the repository root all exited `0`:

```text
cargo test -p topology-undo confirmed_undo_restores_exact_prior_value_only_after_confirmation -- --exact --nocapture
cargo test -p topology-undo undo_proposal_cannot_confirm_against_another_journal -- --exact --nocapture
cargo test -p topology-undo
cargo test -p topology-preset
cargo fmt --all -- --check
cargo clippy -p topology-undo --all-targets -- -D warnings
```

The amended evidence also preserves all owner-scope and final sweep statuses
(`sweep-exit-statuses.txt`, including the final four-command sequence); no
warning or required test is hidden or skipped. The uncounted invalid owner
attempt is explicitly labeled and does not replace the corrected RED.

## Verification-label audit

After the parent lands exactly the bounded source/test/evidence paths and reruns
the commands above from the immutable integration commit, the only earned
product label is `UNIT_VERIFIED` for this in-memory Rust restoration behavior.
`BYTE_FIXTURE_VERIFIED`, `SIMULATOR_VERIFIED`, `PLATFORM_SIMULATOR_VERIFIED`,
`PLATFORM_DEVICE_VERIFIED`, `HARDWARE_VERIFIED`,
accessibility, and release/distribution claims remain unavailable. This review
does not mark the packet `INTEGRATED`.

## Reproduction steps

1. From `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`, run the two
   focused selectors and the four required sweeps listed above; each must exit
   `0` on the amended candidate.
2. To reproduce the corrected RED against the pre-owner implementation,
   initialize two journals identically, confirm one `amp-1/gain` edit in each,
   call `prepare_undo()` on both, and pass the second proposal to the first.
   The old implementation accepts it and consumes the first entry; the
   amended implementation returns `UnknownUndoProposal` and retains both
   entries.
3. For the assertion invariant, prepare an undo, switch branches, append a
   mutation to the originating branch, then confirm the original proposal.
   The proposal removes its original indexed entry without panic. Finally,
   reject a foreign proposal and confirm the owning proposal to verify that a
   failed identity check does not clear pending state.
4. Before integration, isolate only the three candidate paths above, create the
   integration commit, and rerun steps 1-3. Only then promote `UNIT_VERIFIED`
   and `INTEGRATED`.
