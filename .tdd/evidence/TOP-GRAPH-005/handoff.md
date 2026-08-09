# TOP-GRAPH-005 worker handoff

Status: integrated after independent review and parent rerun; see `integration-sweep.txt`.

## Behavior delivered

`Graph::topological_traversal` returns every graph node once in deterministic order for acyclic graphs. Kahn's algorithm uses stable `NodeId` ordering as the tie-breaker whenever multiple nodes are ready, so equivalent split/merge graphs produce the same order across node and connection insertion orders. A parent-authorized `Graph::node_ids` iterator includes isolated nodes. Duplicate connection edges are counted independently for dependency readiness. Existing cycle insertion behavior is unchanged; when a cycle is permitted by the existing policy, unresolved nodes are appended in stable identity order because no topological order exists for that cyclic portion.

## Strict TDD evidence

- Baseline: `cargo test -p topology-routing` passed before the packet test was added; details are in `environment.txt`.
- Focused RED: exact command in `red-command.txt`, exit 101, missing `Graph::topological_traversal` only. After the parent scope amendment, the strengthened isolated-node fixture produced the same intended RED; raw rerun is in `red-amended.log`.
- Focused GREEN: exact command in `green-command.txt`, exit 0; the test asserts stable output, dependency ordering, uniqueness, and isolated-node inclusion.
- Required sweeps: the initial fail-fast attempt stopped at a test-only rustfmt diff (exit 1) before clippy; `sweep-initial.log` and per-command statuses preserve that failure. After the mechanical test formatting correction, package tests, rustfmt check, and clippy with `-D warnings` all exited 0 in the final fail-fast sweep.

## Design decisions and pitfalls

- Stable node identity (`NodeId`'s existing `Ord`) is the documented tie-breaker; grid position and insertion order do not affect traversal.
- The only graph.rs production addition is the parent-authorized `pub(crate) fn node_ids` accessor required to include isolated nodes.
- Node-level outgoing edges use counts so multiple port connections between the same node pair do not release a destination before all dependencies are processed.
- No cycle policy, cycle error, protocol encoding, device geometry, simulator behavior, or hardware behavior was changed.
- A topological order is undefined for a permitted cycle; the method preserves the one-appearance invariant with deterministic unresolved-node fallback while leaving insertion policy untouched. Acyclic graphs receive the dependency-before-destination guarantee.

## Claims

Earned after independent review and integration rerun: `UNIT_VERIFIED` for deterministic topological traversal of the current graph types, including split branches, stable tie-breaking, dependency ordering, uniqueness, and isolated nodes.

Unavailable: `SIMULATOR_VERIFIED`, `HARDWARE_VERIFIED`, device-specific routing compatibility, protocol/transport compatibility, UI/accessibility workflow claims, and command-ordering claims beyond this pure graph traversal.

## Changed files

- `crates/topology_routing/src/graph.rs`
- `crates/topology_routing/src/traversal.rs`
- `crates/topology_routing/src/lib.rs`
- `crates/topology_routing/tests/traversal.rs`
- `.tdd/evidence/TOP-GRAPH-005/**`

No shared manifest or index change was made or proposed.

## Next packet and blockers

Next executable dependents are `TOP-A11Y-001` and `TOP-CMD-003` after this candidate is independently reviewed and integrated. No environment, fixture, protocol, or hardware blocker exists for this L1 Rust cycle. Parent integration must land the candidate, rerun the focused test and required sweeps from the integration worktree, update packet/index status, and retain independent review evidence.
