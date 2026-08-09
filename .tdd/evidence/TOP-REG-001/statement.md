# TOP-REG-001 execution statement

- Requirement IDs: DEV-004, PACK-006.
- Observable behavior: an in-memory registry resolves a profile only when family, model, and opaque firmware identifiers are exactly equal; the resolved session exposes the profile's explicit write capability and exact-match provenance/status.
- Why it matters: device identity alone must never grant writes, and unknown/nonmatching identities must not inherit a nearby profile.
- Test layer: Rust unit/integration test (L1 pure typed behavior).
- Preconditions: TOP-DOM-003 is integrated; the registry crate harness and topology-domain path dependency already exist and pass baseline tests.
- Inputs: one typed profile for `test-family`/`test-model`/`1.0`, with explicit write enabled and `EXPERIMENTAL` verification status; a matching `DeviceIdentity`; a nonmatching-model identity.
- Expected result: the exact identity resolves the same profile, reports writable capabilities, and records exact-profile provenance/status; the nonmatching model does not resolve.
- Non-goals: JSON pack loading, signatures, firmware ranges, unknown-firmware fallback/result handling, protocol bytes, transports, hardware, and UI.
- Allowed writes: `crates/topology_device_registry/Cargo.toml`, `crates/topology_device_registry/src/lib.rs`, `crates/topology_device_registry/src/resolve.rs`, `crates/topology_device_registry/tests/exact_resolution.rs`, `.tdd/evidence/TOP-REG-001/**`.
- Forbidden/shared paths: root `Cargo.toml`, `Cargo.lock`, `apps/**`, `native/**`, `device-packs/**`, and `work-items/index.yaml`.
- Fixture/provenance: synthetic typed in-memory values only; no protocol or external fixture is used.
- Focused command: `cargo test -p topology-device-registry exact_profile_match_can_enable_write -- --exact --nocapture` from the repository root.
- Expected RED: compilation reaches `topology-device_registry` and names the intentionally missing profile/registry/resolver API, without workspace or unrelated dependency failure.
- Required sweeps: `cargo test -p topology-device-registry`; `cargo test -p topology-domain`; `cargo fmt --all -- --check`; `cargo clippy -p topology-device-registry --all-targets -- -D warnings`.
- Claim after GREEN/sweeps/review/integration: `UNIT_VERIFIED` only. `PACK_SIGNATURE_VERIFIED` and `HARDWARE_VERIFIED` remain unavailable.
