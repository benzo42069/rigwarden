# TOP-GRAPH-005 independent review

Reviewer: `/root/graph005_independent_audit` (`topology_reviewer`)
Review timestamp: 2026-08-09T13:38:00-05:00
Public integration base: `e14f1440cc4703229c1a1274a2121fbe092dc9c2`
Candidate state: local, uncommitted; source/evidence frozen by the implementer

## Findings

APPROVED.

- The focused RED is valid: `red.log` reaches `topology-routing` and fails only because `Graph::topological_traversal` is missing. The amended RED repeats that intended missing-API failure after adding the isolated-node assertion.
- The candidate source scope matches the packet and amendment. Compared with the public base, `graph.rs` adds only the parent-authorized read-only `pub(crate) fn node_ids` iterator; `lib.rs` registers `traversal`; `traversal.rs` and `tests/traversal.rs` are the declared additions. No manifest, lockfile, index, or unrelated path changed.
- `topological_traversal` is a Kahn traversal over all node IDs. `BTreeMap`/`BTreeSet` make ready-node selection and outgoing dependency processing deterministic, with `NodeId`'s existing `Ord` as the documented tie-breaker. The two fixture graphs contain the same node identities and directed edges but reverse node/connection insertion order; the expected output is not plain identity sorting, so the test independently exercises dependency ordering.
- The focused test asserts the same output for both insertion orders, the literal stable order (`input`, `isolated`, `split`, `branch-a`, `branch-b`, `output`), all five dependencies before destinations, six total entries, and uniqueness. The isolated node is explicitly included. The method takes `&self` and does not mutate graph state, so repeated calls are deterministic by construction; no stateful ordering is present.
- Node-level edge counts correctly account for multiple distinct connections between one source/destination pair: each source is processed once and releases the destination only after all of that source's edges are consumed. Stored connections are validated at insertion, so the internal `expect` invariants are upheld by the existing graph API.
- Cycle behavior does not violate the packet non-goal. `graph.rs`/`validation.rs` cycle insertion policy is unchanged. For the default `AllowCycles` policy, the method documents that no topological order exists for unresolved cyclic nodes and appends them once in stable identity order without rejecting or mutating the graph. Acyclic dependency guarantees remain explicitly scoped to acyclic graphs; no cycle-detection or command-encoding claim is made.
- Evidence includes the packet copy/amendment, environment, focused RED/GREEN, sweep commands/statuses, file scope, and handoff. The worker's final sweep log is abbreviated at the end of the cached Clippy output, but its per-command exit-status record is 0 and the independent rerun below supplies fresh command evidence.

## Independent reruns (local candidate)

Commands were run from `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`:

1. `cargo test -p topology-routing topological_traversal_is_deterministic_across_insertion_order -- --exact --nocapture` ��� exit 0.
2. `cargo test -p topology-routing` ��� exit 0 (cycle, missing-endpoint, node-identity, serial-connection, and traversal tests; doc-tests clean).
3. `cargo fmt --all -- --check` ��� exit 0.
4. `cargo clippy -p topology-routing --all-targets -- -D warnings` ��� exit 0.

## Verification-label audit

`UNIT_VERIFIED` is supported for deterministic topological traversal of the current in-memory graph types, including stable tie-breaking, dependency ordering, uniqueness, and isolated nodes. `SIMULATOR_VERIFIED`, `HARDWARE_VERIFIED`, device-specific routing compatibility, protocol/transport compatibility, accessibility workflow, and command-ordering claims remain unavailable. Integration still must land the candidate and rerun the focused test and required sweeps before changing packet/index status to `INTEGRATED`.

Decision: APPROVED for parent integration.
