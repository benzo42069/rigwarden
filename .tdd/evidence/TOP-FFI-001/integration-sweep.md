# TOP-FFI-001 integration sweep

Integration commit: `4fc226b1a3904eb539d149b413400077708c7e93` on public `main`  
Executed: 2026-08-09 America/Chicago  
Integrator: `/root` (parent orchestrator)

Published `Cargo.lock`, bridge Rust API/generated glue, Dart generated bindings,
focused test, and review blobs were compared with the reviewed local candidate;
all matched. Build artifacts were excluded from the commit.

All commands exited `0`:

```text
CARGO_TARGET_DIR=crates/topology_bridge/target cargo build -p topology-bridge --release
cd apps/mobile_flutter && flutter test test/core/bridge/device_identity_test.dart --plain-name "typed device identity round trips from Rust"
cargo test -p topology-bridge
cd apps/mobile_flutter && flutter test test/core/bridge/device_identity_test.dart
cd apps/mobile_flutter && flutter analyze
cargo fmt --all -- --check
cargo clippy -p topology-bridge --all-targets -- -D warnings
```

Claim established: `FFI_VERIFIED` only for this generated Rust/Dart opaque
identity round trip. No platform build, native transport, physical device, or
hardware compatibility claim is established.
