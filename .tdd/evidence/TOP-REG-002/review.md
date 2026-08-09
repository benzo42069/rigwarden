# TOP-REG-002 independent review

Reviewer: `/root/reg002_independent_audit` (`topology_reviewer`, OpenAI gpt-5.6-luna/max)
Review basis: public `TOP-REG-001` integration base `3ae249f57b134a74df21804388f078e13f8a6572` plus the final bounded TOP-REG-002 candidate in the shared worktree.
Decision: `REVIEW_APPROVED` (candidate; parent integration rerun remains required)

## Dependency and scope audit

- `TOP-REG-001` is `INTEGRATED` at the stated public base and supplies the exact-only typed registry resolver. The amended TOP-REG-002 packet is `READY` with its dependency satisfied.
- Candidate production/test edits are limited to `crates/topology_device_registry/src/resolve.rs` and `crates/topology_device_registry/tests/unknown_firmware.rs`, plus this evidence directory. No root manifest, lockfile, app, native, device-pack, index, or traceability path was edited by the packet.
- The candidate is an in-memory typed unit slice. It makes no protocol-byte, JSON/signature, transport, simulator, platform, accessibility, or hardware claim.

## Behavior and implementation findings

- `DeviceRegistry::resolve` remains exact equality over family, model, and opaque firmware. It is still the only path that returns a writable profile; the exact-resolution test remains green.
- `DeviceRegistry::resolve_session` first delegates to exact `resolve`, then returns a result only when family/model are known. It never searches by firmware ordering, range, nearest version, or fallback profile.
- The unknown-firmware result synthesizes a profile from the observed identity, preserving family, model, and firmware. It carries `ResolutionProvenance::UnknownFirmware`, unit `ResolutionStatus::UnknownFirmware`, `SessionCapabilities::new(false)`, and `VerificationStatus::ReadOnly`.
- `ResolvedProfile::unsupported_firmware()` carries the observed `FirmwareId` as the machine-readable reason. The focused test asserts this accessor, the status/provenance, write denial, read-only verification, all three identity fields, and that exact `resolve` does not select the `1.0` profile for observed firmware `1.1`.
- The compatibility refactor restores REG-001's `Copy` and `const fn status()` API while retaining the unknown-firmware reason in the dedicated accessor. No compatibility-range or implicit-write behavior was added.

## TDD and evidence audit

- The original `red.log` is preserved as the pre-production missing-result RED. Because the packet forbids `lib.rs` edits, the test was adapted to existing exports before production implementation. A supplementary final-test-shape RED (`red-final-api.log`, exit 101) was then captured against a temporary REG-001-only resolver and the candidate source was restored; its GREEN is recorded in `green-after-final-api-red.log`.
- The compatibility-preserving refactor has its own observed RED (`red-compat-api.log`, exit 101) naming the old payload/unit mismatch and missing `unsupported_firmware` accessor, followed by GREEN (`green-compat-api.log`, exit 0). The first post-refactor sweep stopped fail-fast at a formatter-only diff (exit 1) and is preserved in `sweep-after-compat-api.log`; the test was formatted, then the complete final sweep passed all four commands in `sweep-final.log` with zero statuses.
- I independently reran the final candidate from the shared worktree and every required command passed:

  ```text
  cargo test -p topology-device-registry unknown_firmware_never_inherits_write_capability -- --exact --nocapture  # exit 0
  cargo test -p topology-device-registry                                                               # exit 0
  cargo test -p topology-domain                                                                        # exit 0
  cargo fmt --all -- --check                                                                            # exit 0
  cargo clippy -p topology-device-registry --all-targets -- -D warnings                                # exit 0
  ```

- No required test is skipped. The synthetic typed fixture needs no provenance sidecar or hardware setup.

## Verification-label audit and integration conditions

After the parent publishes this final bounded candidate and reruns the focused test plus every required sweep from that immutable integration commit, it may claim `UNIT_VERIFIED` and `READ_ONLY` for the exercised known-family/model unknown-firmware behavior. `BYTE_FIXTURE_VERIFIED`, `HARDWARE_VERIFIED`, pack-signature, protocol, transport, simulator, platform, accessibility, and UI claims remain unavailable. Parent integration must preserve this review, add post-landing command evidence, and only then promote the packet and index to `INTEGRATED`.
