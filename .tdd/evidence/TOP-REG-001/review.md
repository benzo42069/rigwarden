# TOP-REG-001 independent review

Reviewer: `/root/reg001_independent_audit` (`topology_reviewer`, OpenAI gpt-5.6-luna/max)
Review basis: shared candidate worktree; parent-owned empty workspace harness is public, and the candidate registry source/test/evidence is not yet integrated.
Decision: `REVIEW_APPROVED` (candidate; parent integration rerun remains required)

## Dependency and scope audit

- `TOP-DOM-003` is `INTEGRATED` in `work-items/index.yaml`; the amended `TOP-REG-001` packet is `READY` and its workspace-harness amendment is recorded in the packet copy.
- The candidate source and test are limited to `crates/topology_device_registry/src/lib.rs`, `src/resolve.rs`, and `tests/exact_resolution.rs`, with the packet evidence directory. The parent-created `crates/topology_device_registry/Cargo.toml`, root `Cargo.toml`, and `Cargo.lock` harness changes are outside the worker candidate and are not altered by this review.
- The implementation is an in-memory typed unit-layer slice. It claims no JSON pack loading, signatures, protocol bytes, transport, simulator, platform, or hardware behavior.

## Behavior and implementation findings

- `DeviceRegistry::resolve` compares the typed `DeviceFamilyId`, `DeviceModelId`, and opaque `FirmwareId` with equality only. There are no ranges, normalization rules, fallback profiles, or JSON/signature paths.
- The focused test resolves the matching family/model/firmware profile, observes the same profile and its explicit writable `SessionCapabilities`, checks `ResolutionProvenance::ExactProfile` and `ResolutionStatus::ExactMatch`, and verifies that a nonmatching model returns `None`.
- `ResolvedProfile` copies capability and verification metadata from the selected profile; device identity itself is not consulted for write permission beyond the exact profile match. No implicit write grant or hardware operation is introduced.
- The source exposes only the requested exact-match metadata and profile-declared capability. The additional verification-status enum is metadata, not evidence of capture or hardware verification; no such claim is made.
- The candidate test uses independent typed values and a concrete failure-path assertion. It does not claim family/firmware range compatibility or any higher-layer behavior beyond the exercised exact-resolution unit contract.

## Evidence and independent reruns

The recorded RED is valid: `red.log` exits 101 after reaching `topology-device-registry` and reports only the intentionally absent resolver/profile exports. The recorded focused GREEN and required worker sweeps exit 0.

I independently reran all packet commands from `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1` without source edits:

```text
cargo test -p topology-device-registry exact_profile_match_can_enable_write -- --exact --nocapture  # exit 0
cargo test -p topology-device-registry                                                               # exit 0
cargo test -p topology-domain                                                                        # exit 0
cargo fmt --all -- --check                                                                            # exit 0
cargo clippy -p topology-device-registry --all-targets -- -D warnings                                # exit 0
```

The focused test reports one passing exact-resolution test. The registry package and domain package suites, workspace formatter, and clippy with warnings denied all completed without failures or warnings. No required test is skipped.

## Verification-label audit and integration conditions

After the parent publishes the bounded candidate and reruns the focused test plus every required sweep from that immutable integration commit, the packet may claim `UNIT_VERIFIED` for exact typed family/model/opaque-firmware resolution, explicit profile-derived write capability, exact-match provenance/status, and the exercised nonmatching-model rejection.

`PACK_SIGNATURE_VERIFIED`, `HARDWARE_VERIFIED`, protocol/transport compatibility, simulator behavior, platform behavior, accessibility, and UI claims remain unavailable. Parent integration must preserve this review, add post-landing command evidence, and only then promote the packet and index to `INTEGRATED`.
