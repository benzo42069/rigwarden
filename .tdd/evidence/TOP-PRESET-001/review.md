# TOP-PRESET-001 independent review

Reviewer: `/root/preset001_independent_audit` (`topology_reviewer`, OpenAI `gpt-5.6-luna` / `max`)
Review date: 2026-08-09
Review basis: shared candidate worktree; parent workspace harness is public at `396f1dafcd5e00cc977b5543d2c448478dae415d`.
Decision: `REVIEW_APPROVED` (candidate; parent integration rerun remains required)

## Dependency and scope audit

- `TOP-DOM-003` and `TOP-GRAPH-002` are `INTEGRATED` in the packet/index state. The parent-created `topology-preset` workspace harness is the only root-manifest/lockfile change and is covered by the packet amendment; the worker did not edit those shared files.
- The candidate source/test is bounded to `crates/topology_preset/src/lib.rs`, `crates/topology_preset/src/document.rs`, and `crates/topology_preset/tests/opaque_segment.rs`, with the packet evidence directory. No app, native, device-pack, protocol, fixture, or index paths were changed by the worker.
- This is a synthetic in-memory Rust unit slice. The candidate does not add serialization, vendor/SysEx parsing, protocol transport, semantic interpretation, cross-device conversion, simulator, platform, or hardware behavior.

## Behavior and test audit

- The recorded focused RED is valid: it reaches `topology-preset` and fails only on the intentionally absent `OpaqueSegment`, `PresetDocument`, and `PresetMetadata` exports (exit 101 in `red.log`); it is not a workspace, dependency, selector, syntax, or fixture failure.
- `PresetDocument` stores normalized `PresetMetadata` beside an ordered `Vec<OpaqueSegment>`. `set_name` mutates only the known name field, while `opaque_segments` exposes the retained sequence for observation. No codec or conversion step can be inferred from these types.
- `editing_known_field_preserves_opaque_segment_exactly` invokes `set_name("Edited name")`, asserts that literal edited name independently, checks the literal segment IDs at each position (`known-before`, `unknown-1`, `known-after`), and checks each literal byte vector. The assertions therefore detect name-edit omission, segment loss, reorder, ID change, or byte change; they do not compare a clone or derive expected bytes with production logic.
- The focused GREEN and refactor-green logs show the test passing after the minimum API was added. The handoff correctly limits the candidate to the L1 `UNIT_VERIFIED` claim and explicitly withholds file-codec, byte-fixture, simulator, hardware, and compatibility claims.

## Independent reruns

Working directory: `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`

```text
cargo test -p topology-preset editing_known_field_preserves_opaque_segment_exactly -- --exact --nocapture  # exit 0
cargo test -p topology-routing                                                                            # exit 0
cargo fmt --all -- --check                                                                                # exit 0
cargo clippy -p topology-preset --all-targets -- -D warnings                                              # exit 0
```

The focused test reports one passing integration test. The routing package suite (including the integrated graph tests), formatter check, and preset clippy with warnings denied all complete successfully. No required test is skipped.

## Verification-label audit and integration conditions

After the parent publishes this bounded source/test/evidence patch and reruns the focused test plus every packet sweep from that immutable integration commit, the packet may claim `UNIT_VERIFIED` for preserving ordered opaque segment IDs and bytes while editing the known preset name in this synthetic document model.

`LOSSLESS_FILE_CODEC_VERIFIED`, `BYTE_FIXTURE_VERIFIED`, `SIMULATOR_VERIFIED`, `PLATFORM_VERIFIED`, `HARDWARE_VERIFIED`, vendor-file compatibility, protocol/SysEx compatibility, and cross-device conversion remain unavailable. Parent integration must preserve this review, add post-landing command evidence, and only then promote the packet and index to `INTEGRATED`.
