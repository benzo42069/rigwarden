# TOP-GRAPH-003 handoff

## Status

Candidate complete; awaiting independent review and parent integration. Packet/index status remains `READY` because the implementer does not self-integrate.

## Behavior delivered

`missing_source_connection_is_rejected_without_mutation` constructs a complete destination and a source node whose requested source port is absent. `Graph::connect` returns `GraphError::PortNotFound` with the exact source node and port identities, and the graph retains no connection.

## TDD evidence

- Test-first baseline: focused test passed before production edits because endpoint validation was already present in TOP-GRAPH-002. It is recorded as baseline GREEN, not RED.
- Mutation RED: removing only the source endpoint lookup/guard caused the focused test to fail (exit 101), demonstrating that the test detects acceptance/mutation of the invalid connection.
- GREEN: restoring the exact guard produced focused exit 0.
- Sweeps: package tests, `cargo fmt --all -- --check`, and clippy with `-D warnings` each exited 0.

## Files changed

- `crates/topology_routing/tests/missing_endpoint.rs`
- `.tdd/evidence/TOP-GRAPH-003/**`

`crates/topology_routing/src/graph.rs` has no final diff; its source-endpoint validation was temporarily removed only for mutation testing and restored exactly before GREEN.

## Design and scope

No new error variant or protocol behavior was invented. Existing `NodeNotFound`/`PortNotFound` structured errors satisfy the packet’s source endpoint contract. No destination-missing extension, cycle detection, traversal, device-specific rule, UI, simulator, transport, or hardware behavior was added.

## Claims earned after review/integration

- `UNIT_VERIFIED` for rejecting a missing source port before graph mutation and preserving node/port identity in the structured error.

## Claims unavailable

- `SIMULATOR_VERIFIED`
- `HARDWARE_VERIFIED`
- device-specific routing compatibility, protocol/transport compatibility, and accessibility/UI workflow claims

## Next packet

`TOP-GRAPH-004` after parent integrates this candidate and updates dependency status.

## Blockers

None for the Rust unit cycle. Parent must independently review, publish/integrate the test/evidence patch, rerun the required sweep from the immutable integration commit, and then promote the packet status.
