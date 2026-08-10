# TOP-UNDO-004 independent review

Reviewer: `/root/undo004_review` (`topology_security_reviewer`; OpenAI
`gpt-5.6-luna` / `max`)
Review timestamp: `2026-08-10T03:45:00-05:00`
Review basis: frozen candidate in `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`,
starting commit `536d8901ac91ecdbc15e09356800d9f46be401dd`; no source or test
files were edited by this reviewer.

Decision: `REVIEW_APPROVED` (candidate; parent integration rerun remains
required)

## Findings

- The packet is `READY`, `TOP-UNDO-003` is `INTEGRATED`, and ADR-0006 is
  accepted. The recorded RED is valid: `red.log` exits `101` after the focused
  test reaches `topology-undo` and reports only the intentionally missing
  `Journal::encode_snapshot`/`Journal::decode_snapshot` APIs. It is not a
  selector, fixture, dependency, syntax, filesystem, or unrelated-package
  failure.
- `crates/topology_undo/src/journal.rs:248-292` rejects pending mutations and
  restorations before encoding, stores the explicit `RWJS`/schema header,
  traverses `BTreeMap` branches in stable order, and writes both `f64` values
  with `to_bits()`. The independent test literals in
  `crates/topology_undo/tests/persistence.rs:5-23` assert the magic and
  little-endian schema bytes without importing the production constants; lines
  `30-33` compare exact IEEE-754 bit patterns.
- Decoder bounds and allocation checks are fail-closed. The input cap is
  checked before parsing (`journal.rs:295-301`); `SnapshotReader::take_exact`
  uses checked offset arithmetic (`587-601`); string, branch, per-branch
  entry, and total-entry limits are checked before `String`/`Vec` growth
  (`318-349`, `621-655`); and writer growth is checked before append
  (`507-520`). UTF-8, duplicate branches, missing active branches, and
  trailing bytes return structured errors (`323-357`, `621-640`, `658-665`).
  The parser contains no panic path for malformed input; the focused test's
  `catch_unwind` helper exercises truncation, invalid magic/schema, and an
  oversized count (`persistence.rs:44-69,92-98`).
- The final focused GREEN, full `topology-undo` package, and
  `topology-preset` package all pass. Independent reruns by this reviewer
  exited `0` for the focused selector, both package suites,
  `cargo fmt -p topology-undo -- --check`, workspace `cargo fmt --all -- --check`,
  and `cargo clippy -p topology-undo --all-targets -- -D warnings`. No warning,
  skipped test, or unrelated failure was observed.
- Scope is bounded to `journal.rs`, `lib.rs`, `persistence.rs`, and this
  evidence directory. The candidate adds no filesystem adapter, SQLite layer,
  crash/power-loss recovery, protocol bytes, transport, network, simulator,
  Flutter/FFI, firmware matching, hardware write, secret, telemetry, pack
  trust, or AI capability. The packet's only available product label remains
  `UNIT_VERIFIED` after integration; all higher-layer labels remain unavailable.

## Nonblocking test-coverage follow-up

The source has explicit checks for trailing bytes, duplicate branches, a
missing active branch, invalid UTF-8, oversized strings, per-branch/total
entry counts, and an over-maximum snapshot, but the committed focused test
does not construct each of those vectors (nor an arbitrary-byte/property
corpus). Those are useful hardening regressions, not a blocking defect for
this leaf: the packet explicitly requires malformed/truncated and transient
state rejection, which is exercised with no-panic assertions, and the source
review confirms the additional guards. Add direct negative vectors in a later
codec-hardening cycle before making a broader parser/fuzzing claim.

The evidence also preserves a pre-tightening GREEN and a later final GREEN.
The parent must verify that the post-GREEN tightening introduced only test
assertions/formatting and no new production behavior; if the integration diff
shows a new behavior added after the recorded RED, reopen that behavior with
its own observed RED/GREEN before integration.

## Integration conditions

1. Isolate and land only the packet-authorized source/test/evidence paths; the
   current candidate has no immutable integration commit.
2. From that immutable integration worktree, rerun the focused selector,
   `cargo test -p topology-undo`, `cargo test -p topology-preset`,
   `cargo fmt --all -- --check` (or the packet-scoped formatter plus a clean
   global check), and `cargo clippy -p topology-undo --all-targets -- -D warnings`.
   Preserve the existing RED and intermediate logs; do not rewrite them.
3. Confirm the post-GREEN test-tightening condition above and keep
   `BYTE_FIXTURE_VERIFIED`, `SIMULATOR_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`,
   `HARDWARE_VERIFIED`, filesystem durability, crash recovery, SQLite,
   protocol, transport, firmware, network, secret, pack, telemetry, and AI
   claims unavailable. Only then may the parent promote `UNIT_VERIFIED` or
   `INTEGRATED`.
