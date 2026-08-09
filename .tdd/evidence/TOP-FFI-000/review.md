# TOP-FFI-000 independent review

Reviewer: `/root/ffi000_review` (`luna_reviewer`; configured OpenAI
`gpt-5.6-luna` / `max`)
Reviewed: 2026-08-09
Decision: `REVIEW_APPROVED`

## Findings and correction

The initial review found that the generated iOS and macOS podspecs referenced
`../LICENSE` while `apps/mobile_flutter/rust_builder/LICENSE` was absent. The
parent ran a focused static RED/GREEN correction cycle, added the package-local
MIT wrapper license, recorded the packet amendment, and updated
`files-changed.txt` and `THIRD_PARTY_NOTICES.md`. The independent path check now
resolves both podspec references to a versioned file. No finding remains.

## Independent verification

- `flutter_rust_bridge_codegen --version` reports `2.12.0`; rerunning
  `flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
  --stop-on-error` exits 0.
- `cargo test -p topology-bridge` exits 0. A separate
  `cargo build -p topology-bridge --release` emits both
  `target/release/libtopology_bridge.a` (static archive) and
  `target/release/libtopology_bridge.dylib` (arm64 Mach-O dylib), corroborating
  the declared `staticlib` and `cdylib` crate types.
- `flutter analyze` exits 0 when run serially; `flutter test
  test/harness_test.dart` exits 0. A concurrent analyzer invocation hit a
  transient Flutter ephemeral-file lock and was not treated as evidence; the
  required serial rerun passed.
- `cargo fmt --all -- --check` exits 0.
- Generated Rust/Dart output carries FRB 2.12.0 markers and is under the
  packet-owned paths. The generated API contains only the required bridge
  initializer; no product identity endpoint, `greet` demo, `lib/src/rust`
  output, or forbidden bridge test/file is present.
- `THIRD_PARTY_NOTICES.md` records the FRB v2.12.0 source commit and bundled
  Cargokit provenance; the bundled Cargokit license text remains present.

## Verification-label audit

After integration, the packet may claim `BUILD_CODEGEN_VERIFIED` only. This
review does not grant `FFI_VERIFIED`, `SEMANTICS_VERIFIED`,
`PLATFORM_SIMULATOR_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`, or
`HARDWARE_VERIFIED`; no Flutter-to-Rust product round trip, native build,
simulator/device, transport, protocol, or modeler evidence was run.

The integration owner must rerun the focused generator command and required
sweeps in the integration worktree before changing the work-item status to
`INTEGRATED`.

## Scope/provenance recheck (2026-08-09)

The public `main` blobs for both `apps/mobile_flutter/ios/Flutter/Debug.xcconfig`
and `Release.xcconfig` contain only `#include "Generated.xcconfig"`. The
candidate adds exactly one optional Pods-Runner include to each file, before
that existing generated include:

- `#include? "Pods/Target Support Files/Pods-Runner/Pods-Runner.debug.xcconfig"`
- `#include? "Pods/Target Support Files/Pods-Runner/Pods-Runner.release.xcconfig"`

These lines match Flutter 3.44.9's `flutter_tools` CocoaPods setup
(`addPodsDependencyToFlutterXcconfig`), which writes the optional include when
the app has a Podfile and no existing Pods include. The `#include?` form keeps
the host harness usable before Pods are installed; it is static configuration,
not proof of an iOS/macOS build. The packet amendment, evidence copy, and
`files-changed.txt` now explicitly include both files. No additional finding
or claim change is required; the approval and `BUILD_CODEGEN_VERIFIED`-only
boundary remain unchanged.
