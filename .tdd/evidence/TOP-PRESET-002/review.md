# TOP-PRESET-002 independent review

Reviewer: `/root/preset002_independent_audit` (`topology_reviewer`, OpenAI `gpt-5.6-luna` / `max`)
Review date: 2026-08-09
Review basis: requested public base `ef305c8f8f42e66e9419f0de6322c9f85b6265ca`; final candidate source and evidence in the shared worktree at starting commit `536d8901ac91ecdbc15e09356800d9f46be401dd`.
Decision: `REVIEW_APPROVED` (candidate; parent integration rerun remains required)

## Dependency and scope audit

- The public packet at `ef305c8` is `READY`, and its declared `TOP-PRESET-001` dependency is `INTEGRATED` at the public base.
- Candidate production/test edits are bounded to `crates/topology_preset/src/container.rs`, `crates/topology_preset/src/lib.rs`, and `crates/topology_preset/tests/container_roundtrip.rs`, plus this evidence directory. The parent-created `Cargo.toml`, lockfile, `document.rs`, and prior opaque-segment test are correctly recorded as pre-existing harness/dependency files; no root manifest, app, native, device-pack, index, or traceability path is attributed to this worker.
- The implementation is explicitly a synthetic RigWarden-owned offline container (`RWOC`, schema `1`). It does not implement a vendor file, SysEx, protocol transport, database storage, compression/encryption, simulator, platform bridge, or hardware behavior.

## TDD and behavior findings

- The recorded RED is valid: `red.log` shows the focused `topology-preset` test reached the intended package and failed only on the deliberately absent container exports (exit `101`). It is not a selector, syntax, dependency, fixture, or unrelated workspace failure.
- The minimum GREEN adds a deterministic versioned container with little-endian bounded length/count fields, explicit `ContainerError` variants, normalized device/graph/value fields, and ordered opaque ID/byte fields. The source comments and handoff preserve the synthetic/no-vendor compatibility boundary.
- The final test-strengthening cycle is present in `refactor-green-2.log`, `refactor-green-3.log`, and `sweep-final-2.*`. `container_roundtrip.rs` uses independent literal expectations for `RWOC`, schema bytes `[0x01, 0x00]`, schema value `1`, all device metadata, the graph edge, the known value, the opaque segment ID, and the opaque bytes. It does not derive expected bytes or values by calling the encoder. The declared fixture contains one edge, one known value, and one opaque segment, as required by the packet; the existing `TOP-PRESET-001` test separately covers ordered multi-segment preservation.
- The truncation test removes a byte from a valid encoded document, wraps parsing in `catch_unwind`, and requires both no panic and a `Result::Err`. The parser audit confirms that invalid magic, unsupported schema, invalid UTF-8, oversized lengths/counts, truncated fields, and trailing bytes return structured `ContainerError` values. Checked offset arithmetic and bounded allocations prevent the exercised parser from panicking on malformed/truncated input.

## Independent reproduction

All commands were rerun without source edits from `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1` after the final literal-schema test change:

```text
cargo test -p topology-preset offline_container_preserves_normalized_and_opaque_data -- --exact --nocapture  # exit 0
cargo test -p topology-preset offline_container_rejects_truncated_input_without_panic -- --exact --nocapture # exit 0
cargo test -p topology-preset                                                                                 # exit 0
cargo fmt --all -- --check                                                                                     # exit 0
cargo clippy -p topology-preset --all-targets -- -D warnings                                                 # exit 0
```

The package suite reports both container tests, the integrated opaque-segment test, and doc-tests passing. No required test is skipped and Clippy emits no warnings.

## Verification-label audit and integration conditions

After the parent publishes this bounded candidate and reruns the focused tests plus every packet sweep from that immutable integration commit, it may claim `UNIT_VERIFIED` and `FILE_CODEC_VERIFIED` for the synthetic RWOC round-trip and truncation behavior. `BYTE_FIXTURE_VERIFIED`, `SIMULATOR_VERIFIED`, `PLATFORM_VERIFIED`, `HARDWARE_VERIFIED`, vendor-file/SysEx compatibility, cross-device conversion, and long-term format guarantees remain unavailable.

This review does not promote the packet to `INTEGRATED`; preserve this record, publish only the authorized source/test/evidence paths, rerun the required integration sweep, and update packet/index status only after that rerun passes.
