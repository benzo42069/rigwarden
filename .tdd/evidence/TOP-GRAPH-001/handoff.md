# TOP-GRAPH-001 worker handoff

Status: candidate complete; awaiting independent topology reviewer and parent integration.

## Behavior delivered

The routing crate now models stable typed node identities separately from mutable logical-grid positions. `Graph::insert` rejects duplicate IDs, `Graph::move_node` changes only the position for an existing node, and `Graph::node` exposes the stored node for observation.

## Strict TDD evidence

- Focused RED: `cargo test -p topology-routing moving_a_node_preserves_its_stable_identity -- --exact --nocapture`, exit 101, unresolved `topology_routing::graph` import; captured in `red.log`.
- Focused GREEN: same selector, exit 0; captured in `green.log` and post-format rerun evidence.
- Required package, formatting, and clippy sweeps are captured in `sweep.log` with per-command statuses. The initial format check failed only on rustfmt layout, was corrected mechanically, and the final fail-fast composite passed.

## Design decisions

- `NodeId` preserves supplied text and derives ordering/equality so the graph can key nodes deterministically without position-derived identity.
- `GridPosition` stores row/column as unsigned coordinates and exposes accessors.
- `Graph` uses a `BTreeMap<NodeId, Node>` for deterministic keying and explicit duplicate rejection.
- No connections, catalogs, device constraints, serialization, or hardware behavior were added.

## Claims

Earned after reviewer/integration rerun: `UNIT_VERIFIED` for this node identity behavior only.

Unavailable: `SIMULATOR_VERIFIED`, `HARDWARE_VERIFIED`, connection/cycle/traversal claims, and device-specific routing compatibility.

## Next packet

`TOP-GRAPH-002` (serial connection) after this candidate is independently reviewed and integrated.

## Blockers

None for the L1 Rust unit cycle. Parent integration must land the candidate, rerun the required sweep, update packet status/index, and retain the independent review record.
