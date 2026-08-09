# TOP-FFI-000 handoff

Status: `REVIEW_APPROVED_INTEGRATION_PENDING_STATUS_COMMIT`

## Delivered

- ADR-0004-selected Flutter Rust Bridge 2.12.0 bootstrap uses generated Cargokit wiring.
- `topology-bridge` is a workspace `staticlib` + `cdylib` with only generated initialization glue.
- Generated Dart code is versioned under `apps/mobile_flutter/lib/core/bridge/generated`.
- The generated demo `greet` endpoint and demo UI were removed before final generation/sweeps.

## Evidence

- Intended RED: missing generation config, exit 1.
- GREEN: pinned generator, exit 0.
- Final Rust/Flutter/format/demo-absence sweeps: all exit 0.
- `cargo expand` and package-name setup failures are preserved as rejected harness diagnostics; neither is claimed as RED or GREEN.
- Review-correction RED: both generated podspecs referenced a missing
  `rust_builder/LICENSE`; a package-local MIT wrapper license now resolves
  that static CocoaPods metadata defect. This does not prove a CocoaPods,
  iOS, or macOS build.
- Parent-owned `THIRD_PARTY_NOTICES.md` records Flutter Rust Bridge 2.12.0 and
  its bundled Cargokit template; their exact license material remains retained
  with the generated template. The root MIT copyright now uses the public
  RigWarden project identity.

## Boundaries

This is build/codegen evidence only. No domain endpoint, Flutter-to-Rust round trip, Android/iOS build, platform device, transport, protocol, simulator, or hardware claim is earned.

## Follow-up

`TOP-FFI-001` adds exactly one typed read-only DeviceIdentity endpoint and must call the generated binding in its Flutter test.
