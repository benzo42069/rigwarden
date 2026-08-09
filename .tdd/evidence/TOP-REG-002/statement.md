# TOP-REG-002 strict-TDD work-item statement

- Work item: `TOP-REG-002`
- Requirement IDs: `DEV-005`, `PACK-006`, `SEC-001`
- Observable behavior: resolving a known family/model with an unprofiled firmware returns an explicit machine-readable read-only session result.
- Why it matters: an adjacent or future firmware must never inherit writable mappings from a nearby profile.
- Test layer: Rust unit (`rust_unit`, topology test ladder L1).
- Preconditions: `TOP-REG-001` is `INTEGRATED` at the recorded integration base `a24244cd849658f9758adbf86c7b505ec8b9bf65`; the registry package and exact-resolution baseline run successfully.
- Input: one writable profile for `test-family` / `test-model` / firmware `1.0`; identity for the same family/model with firmware `1.1` on a synthetic endpoint.
- Expected result: known device plus unknown firmware is identified; `can_write()` is false; the reason identifies the unsupported firmware as machine-readable; no nearest/lower profile is selected.
- Non-goals: offline best-effort decoding, compatible firmware ranges, profile download UI, JSON/signature handling, protocol bytes, transport, or hardware behavior.
- Allowed write paths: `crates/topology_device_registry/src/resolve.rs`, `crates/topology_device_registry/tests/unknown_firmware.rs`, `.tdd/evidence/TOP-REG-002/**`.
- Forbidden/shared paths: `Cargo.toml`, `Cargo.lock`, `apps/**`, `native/**`, `device-packs/**`, `work-items/index.yaml`, and `docs/requirements/traceability.yaml`.
- Fixture/provenance: synthetic typed in-memory identifiers; no external or protocol fixture.
- Focused command: `cargo test -p topology-device-registry unknown_firmware_never_inherits_write_capability -- --exact --nocapture`.
- Expected RED: the focused test reaches the registry package and fails because the additive session-result API/reason is absent; unrelated exact-resolution behavior remains compilable and green.
- Required sweeps: registry package tests, domain package tests, workspace format check, and registry clippy with warnings denied.
- Claims available after candidate GREEN plus independent review/integration rerun: `UNIT_VERIFIED`, `READ_ONLY`.
- Claims unavailable: `BYTE_FIXTURE_VERIFIED`, `HARDWARE_VERIFIED`.
