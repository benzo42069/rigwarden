# TOP-FFI-001 independent review

Reviewer: `/root/ffi001_review` (`topology_reviewer`, OpenAI gpt-5.6-luna/max)  
Reviewed: 2026-08-09  
Review basis: frozen candidate in the shared worktree plus independent release-build, FFI-test, package, analyzer, formatter, and clippy reruns.  
Decision: `REVIEW_APPROVED` (candidate; immutable integration rerun remains required)

## Findings

- The packet is `READY`; `TOP-DOM-003`, `TOP-BOOT-003`, and `TOP-FFI-000` are `INTEGRATED` in `work-items/index.yaml`. The evidence packet copy is byte-identical to the current packet (matching SHA-256 `2ca659f2d3b5fbddf254f83e334e490fce8f7af46c382833b2fd27306d86cb6a`). The `Cargo.lock` topology-bridge → topology-domain edge is explicitly permitted by the 17:02 packet amendment.
- The canonical RED is real and intended. `.tdd/evidence/TOP-FFI-001/red.log` records exit 1 from the exact focused Flutter command after the test reaches the generated `RustLibApi` and reports only the deliberately absent `readFixtureDeviceIdentity` API. It is not a selector, fixture, native-loader, dependency, or unrelated compiler failure.
- The test is an actual Rust/Dart FFI integration, not a fake. `apps/mobile_flutter/test/core/bridge/device_identity_test.dart:7-9` calls `RustLib.init()` and `readFixtureDeviceIdentity()`; it does not call `initMock`, inject an API implementation, or construct a Dart DTO. Generated `frb_generated.dart:71-75` loads `topology_bridge` from `../../crates/topology_bridge/target/release/`, and the independent release build produced an arm64 Mach-O `libtopology_bridge.dylib`. The dylib exports the FRB dispatcher and opaque-handle retain/release symbols. The focused test therefore crossed the native library and passed with exit 0.
- Rust owns the validated identity. `crates/topology_bridge/src/api/mod.rs:16-17` declares an opaque `DeviceIdentityHandle(DomainDeviceIdentity)` whose tuple field is private; `:20-37` exposes only copied string getters; `:42-48` constructs the existing `topology_domain::DeviceIdentity` from validated typed IDs. There is no Dart constructor, mutation method, raw protocol buffer, transport handle, or presentation pointer. `apps/mobile_flutter/lib/core/bridge/generated/api.dart:9-21` contains only the typed factory and abstract getters; the independent token/constructor scan found no raw/pointer/byte API and no Dart constructor.
- The focused assertion is independent of generated implementation details: it checks literal family, model, firmware, and fixture-endpoint values at `apps/mobile_flutter/test/core/bridge/device_identity_test.dart:11-14`. The fixture is deterministic and in-Rust; it is not a protocol or circular encode/decode fixture. The current API shape structurally prevents Dart-side invalid identity construction; this review does not infer any broader validation, protocol, endpoint, or hardware behavior.
- GREEN is minimal for the packet: one read-only generated factory, one Rust-owned opaque handle, four generated getters, and one focused Flutter test. No native MIDI, device discovery, async stream, protocol bytes, simulator, platform-device, or hardware behavior was added.
- Scope is bounded to the amended paths: `crates/topology_bridge/**`, generated `apps/mobile_flutter/lib/core/bridge/**`, the focused bridge test, and the permitted `Cargo.lock` dependency edge. No native, device-pack, Flutter manifest, index, or traceability file was changed by this candidate. The shared worktree contains unrelated parent candidates; those are not attributed to TOP-FFI-001.

## Independent reproduction

All commands were run without source edits from `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1` (or the app directory shown):

```text
CARGO_TARGET_DIR=crates/topology_bridge/target cargo build -p topology-bridge --release  # exit 0; libtopology_bridge.dylib
cd apps/mobile_flutter && flutter test test/core/bridge/device_identity_test.dart --plain-name "typed device identity round trips from Rust"  # exit 0; 1 test passed
cargo test -p topology-bridge  # exit 0; 0 Rust tests, no failures
cd apps/mobile_flutter && flutter test test/core/bridge/device_identity_test.dart  # exit 0; 1 test passed
cd apps/mobile_flutter && flutter analyze  # exit 0; no issues
cargo fmt --all -- --check  # exit 0
cargo test -p topology-bridge --locked  # exit 0
cargo clippy -p topology-bridge --all-targets -- -D warnings  # exit 0; no warnings
flutter_rust_bridge_codegen --version  # 2.12.0
```

The recorded sweep also includes a successful generator run. Its earlier formatter-only exit 1 is preserved in `sweep.log` and `sweep-exit-statuses.txt`; the required formatter was rerun after the unrelated concurrent correction and exited 0. Flutter's package-update notices are informational; no test/analyzer/compiler warning was hidden.

## Evidence gaps and integration conditions

- Local Git `HEAD` (`536d890`) predates the parent candidate and the worktree is shared/dirty, so this is candidate approval, not immutable integration truth. The parent integration owner must land only the bounded patch, rerun the focused test and every required sweep from the resulting immutable integration commit, and then update packet/index status.
- The host has no Android JDK/Gradle or attached mobile/modeler hardware. Those resources are not required by this packet and no platform or hardware evidence is inferred from the host FFI run.

## Verification-label audit

After the integration rerun, the packet may claim `FFI_VERIFIED` for this generated Rust/Dart identity round trip only. `PLATFORM_DEVICE_VERIFIED`, `HARDWARE_VERIFIED`, Android/iOS platform-build claims, native MIDI/USB/BLE compatibility, protocol compatibility, and physical device support remain unavailable.
