# Third-party notices

## Current baseline

This repository baseline contains no imported third-party fixture, artwork,
vendor documentation, or generated catalog. The generated Rust/Dart bridge
does vendor build glue and uses the following declared dependencies.

## Flutter Rust Bridge 2.12.0

- **Upstream:** [fzyzcjy/flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge), tag [`v2.12.0`](https://github.com/fzyzcjy/flutter_rust_bridge/tree/v2.12.0) (`62b9330ed2f900535e34d8443ff82dc54070579a`).
- **License:** MIT; see [upstream LICENSE](https://github.com/fzyzcjy/flutter_rust_bridge/blob/v2.12.0/LICENSE).
- **Affected files:** `crates/topology_bridge/**`, generated Dart bridge files under `apps/mobile_flutter/lib/core/bridge/generated/**`, and the Rust/Dart lockfiles.
- **Provenance:** generated with the pinned `flutter_rust_bridge_codegen` 2.12.0 tool. Generated code remains generated glue; it does not establish a product API or platform compatibility claim.

## Cargokit template bundled by Flutter Rust Bridge 2.12.0

- **Upstream/provenance:** the Cargokit template embedded in Flutter Rust Bridge tag `v2.12.0` above; it is generated into `apps/mobile_flutter/rust_builder/cargokit/**` by the pinned tool.
- **License:** the exact bundled MIT and Apache-2.0 text is retained at `apps/mobile_flutter/rust_builder/cargokit/LICENSE` (copyright 2022 Matej Knopp).
- **Affected files:** `apps/mobile_flutter/rust_builder/**` and the iOS Podfile wiring generated for the Flutter app.
- **Modification note:** RigWarden retains the generated template and adds only product-specific crate/package paths. No precompiled binary is imported or distributed.

## Before adding material

Add an entry with the exact upstream name, revision, license/NOTICE text or
link, files affected, provenance, and any modification notice. Do not list a
candidate merely because it was researched; see the source-license audit for
the current reference-only boundaries.
