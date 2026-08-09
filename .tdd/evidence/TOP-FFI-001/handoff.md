# TOP-FFI-001 candidate handoff

status: INTEGRATED
work_item: TOP-FFI-001
requirement_ids: DEV-003, PLAT-001, PLAT-002

## Behavior delivered

The generated Flutter Rust Bridge now exposes one read-only
`readFixtureDeviceIdentity` factory. Rust constructs the existing
`topology_domain::DeviceIdentity` from validated typed IDs, stores it in a
Rust-owned opaque `DeviceIdentityHandle`, and exposes only copied string
getters (`family`, `model`, `firmware`, `transportEndpoint`). Generated Dart
`api.dart` provides the typed endpoint and handle interface; the test calls
that generated API and asserts all four fields exactly.

The handle has no Dart constructor and no mutation method. Domain validation
remains in Rust. No raw protocol bytes, transport handles, or pointers appear
in the presentation-facing generated API (FRB pointer glue remains generated
runtime internals only).

## Observed cycle

- RED: `cd apps/mobile_flutter && flutter test test/core/bridge/device_identity_test.dart --plain-name "typed device identity round trips from Rust"` — exit 1. The test reached `RustLibApi` and failed on the intentionally absent endpoint. See `red.log` and `red-exit-status.txt`.
- GREEN: same focused command — exit 0 after codegen and the release bridge dylib build. The generated endpoint crossed the Rust/Dart boundary and returned AM4 / AM4 / 1.00 / fixture://am4. See `green.log` and `green-exit-status.txt`.
- Codegen: `flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml --stop-on-error` — exit 0.
- Native test artifact: `CARGO_TARGET_DIR=crates/topology_bridge/target cargo build -p topology-bridge --release` — exit 0. The target override places `libtopology_bridge.dylib` at the generated `ioDirectory`; `target/` is ignored output.

## Required sweeps

- `cargo test -p topology-bridge` — 0.
- `cd apps/mobile_flutter && flutter test test/core/bridge/device_identity_test.dart` — 0.
- `cd apps/mobile_flutter && flutter analyze` — 0, no issues.
- `cargo fmt -p topology-bridge -- --check` — 0.
- Initial `cargo fmt --all -- --check` — 1 because an unrelated pre-existing diff remained in `crates/topology_command_engine/tests/read_only.rs`; the exact failure is preserved in `sweep.log` and labeled `BLOCKED_CONCURRENT`.
- Rerun after the parent's concurrent CMD002 formatting correction — 0; see `sweep-rerun.log` and command 8 in `sweep-exit-statuses.txt`. The worker did not edit that shared file.

The broader `cargo test --workspace` sweep also passed in the shared worktree,
but it is not substituted for the packet's required commands.
Additional checks (locked bridge test, bridge clippy with `-D warnings`, Dart
format, and generated presentation-API token scan) are summarized in
`additional-checks.txt`.

## Dependencies and environment

The work-item index and evidence identify TOP-DOM-003, TOP-BOOT-003, and
TOP-FFI-000 as `INTEGRATED`; ADR-0004 selects FRB 2.12.0/Cargokit. The local
Git object database is intentionally stale: HEAD is `536d890` and cannot
resolve public-main `dc77496`; the parent verified that public commit through
the GitHub API. The candidate was executed against the parent's integrated
files in the shared worktree and does not claim a local public-main commit.

The host is macOS arm64 with Rust 1.97.0, Flutter 3.44.9/Dart 3.12.2,
FRB codegen 2.12.0, and Xcode 26.5. Android adb is present, but Java/JDK and
Gradle are unavailable; this packet does not require Android or iOS device
verification. No protocol fixture or hardware input is used; the identity is
an in-Rust deterministic test value.

## Candidate files

See `files-changed.txt` for the exact candidate set and the explanation of
bootstrap files that are untracked only because of the stale local Git base.
The parent may land the generated files and lockfile edge; no shared index,
manifest, native, or device-pack files were changed.

## Claims

Candidate claim available after independent review and integration rerun:
`FFI_VERIFIED` for this generated Rust/Dart identity round trip only.

Not earned: `PLATFORM_DEVICE_VERIFIED`, `HARDWARE_VERIFIED`, Android/iOS
platform builds, native MIDI/USB/BLE compatibility, protocol compatibility,
or any broader device support claim.

## Handoff actions

Independent review approved the candidate after confirming the test calls
`RustLib.init()` and the generated native bridge rather than a fake. Parent
published the bounded patch in public commit
`4fc226b1a3904eb539d149b413400077708c7e93`, matched the reviewed blobs, and
reran the release build, focused/full Flutter tests, analyzer, Rust package
test, formatter, and Clippy checks successfully. Packet and index status are
now `INTEGRATED`; see `integration-sweep.md`.
