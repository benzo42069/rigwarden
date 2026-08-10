# TOP-FFI-002 candidate handoff

status: INTEGRATED
work_item: TOP-FFI-002
requirement_ids: GRAPH-008, GRAPH-010, A11Y-002
candidate_commit: no worker commit; parent integration must land this shared-worktree candidate
starting_commit: 536d8901ac91ecdbc15e09356800d9f46be401dd
evidence: .tdd/evidence/TOP-FFI-002

## Behavior delivered

Rust now builds the synthetic `Input 1 -> Drive 1 -> Output 1` graph through
the existing `topology_routing::Graph` APIs and `topological_traversal`. It
returns a typed, read-only `SerialRouteSnapshot` containing deterministic node
order, exact source/destination node and port identities, and per-node
incoming/outgoing context. The generated Flutter bridge exposes the endpoint
as `readFixtureSerialRouteSnapshot`; no transport bytes, endpoint handle, or
connection mutation capability is present.

Connections are emitted in Rust traversal order (then the graph's stable
endpoint order), so the serial route is independently asserted as Input→Drive
then Drive→Output. Dart only consumes generated final-value records; it does
not recreate graph nodes or traversal.

## Observed cycles

- Rust RED: `cargo test -p topology-bridge rust_authored_serial_route_snapshot_is_stable -- --exact --nocapture`, exit 101. The first attempt exposed a manifest harness issue (cdylib/staticlib only); that invalid output is preserved in `red-invalid-harness-exit-status.txt`. After the allowed harness-only `rlib` addition, the accepted RED names only the absent snapshot API/types (`red.log`, `red-exit-status.txt`).
- Rust GREEN: same focused command, exit 0 after the minimum bridge implementation (`rust-green.log`, `rust-green-exit-status.txt`). The post-format rerun also exits 0 (`rust-green-rerun-*`).
- Dart RED: `flutter test ... --plain-name "typed serial route snapshot round trips from Rust"`, exit 1 before codegen because the generated endpoint was absent (`dart-red.log`, `dart-red-exit-status.txt`). Two invalid evidence-path setup attempts are preserved separately; the accepted rerun uses the absolute evidence path.
- Codegen: `flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml --stop-on-error`, exit 0 (`codegen-*`).
- Release bridge: `CARGO_TARGET_DIR=crates/topology_bridge/target cargo build -p topology-bridge --release`, exit 0 (`release-build-*`).
- Flutter GREEN: packet command, exit 0; the test loaded the real release library through generated FRB (`green-*`).

## Required sweeps

The initial fail-fast sweep reached commands 1–5, failed command 6 only on
bridge source/test formatting, and stopped before Clippy; raw output is
preserved in `sweep.log` with statuses in `sweep-exit-statuses.txt`. After the
package-local mechanical formatter run and focused Rust rerun, a fresh
fail-fast sweep passed every required command (all seven statuses 0) in
`sweep-rerun.log` / `sweep-exit-statuses-rerun.txt`:

1. `cargo test -p topology-bridge`
2. `cargo test -p topology-routing`
3. `CARGO_TARGET_DIR=crates/topology_bridge/target cargo build -p topology-bridge --release`
4. `cd apps/mobile_flutter && flutter test test/core/bridge/serial_route_test.dart`
5. `cd apps/mobile_flutter && flutter analyze`
6. `cargo fmt --all -- --check`
7. `cargo clippy -p topology-bridge --all-targets -- -D warnings`

## Scope and pitfalls

- Candidate paths are enumerated in `files-changed.txt`. No routing source,
  root manifest, native adapter, device pack, index, or Flutter pubspec was
  edited.
- The bridge manifest needed `rlib` in addition to its existing cdylib/staticlib
  outputs so the packet's integration test could link the crate. This is a
  harness/build-target correction, not a protocol behavior change.
- `apps/mobile_flutter/pubspec.lock` was already `AM` in the starting dirty
  worktree. Flutter's required test/analyzer commands refreshed that pre-existing
  lockfile state; it remains outside this packet's candidate scope for the
  parent to reconcile.
- No fixture or hardware input is used; the synthetic graph is Rust-authored.
- Android/JDK and physical iOS/Android/modeler resources are not required for
  this packet and were not claimed.

## Claims

Earned after independent review and parent integration rerun: `FFI_VERIFIED`
for this generated Rust/Dart serial-route snapshot only.

Not earned: `SEMANTICS_VERIFIED`, physical VoiceOver/TalkBack proof,
`PLATFORM_DEVICE_VERIFIED`, `HARDWARE_VERIFIED`, protocol/transport/device
compatibility, native MIDI/USB/BLE behavior, live graph support, or any broader
nonvisual workflow claim.

## Next packet / integration action

Independent review approved this candidate. The parent integration agent reran
the focused Rust/Flutter checks, release build, analyzer, bridge-local format,
and Clippy before landing the bounded files. The worker did not mutate the
index; the integration commit reconciles the packet/index transition and keeps
the pre-existing Flutter lockfile outside this candidate scope.
