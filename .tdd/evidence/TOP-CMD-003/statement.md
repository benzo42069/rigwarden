# TOP-CMD-003 executable work-item statement

- Work-item: `TOP-CMD-003`
- Requirements: `GRAPH-008` deterministic traversal and command ordering; `UNDO-005` truthful ordering foundation for partial completion.
- Observable behavior: equivalent semantic graph-mutation sets produce one identical operation order independent of insertion order, while every declared dependency precedes its dependent.
- Why it matters: deterministic semantic ordering prevents repeated planning from producing order-dependent hardware behavior and gives later acknowledgement/partial-completion work a stable sequence.
- Layer: Rust unit (Topology L1).
- Preconditions: `TOP-CMD-001` and `TOP-GRAPH-005` are integrated prerequisites in the packet; the existing command-engine harness is workspace-enabled; no protocol fixture, simulator, transport, platform, or hardware runtime is needed.
- Inputs: two local typed sets containing `input`, `split`, `branch-a`, `branch-b`, and `output`; the sets are semantically equivalent but reverse insertion order for operations and dependency declarations.
- Expected result: planning returns the same operation ID sequence for both inputs, uses operation ID lexical order as the stable key whenever ready operations are tied, and never places a dependent before a declared dependency.
- Explicit non-goals: atomic batches, protocol-specific ordering, retries, transport, wire bytes, profile changes, and hardware behavior.
- Allowed writes: `crates/topology_command_engine/src/plan.rs`, `crates/topology_command_engine/src/lib.rs`, `crates/topology_command_engine/tests/deterministic_plan.rs`, and `.tdd/evidence/TOP-CMD-003/**`.
- Forbidden/shared paths: `Cargo.toml`, `Cargo.lock`, `apps/**`, `native/**`, `device-packs/**`, `work-items/index.yaml`, and `docs/requirements/traceability.yaml`.
- Fixtures/provenance: no external or protocol fixture; the test uses locally constructed typed operation IDs and dependencies.
- Focused command: `cargo test -p topology-command-engine equivalent_mutation_sets_produce_same_operation_order -- --exact --nocapture`.
- Expected RED: the focused test reaches `topology-command-engine` and reports missing `GraphMutation`, `SemanticCommandPlan`, and `plan_graph_mutations` APIs (exit 101); this is captured in `red.log`.
- Required sweeps: `cargo test -p topology-command-engine`; `cargo test -p topology-routing`; `cargo fmt --all -- --check`; `cargo clippy -p topology-command-engine --all-targets -- -D warnings`, each run fail-fast with individual status evidence.
- Claim after GREEN/review/integration: `UNIT_VERIFIED` only.
- Claims unavailable: `BYTE_FIXTURE_VERIFIED`, `SIMULATOR_VERIFIED`, `PLATFORM_SIMULATOR_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`, `HARDWARE_VERIFIED`, protocol/transport, UI/accessibility, AI, and release/distribution claims.
