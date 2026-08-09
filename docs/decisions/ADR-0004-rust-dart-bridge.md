# ADR-0004: Use stable Flutter Rust Bridge v2 with Cargokit for the first typed Rust–Dart boundary

- Status: Accepted for the pre-alpha bridge bootstrap
- Date: 2026-08-09
- Owner: parent orchestrator
- Requirements: `DEV-003`, `PLAT-001`, `PLAT-002`, `QA-001`

## Context

RigWarden keeps deterministic domain validation in Rust and Flutter owns
presentation. `TOP-FFI-001` needs one generated, typed, read-only identity
round trip; a pure-Dart adapter or hand-maintained duplicate model would break
that boundary.

The current checked toolchain is Flutter 3.44.9 / Dart 3.12.2 and Rust 1.97.0.
The selected binding tooling must build from public source, avoid a local
service or WebView, generate reviewable source, and support iOS and Android
without making either platform or hardware claims from a host-only test.

## Decision

Use [Flutter Rust Bridge](https://cjycode.com/flutter_rust_bridge/) **2.12.0**
for the first bridge slice, pinning both the Rust runtime/code generator and
the Dart runtime to that release line. Use its default
[Cargokit integration backend](https://cjycode.com/flutter_rust_bridge/manual/integrate/cargokit)
for the pre-alpha mobile build wiring.

The integration owner will:

1. create the `topology-bridge` Rust crate as a `staticlib` and `cdylib`;
2. add the generated Rust/Dart glue and generation configuration to version
   control;
3. pin the code generator invocation and runtime dependencies;
4. run generated-binding, Rust, Flutter-test, analyzer, and platform-build
   checks as separate evidence commands; and
5. expose only typed, read-only application values. Raw transport handles,
   protocol bytes, and pointers stay outside the presentation API.

`TOP-FFI-001` is not eligible until that parent-owned bootstrap is complete.
Its focused test must call the generated binding; a Dart fake is not an
acceptable substitute.

## Alternatives considered

### Flutter Rust Bridge native assets

The FRB Native Assets backend requires FRB codegen 2.13.0-beta.2 or newer.
That is useful future work, but a beta codegen line is not the correct
foundation for the first public pre-alpha bridge. Reassess it after a stable
FRB release supports the checked Flutter toolchain and after a clean mobile
build comparison.

### Hand-written Dart FFI or method-channel DTOs

Rejected. Either route would introduce hand-maintained glue and make it too
easy to duplicate Rust validation or present a fake as a bridge test.

### UniFFI

Rejected for this slice because its maintained target languages do not give
this repository a documented Flutter/Dart generated binding workflow.

## Consequences and boundaries

- Generated files and lockfile changes are integration-owned and reviewed.
- Cargokit is the current stable FRB default; its upstream lineage and build
  integration remain a supply-chain/reproducibility review point.
- This decision permits bridge setup, not native transport, device discovery,
  physical-device support, or modeler compatibility claims.
- Android command-line tools and CocoaPods are incomplete in the current host
  environment. Their platform-build gaps remain explicit `BLOCKED_ENVIRONMENT`
  evidence if encountered; they do not justify a fake FFI test.

## Revisit gate

Before beta distribution, compare a stable Native Assets backend with the
selected Cargokit path on clean iOS and Android builds, record dependency and
artifact provenance, and amend this ADR only with that evidence.

## References

- [FRB v2 integration overview](https://cjycode.com/flutter_rust_bridge/manual/integrate/overview)
- [FRB Cargokit backend](https://cjycode.com/flutter_rust_bridge/manual/integrate/cargokit/)
- [FRB Native Assets backend](https://cjycode.com/flutter_rust_bridge/manual/integrate/native-assets)
- [FRB 2.12.0 crate metadata](https://docs.rs/flutter_rust_bridge/2.12.0)
