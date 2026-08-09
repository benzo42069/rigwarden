# TOP-GRAPH-003 independent review

Reviewer: `/root/graph003_review` (OpenAI `gpt-5.6-luna`, max reasoning)
Review basis: shared candidate worktree at local starting commit `536d8901ac91ecdbc15e09356800d9f46be401dd`; dependency `TOP-GRAPH-002` is present on public `main` at `6ec7eddf668ca07875a0c4b2cb5d6bbb27ca36d5`.
Candidate decision: `REVIEW_APPROVED` (candidate; parent integration and immutable post-landing rerun remain required)

## Findings

- The packet is `READY`, its declared dependency is `INTEGRATED`, and the candidate stays within the authorized routing test/evidence paths. No root manifest, lockfile, app, native, device-pack, index, or traceability file is part of this candidate.
- The focused baseline was already green because the source-endpoint validation was delivered by `TOP-GRAPH-002`; the evidence labels that pass as baseline only, not as RED. The deliberate mutation cycle is justified: removing only the source endpoint lookup/guard caused the focused test to exit 101 after the invalid missing-port connection was accepted, and restoring the exact guard returned the test to exit 0. This is valid mutation-sanity evidence for a regression test whose behavior pre-existed the packet; no production behavior was added to manufacture a RED.
- `crates/topology_routing/tests/missing_endpoint.rs` builds a complete destination and a source node with a deliberately absent requested source port. It matches `GraphError::PortNotFound` on the exact `node_id` and `port_id`, then asserts that the connection set remains empty. The current `Graph::connect` validates endpoints before inserting into the only mutable graph collection, so this is the correct L1 observable boundary. The missing-source-node branch (`NodeNotFound`) is not separately exercised and is not claimed by this focused test.
- The current source matches the public `TOP-GRAPH-002` `graph.rs` byte-for-byte, including the pre-existing source guard; there is no durable production source diff in this packet. The test and evidence remain bounded to the packet's allowed paths.
- Independent rerun from `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`:
  - `cargo test -p topology-routing missing_source_connection_is_rejected_without_mutation -- --exact --nocapture` — exit 0.
  - `cargo test -p topology-routing` — exit 0; node identity, missing endpoint, all three serial-connection tests, and doc-tests pass.
  - `cargo fmt --all -- --check` — exit 0.
  - `cargo clippy -p topology-routing --all-targets -- -D warnings` — exit 0, no warnings.
- Evidence contains the required packet copy, environment, focused RED/GREEN commands/logs/statuses, sweep commands/log/statuses, files-changed record, and handoff. The mutation RED log is preserved rather than rewritten. No fixture, simulator, platform, protocol, accessibility, or hardware evidence is needed for this L1 packet.

## Verification-label audit and integration conditions

After the parent publishes this bounded test/evidence patch and reruns the focused test plus all required sweeps from that immutable integration commit, the packet may claim `UNIT_VERIFIED` for rejection of the exercised missing source port, preservation of its structured node/port identity, and no connection mutation. `SIMULATOR_VERIFIED` and `HARDWARE_VERIFIED` remain unavailable. Do not infer device-specific routing, protocol/transport, UI/accessibility, platform, or hardware behavior from this unit evidence.

Integration state is still pending: public `main` has the packet at `READY` and does not yet contain `.tdd/evidence/TOP-GRAPH-003/`. Preserve the raw evidence, publish only the bounded candidate, rerun the required integration sweep, then promote the packet/index to `INTEGRATED` if that sweep passes.
