# TOP-GRAPH-005 executable work-item statement

- Work-item: `TOP-GRAPH-005`
- Requirement: `GRAPH-008` ��� deterministic traversal and command ordering foundation.
- Observable behavior: an acyclic graph with one split and one isolated node produces the same topological node order across equivalent node/connection insertion orders; every node appears once and each dependency precedes its destination.
- Why it matters: deterministic ordering is required by command planning, accessible route descriptions, and graph diffs.
- Layer: Rust unit (Topology L1).
- Preconditions: `TOP-GRAPH-004` is integrated; the existing topology-routing harness passes; no protocol fixture, simulator, hardware, or platform runtime is needed.
- Inputs: two semantically equivalent local graph fixtures containing `input -> split`, split branches to `branch-a`/`branch-b`, both branches to `output`, and an isolated node; node and edge insertion order differs between fixtures.
- Expected result: stable `NodeId` tie-breaking yields `[input, isolated, split, branch-a, branch-b, output]`, one appearance per node, and source index less than destination index for every declared edge.
- Non-goals: device-grid visual ordering, cycle-policy changes, cycle detection redesign, command encoding, protocol/transport behavior, UI/accessibility, simulator, and hardware verification.
- Allowed writes: `crates/topology_routing/src/graph.rs` (amended minimal node-ID iterator), `src/traversal.rs`, `src/lib.rs`, `tests/traversal.rs`, and `.tdd/evidence/TOP-GRAPH-005/**`.
- Forbidden/shared paths: `Cargo.toml`, `Cargo.lock`, `apps/**`, `native/**`, `device-packs/**`, `work-items/index.yaml`, and `docs/requirements/traceability.yaml`.
- Fixtures/provenance: no external/protocol fixture; graph is locally constructed typed data.
- Focused command: `cargo test -p topology-routing topological_traversal_is_deterministic_across_insertion_order -- --exact --nocapture`.
- Expected RED: missing `Graph::topological_traversal` API (exit 101); captured in `red.log` and the amended fixture rerun in `red-amended.log`.
- Required sweeps: `cargo test -p topology-routing`; `cargo fmt --all -- --check`; `cargo clippy -p topology-routing --all-targets -- -D warnings`, executed with `set -euo pipefail` and per-command statuses.
- Claim after GREEN/review/integration: `UNIT_VERIFIED` only.
- Claims unavailable: `SIMULATOR_VERIFIED`, `HARDWARE_VERIFIED`, device-specific routing compatibility, protocol/transport, UI/accessibility, and release/distribution claims.
