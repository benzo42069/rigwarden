# TOP-GRAPH-004 handoff

Status: integrated after independent review and parent rerun; see `integration-sweep.txt`.

## Behavior delivered

An explicit `GraphPolicy::RejectCycles` profile rejects a proposed directed connection when it would close a cycle. The check runs before the connection is inserted. A deterministic node path is returned in `GraphError::CycleDetected`; the graph remains unchanged. `Graph::new()` retains the prior cycle-permissive default so cycle policy is not an undocumented global assumption.

The focused test constructs valid input/output ports for nodes A, B, and C, accepts A���B and B���C, rejects C���A with path `[C, A, B, C]`, and compares the stored connections before and after rejection.

## TDD evidence

- RED: `red-command.txt`, `red.log`, `red-exit-status.txt`; exit 101 for the deliberately missing cycle-policy API only.
- GREEN: `green-command.txt`, `green.log`, `green-exit-status.txt`; focused test exit 0.
- Required sweeps: `sweep-commands.txt`, `sweep.log`, `sweep-exit-statuses.txt`; package tests, fmt check, and Clippy each exit 0.
- Mutation sanity: temporarily disabling only the cycle guard made the focused assertion fail with exit 101; the guard was restored and the final focused/sweep cycle was rerun green.

## Files and design

- `crates/topology_routing/src/graph.rs`: exposes `GraphPolicy`, `Graph::with_policy`, `GraphError::CycleDetected`, and invokes validation before commit.
- `crates/topology_routing/src/validation.rs`: deterministic BTree-backed node adjacency/path search and explicit policy.
- `crates/topology_routing/tests/cycle.rs`: focused unit behavior.

The path search is deterministic because node identities and adjacency sets use ordered collections. It reports the proposed source followed by the existing return path to that source. It does not implement traversal, splits, merges, device-specific limits, or protocol behavior.

## Fixtures and claims

No protocol or hardware fixture is used; this is an L1 Rust unit behavior with locally constructed typed ports. Earned claim after review/integration: `UNIT_VERIFIED`. Not earned: `DEVICE_SPECIFIC_ROUTING_VERIFIED`, `SIMULATOR_VERIFIED`, `HARDWARE_VERIFIED`, accessibility, or UI claims.

## Review/integration

`review.md` records the independent `topology_reviewer` approval. Parent integration published the candidate, reran the focused test and required sweeps, and promoted the packet status.

Next packet: `TOP-GRAPH-005` (deterministic topological traversal), after this packet is integrated.

Blockers: none for the assigned pure Rust cycle; physical hardware and protocol verification are outside scope and unavailable by design.
