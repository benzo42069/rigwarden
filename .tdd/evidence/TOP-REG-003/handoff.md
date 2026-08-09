# TOP-REG-003 handoff

## Status

Integrated after independent review and parent post-landing sweep. The
historical `work-item.yaml` remains the exact READY-state execution packet;
the source packet and work-item index record the later integration status.

## Behavior delivered

`DeviceProfile` retains an additive, profile-owned numeric catalog. A typed
`NumericParameterMetadata` stores inclusive integer bounds and decimal precision
without floating-point or NaN semantics. Exact `(block_id, parameter_id)` lookup
returns metadata, missing identifiers return no definition, and the synthetic
unknown-firmware profile created by `resolve_session` starts with an empty
catalog.

The existing `DeviceProfile::new` constructor signature remains unchanged. The
catalog is populated through `add_numeric_parameter` or the chaining
`with_numeric_parameter` builder method.

## Files changed

- `crates/topology_device_registry/src/lib.rs`
- `crates/topology_device_registry/src/resolve.rs`
- `crates/topology_device_registry/tests/numeric_parameter_metadata.rs`
- `.tdd/evidence/TOP-REG-003/**`

No root manifests, lockfiles, Flutter/native paths, device packs, work-item
index, or command-engine files were changed.

## TDD evidence

- Focused RED: `cargo test -p topology-device-registry exact_numeric_parameter_metadata_is_profile_owned -- --exact --nocapture` failed with unresolved `NumericParameterMetadata`, `with_numeric_parameter`, and `numeric_parameter` APIs; see `red.log` and `red-exit-status.txt`.
- Focused GREEN: the same literal command passed one focused test; see `green.log` and `green-exit-status.txt`.
- Required sweeps all passed: registry tests, domain tests, `cargo fmt --all -- --check`, and registry clippy with `-D warnings`; see `sweep.log` and `sweep-exit-statuses.txt`.
- Harness mistakes were preserved separately as `red-invalid-harness.*`, `sweep-invalid-harness.*`, and `sweep-preformat.*`; none are presented as TDD evidence.

## Claims

Available after review/integration: `UNIT_VERIFIED` only.

Unavailable: `PACK_SIGNATURE_VERIFIED`, `BYTE_FIXTURE_VERIFIED`,
`HARDWARE_VERIFIED`.

## Design and risk notes

- IDs remain opaque strings and are matched exactly; no range inference,
  protocol mapping, transport coupling, or firmware fallback was added.
- Stored bounds use `i32` so future profile-owned numeric definitions can model
  negative ranges without switching to floating point; this slice exercises
  literal `0..=100` with one decimal place.
- Duplicate `(block_id, parameter_id)` additions replace the existing metadata,
  keeping lookup deterministic. This is an additive API choice, not a device
  pack or serialization contract.
- Existing unknown-firmware construction calls `DeviceProfile::new`, so its
  catalog is empty by construction; the focused test asserts this boundary.

## Next packet

`TOP-CMD-001` may consume the exact metadata lookup after this candidate is
integrated.
