# TOP-GRAPH-004 independent review

Reviewer: `/root/graph004_independent_audit` (independent `topology_reviewer`)
Review date: 2026-08-09
Candidate base: public `main` `b6d8e73b6fa68c5c2bd4012289b3452fa32520e1`, with the packet candidate present in the shared worktree
Decision: **APPROVED** for parent integration

## Dependency and scope audit

- `TOP-GRAPH-002` and `TOP-GRAPH-003` are `INTEGRATED` in the public `main` work-items index and their packet files.
- `TOP-GRAPH-004` remains `READY`; no status promotion was made by this review.
- The candidate diff against public base is limited to the packet-granted `crates/topology_routing/src/graph.rs`, new `src/validation.rs`, and new `tests/cycle.rs`. No root manifest, lockfile, index, app, native, device-pack, or traceability files changed.
- The packet's pure Rust unit layer is appropriate. No protocol fixture, simulator, device, or hardware evidence is required or claimed.

## Behavior and implementation findings

`GraphPolicy::RejectCycles` is an explicit per-graph policy; `Graph::new()` retains the documented permissive default, so cycle rejection is not an undocumented global assumption. `Graph::connect` validates source and destination endpoints and duplicate identity before calling `validation::cycle_path`, then returns `GraphError::CycleDetected` before insertion. Therefore the rejected operation cannot mutate the connection set.

The validator builds ordered node adjacency using `BTreeMap`/`BTreeSet`, then deterministically searches for an existing path from the proposed destination back to its source. For A���B���C followed by C���A, the focused test independently observes `[C, A, B, C]`; the error display also includes the node path. The source-equals-destination case is explicitly represented as a two-node repeated path, so direct self-loops are rejected under the same policy.

The focused fixture creates valid typed input/output ports and proves both acyclic insertions succeed before attempting the prohibited edge. It matches the packet's acceptable RED cause (missing cycle-policy API) and its required no-mutation/path assertions; it does not rely on an endpoint failure or an implementation-derived expected value.

## Evidence audit

- Recorded RED exit `101` is the intended missing `GraphPolicy`, `Graph::with_policy`, and `GraphError::CycleDetected` API failure with valid A/B/C ports (`red.log`, `red-exit-status.txt`).
- Recorded GREEN exit `0` passes the focused test (`green.log`, `green-exit-status.txt`).
- Worker evidence records a focused mutation sanity check in which disabling only the cycle guard failed the assertion, followed by restoration and rerun.
- Worker sweep records package tests, `cargo fmt --all -- --check`, and Clippy as exit `0`; this review reran every required command independently and observed exit `0` for each:

```text
cargo test -p topology-routing prohibited_cycle_is_rejected_without_mutating_graph -- --exact --nocapture  # 0
cargo test -p topology-routing                                                       # 0
cargo fmt --all -- --check                                                           # 0
cargo clippy -p topology-routing --all-targets -- -D warnings                       # 0
```

The review rerun observed the cycle test passing with valid A���B and B���C setup, deterministic C���A rejection, and unchanged stored connections; all three existing routing tests and the missing-endpoint/node-identity tests also passed in the package sweep. No warnings or ignored required tests were reported.

## Verification labels and disposition

This candidate earns only `UNIT_VERIFIED` after parent integration and required integration reruns. It does not earn `DEVICE_SPECIFIC_ROUTING_VERIFIED`, `SIMULATOR_VERIFIED`, accessibility, platform, or `HARDWARE_VERIFIED` claims. Parent integration must land the candidate, rerun the focused test and required sweeps in the integration worktree, add integration evidence, and only then mark the packet `INTEGRATED`. The next declared packet is `TOP-GRAPH-005`.
