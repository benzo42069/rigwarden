Work item: TOP-BOOT-005

Amendment integrity: the parent-provided packet already contained both
amendments (including the `TOP-BOOT-002R` dependency and the Cargo.toml
membership grant) when I read and copied it before harness setup and RED. The
recorded amendment timestamp is preserved verbatim in `work-item.yaml`; local
filesystem mtimes are not used as execution-order evidence.

Behavior delivered:
- `topology_devtools::work_item::validate_yaml` accepts a YAML mapping with a
  non-empty top-level `id`.
- It rejects a missing, null, or empty top-level `id` without panicking and
  returns a stable `missing_field` diagnostic whose path is `id`.
- The parser is intentionally limited to this packet's minimum invariant; it
  does not validate the remaining work-item schema.

Files changed:
- `Cargo.toml` (the packet amendment grants only topology_devtools workspace
  membership setup).
- `crates/topology_devtools/Cargo.toml`.
- `crates/topology_devtools/src/lib.rs`.
- `crates/topology_devtools/src/work_item.rs`.
- `crates/topology_devtools/tests/work_item_validation.rs`.
- `.tdd/evidence/TOP-BOOT-005/*`.

Evidence:
- Focused RED: `cargo test -p topology-devtools missing_id_is_rejected --
  --exact --nocapture`, exit 101, unresolved intended `validate_yaml` API.
- Focused GREEN: same command, exit 0.
- Required sweeps and adjacent `cargo test --workspace`: all exit 0.

Claims earned:
- `UNIT_VERIFIED` for the missing top-level work-item ID rule, pending
  integration rerun after independent review approval.

Claims unavailable:
- `RELEASE_VERIFIED` and `HARDWARE_VERIFIED`.
- Full work-item schema validation, fixture provenance validation, evidence
  completeness validation, asset-policy validation, and CLI behavior.

Pitfalls/deferred behavior:
- No YAML dependency was introduced, so Cargo.lock remains untouched as
  required by packet scope. The parser only establishes the top-level mapping
  and ID invariant; future packets must add independently tested rules rather
  than treating this as full schema validation.
- Cargo temporarily regenerated a lock entry for the new workspace member
  during test execution; it was restored before handoff because Cargo.lock is
  forbidden/shared. The integration owner must decide and own any lockfile
  update when landing the workspace membership.

Next executable packets: TOP-BOOT-006 and TOP-BOOT-007 after integration.
Review: `REVIEW_APPROVED` by `/root/boot005_review`.

Integration: reviewed source/evidence were published at `bfaf0ae08568a13975ed234e20236b8117cf3aa4`. The integration owner adopted the deterministic shared Cargo.lock entry, reran focused tests, workspace formatter, workspace clippy, and workspace tests with exit 0, and promoted this packet to `INTEGRATED`.
