# TOP-FFI-002 independent review

decision: REVIEW_APPROVED
reviewer: /root/ffi002_final_audit (topology_reviewer)
candidate_commit: none (shared worktree candidate)
starting_commit: 536d8901ac91ecdbc15e09356800d9f46be401dd
review_date: 2026-08-10

## Decision and findings

No blocking correctness, TDD, scope, or verification-label finding was found.
The candidate is approved for parent integration, subject to the integration
conditions below. The work-item index remains `READY` until the parent lands
the candidate and reruns the checks in the integration worktree.

Dependencies are integrated in the current index: TOP-GRAPH-002,
TOP-GRAPH-005, and TOP-FFI-001. The packet paths and behavior still match the
repository.

## RED/GREEN audit

- The first Rust attempt is correctly rejected as harness-only invalid:
  `red-invalid-harness-exit-status.txt` records the bridge crate exposing only
  `cdylib/staticlib`, so the integration test could not link. The allowed
  `rlib` addition in `crates/topology_bridge/Cargo.toml` fixes that harness
  issue without changing the release outputs.
- The accepted Rust RED is real and intended:
  `red-command.txt` / `red.log` show the focused selector reaching
  `topology-bridge` and failing only on the absent
  `read_fixture_serial_route_snapshot` and `SerialRouteConnection` API. No
  unrelated compiler or fixture failure is present.
- The Dart RED is also real and intended: `dart-red.log` shows the focused
  test reaching the generated imports and failing on the absent
  `readFixtureSerialRouteSnapshot` endpoint before codegen. The invalid
  relative-evidence-path attempt is explicitly rejected in
  `dart-red-invalid-harness-exit-status.txt` and is not used as proof.
- Rust GREEN and the post-format focused rerun both pass. The implementation
  in `crates/topology_bridge/src/api/mod.rs:54-163` constructs the graph only
  through `topology_routing::Graph`, `connect`, and
  `topological_traversal`; it returns typed route records and derives all
  connection context from the graph. The Rust test at
  `crates/topology_bridge/tests/serial_route.rs:3-48` uses independent literal
  IDs/ports, checks deterministic repeatability, node order, both connection
  identities, and incoming/outgoing context.
- Codegen evidence (`codegen-command.txt`, status 0) is pinned to
  `flutter_rust_bridge_codegen 2.12.0`. The generated headers in
  `apps/mobile_flutter/lib/core/bridge/generated/{api.dart,frb_generated.dart,frb_generated.io.dart,frb_generated.web.dart}` all identify FRB 2.12.0.
- The Flutter GREEN test at
  `apps/mobile_flutter/test/core/bridge/serial_route_test.dart:6-35` calls
  `await RustLib.init()` and then the generated endpoint. It does not call
  `RustLib.initMock`, construct a DTO, build a Dart graph, or compute a Dart
  traversal. The generated `const` DTO constructors in `api.dart` are
  final-value artifacts from FRB only; no test/API fixture constructs them.
  The release dylib exists at
  `crates/topology_bridge/target/release/libtopology_bridge.dylib` and the
  focused test passes against the default generated loader path, so this is
  not a mock-only GREEN.

## Independent command evidence

All commands below were run independently from the current shared worktree;
each exited 0.

```text
cargo test -p topology-bridge rust_authored_serial_route_snapshot_is_stable -- --exact --nocapture
cargo test -p topology-bridge
cargo test -p topology-routing
CARGO_TARGET_DIR=crates/topology_bridge/target cargo build -p topology-bridge --release
(cd apps/mobile_flutter && flutter test test/core/bridge/serial_route_test.dart --plain-name "typed serial route snapshot round trips from Rust")
(cd apps/mobile_flutter && flutter test test/core/bridge/serial_route_test.dart)
(cd apps/mobile_flutter && flutter analyze)
cargo fmt -p topology-bridge -- --check
cargo clippy -p topology-bridge --all-targets -- -D warnings
```

The required package tests cover the bridge snapshot and all existing routing
unit tests (cycle, missing endpoint, identity, direction, valid connection,
and deterministic traversal). Analyzer reports no issues. The package-local
formatter and bridge Clippy check are clean.

The packet's workspace-wide `cargo fmt --all -- --check` was **not** rerun by
this reviewer because this environment has a documented reproducible hang for
that command. The worker's raw final result is preserved, not rewritten, in
`sweep-rerun.log` command 6 and `sweep-exit-statuses-rerun.txt` (status 0).
The worker's initial fail-fast sweep also preserves the real formatter failure
(status 1 before Clippy) in `sweep.log` / `sweep-exit-statuses.txt`; it was
fixed mechanically with package-local formatting before the recorded final
sweep. This is an evidence-layer caveat, not a substituted success claim.

## Boundary/scope audit

Static inspection found no route-facing raw protocol bytes, transport handle,
endpoint-open operation, or graph connection mutation. The generated FRB
files necessarily contain codec/FFI pointer internals (`raw`, `dart:ffi`, and
serialization helpers), but those are generated runtime implementation and
are not exposed as route DTO fields or endpoint arguments. The route API is
limited to strings and read-only snapshot records. No Dart test-side graph or
mock API is present.

Candidate behavior files are bounded to the packet paths:

- `crates/topology_bridge/Cargo.toml` (the harness-only `rlib` plus bridge
  dependency);
- `crates/topology_bridge/src/api/mod.rs`;
- `crates/topology_bridge/src/frb_generated.rs`;
- `crates/topology_bridge/tests/serial_route.rs`;
- the four generated Dart bridge files under
  `apps/mobile_flutter/lib/core/bridge/generated/`; and
- `apps/mobile_flutter/test/core/bridge/serial_route_test.dart`.

No root `Cargo.toml`, `apps/mobile_flutter/pubspec.yaml`, routing source,
native adapter, device pack, or `work-items/index.yaml` change is attributed
to this packet.

`Cargo.lock` is packet-granted, but this is a dirty shared worktree and the
lockfile diff also contains pre-existing workspace expansion. The candidate
description identifies the intended bridge-to-routing edge; the parent must
reconcile the lockfile in integration rather than blindly treating the full
base-to-current diff as this worker's change. `apps/mobile_flutter/pubspec.lock`
was already `AM` at packet start and was refreshed by Flutter pub commands; it
is outside the packet candidate and must likewise be reconciled by the parent.

## Verification labels and gaps

The evidence supports `FFI_VERIFIED` only for this host-generated
Rust/Dart serial-route snapshot after the parent integration rerun (L5
host/native FFI evidence). It does not support `SEMANTICS_VERIFIED`,
`PLATFORM_DEVICE_VERIFIED`, `VOICEOVER_VERIFIED`, `TALKBACK_VERIFIED`,
`HARDWARE_VERIFIED`, protocol/transport compatibility, live-device graph
support, or any broader accessibility workflow. No physical platform or
modeler was used, and no such claim is made.

Independent workspace-format evidence is intentionally unavailable because
the command was not run here; the worker's raw final status 0 is the only
workspace-format record. There is no worker commit or clean-clone integration
proof yet; those are parent integration responsibilities.

## Integration conditions

1. Land the exact bounded source/generated/test files and the reconciled
   bridge lockfile edge; preserve the pre-existing Flutter lockfile caveat.
2. In the integration worktree rerun the focused Rust selector, bridge and
   routing package tests, release bridge build, exact/full Flutter serial-route
   tests, analyzer, package-local `cargo fmt -p topology-bridge -- --check`,
   and bridge Clippy with `-D warnings`. Do not claim an independent
   workspace-format pass from this review.
3. Confirm the generated files remain FRB 2.12.0 output and the Flutter test
   still uses `RustLib.init()` with the actual release bridge library, not a
   mock or hand-built DTO/graph.
4. Only then mark TOP-FFI-002 `INTEGRATED` and carry forward `FFI_VERIFIED`;
   keep all higher-layer labels unavailable.

## Reproduction

From `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`, run the commands
listed in **Independent command evidence**. The accepted pre-GREEN REDs are
reproduced by checking out the candidate's test before its endpoint/types and
generated bindings are present: the Rust selector reports unresolved imports
(`red.log`), and the Dart selector reports the missing generated endpoint
(`dart-red.log`). Do not use the earlier invalid harness/path failures as RED
evidence.
