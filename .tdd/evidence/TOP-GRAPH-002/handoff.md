# TOP-GRAPH-002 worker handoff

Status: candidate complete; awaiting independent topology review and parent integration.

## Behavior delivered

The routing graph now models typed node ports and stores one directed connection
from an existing output port to an existing input port. Incoming and outgoing
queries return the stored connection from either endpoint. Endpoint direction is
validated before mutation. Exact duplicate connections are rejected with the
structured `GraphError::DuplicateConnection` variant, leaving the graph unchanged.

## Strict TDD evidence

- Focused RED: `cargo test -p topology-routing valid_serial_connection_is_queryable_from_both_ends -- --exact --nocapture`, exit 101. The test reached the routing crate and reported the intentionally absent typed port, connection, and query APIs. Raw output is in `red.log`.
- Focused GREEN: same selector, exit 0 after the minimum implementation. Raw output is in `green.log`.
- Adjacent direction/duplicate tests, package suite, final rustfmt check, and clippy sweep all pass. The initial formatter probe failed only on mechanical layout and was corrected with `cargo fmt --all`; all per-command statuses and raw output are in `sweep.log`.
- Reviewer-identified destination-direction gap was closed with a second tightly coupled cycle. `destination-red.log` records a mutation RED (destination guard temporarily removed; exit 101 because Output-to-Output was accepted), and `destination-green.log` records the restored implementation passing `output_to_output_connection_is_rejected_before_mutation`. The follow-up package/format/clippy sweep is in `destination-sweep.log` with per-command statuses.

## Design decisions

- `PortId`, `PortDirection`, `Port`, and `PortRef` are typed values re-exported from `topology_routing::graph`.
- `Node::with_ports`/`Node::add_port` reject duplicate port identities and retain the existing stable node identity API.
- `Graph` uses a `BTreeSet<Connection>` for deterministic endpoint ordering and exact duplicate detection.
- `Graph::connect` validates source output and destination input before inserting the connection. Missing nodes/ports and direction mismatches are structured errors for later endpoint-validation packets to extend.
- The destination-direction regression test uses a valid Output source and valid Output destination, asserts `GraphError::PortDirectionMismatch { expected: Input, actual: Output }`, and verifies no connection is stored.
- No cycle detection, split/merge/device constraints, serialization, traversal, visual cables, simulator, protocol, or hardware behavior was added.

## Claims

Earned after independent review and parent integration rerun: `UNIT_VERIFIED` for one serial output-to-input connection, bidirectional queries, direction enforcement, and explicit duplicate rejection.

Unavailable: `SIMULATOR_VERIFIED`, `HARDWARE_VERIFIED`, device-specific routing compatibility, protocol/transport behavior, visual or accessibility workflow claims.

## Next packet

`TOP-GRAPH-003` (reject a connection whose source node or port does not exist) after this candidate is independently reviewed and integrated.

## Blockers

None for the Rust unit cycles. Parent integration must land the candidate, rerun the required package/format/clippy sweep in the integration worktree, update the packet/index, and retain the independent review record.
