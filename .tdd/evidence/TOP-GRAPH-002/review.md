# TOP-GRAPH-002 independent review

Reviewer: `/root/graph002_review` (OpenAI `gpt-5.6-luna`, max reasoning)
Review basis: shared candidate worktree at local starting commit `536d8901ac91ecdbc15e09356800d9f46be401dd`; dependency `TOP-GRAPH-001` is recorded at public integration commit `16a35421979cd1451ca33a57408814ef358b1c15`.
Decision: `REVIEW_APPROVED` (candidate; public integration rerun remains required)

## Findings

- The packet is `READY`, and its declared dependency `TOP-GRAPH-001` is integrated. The candidate stays within the authorized routing source, test, and evidence paths; no root manifest, lockfile, app, native, device-pack, index, or traceability file was changed.
- The canonical focused RED in `red.log` is real and intended. Exit 101 reaches `topology-routing` and reports the deliberately absent typed port/connection/query APIs while the existing node-identity harness compiles. It is not a selector, syntax, fixture, dependency, or unrelated-suite failure.
- The original GREEN proves one output-to-input connection is stored once, queryable from both endpoints, and rejected on an exact duplicate. The adjacent direction test rejects the invalid input-to-output ordering without mutation.
- The prior destination-direction coverage gap is closed by `output_to_output_connection_is_rejected_before_mutation` in `crates/topology_routing/tests/serial_connection.rs`. It uses a valid Output source and valid Output destination, asserts the structured `GraphError::PortDirectionMismatch { expected: Input, actual: Output }` for the destination endpoint, and asserts the graph remains connection-free.
- `destination-red.log` is a valid mutation RED: after the test existed, only the destination-direction guard was temporarily removed; the focused test then failed because the output-to-output connection was accepted. The raw failure is preserved, the production guard is restored in `graph.rs`, and `destination-green.log` shows the same focused test passing. This is a tightly coupled regression cycle, not a circular fixture or a skipped requirement.
- The implementation is minimal and bounded: typed `PortId`/`PortDirection`/`PortRef`, node port storage, output-to-input validation before mutation, deterministic connection storage, bidirectional queries, and explicit duplicate rejection. No cycle, split/merge, device constraint, serialization, visual, simulator, protocol, transport, or hardware behavior was added.
- The worker’s follow-up sweep records the formatter probe failure (mechanical layout only), the correction, final package tests, rustfmt check, and clippy `-D warnings`, each with per-command statuses. I independently reran the focused positive test, destination negative test, package suite, formatter check, and clippy; all exited 0 with no warnings.

## Independent reproduction

Working directory: `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`

```text
cargo test -p topology-routing valid_serial_connection_is_queryable_from_both_ends -- --exact --nocapture  # exit 0
cargo test -p topology-routing output_to_output_connection_is_rejected_before_mutation -- --exact --nocapture # exit 0
cargo test -p topology-routing                                                                               # exit 0
cargo fmt --all -- --check                                                                                   # exit 0
cargo clippy -p topology-routing --all-targets -- -D warnings                                               # exit 0
```

The package suite reports three serial-connection tests and the existing node-identity test passing; doc-tests also pass.

## Verification-label audit and integration conditions

After the parent publishes the bounded source/test/evidence patch and reruns the focused tests plus all packet sweeps from that immutable integration commit, the packet may claim `UNIT_VERIFIED` for this serial output-to-input behavior, direction enforcement, bidirectional queries, and duplicate rejection. `SIMULATOR_VERIFIED` and `HARDWARE_VERIFIED` remain unavailable. This review does not approve endpoint-existence validation, cycle detection, traversal, device compatibility, protocol/transport, accessibility, platform, or hardware behavior.

The remaining evidence gap is integration state: no immutable public commit contains this candidate yet, and the packet/index status remains `READY`. Preserve this review, publish the bounded patch, rerun the required integration sweep, then update the packet/index to `INTEGRATED` only if that sweep passes.
