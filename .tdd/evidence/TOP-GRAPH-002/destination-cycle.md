# TOP-GRAPH-002 destination-direction cycle

This tightly coupled follow-up cycle closes a reviewer-identified coverage gap:
the original adjacent test exercised an invalid Input-as-source/Output-as-
destination ordering, but did not prove that a valid Output source cannot target
an Output destination.

The new `output_to_output_connection_is_rejected_before_mutation` test builds
two existing nodes with Output ports, asserts the exact structured
`GraphError::PortDirectionMismatch { expected: Input, actual: Output }`, and
asserts that the graph remains connection-free.

For an observed mutation RED, only the destination-direction guard was removed
temporarily after the test was written. The focused test then failed because the
mutated implementation accepted the connection. The guard was restored
immediately; no mutation probe remains in production source. The raw mutation
RED and restored GREEN are in `destination-red.log` and `destination-green.log`.
