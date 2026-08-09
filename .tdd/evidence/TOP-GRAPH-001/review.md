# TOP-GRAPH-001 independent review

Reviewer: `/root/graph001_review` (OpenAI `gpt-5.6-luna`, max reasoning)
Review basis: shared candidate worktree at local starting commit `536d8901ac91ecdbc15e09356800d9f46be401dd`; the parent-owned routing harness is public at `a4823fd92a9bb216224068e7a759364dc48da89e`.
Decision: `REVIEW_APPROVED` (candidate; integration rerun remains required)

## Findings

- Dependency/precondition is satisfied in the public index at `a4823fd`: `TOP-DOM-001` is `INTEGRATED`, and the parent harness commit adds only the product-empty `topology-routing` workspace member, manifest, and documentation-only `lib.rs`.
- The RED is real and intended. `red.log` records exit 101 from the focused selector with only the unresolved `topology_routing::graph` import after the routing crate compiled. It reaches the intended package and is not caused by a missing workspace member, unrelated dependency, fixture, or selector. The evidence timestamp precedes the candidate source timestamps, consistent with test-first execution.
- The GREEN is minimal and behavioral. `node_identity.rs` inserts `amp-1`, moves it to a new row/column, observes the same `NodeId` and new `GridPosition`, then verifies a duplicate insertion returns an error. It does not rely on private storage or duplicate the implementation algorithm.
- `graph.rs` implements only the packet minimum: typed `NodeId`, row/column position, node insertion, stable-identity move, lookup, and duplicate/missing-node errors. `BTreeMap<NodeId, Node>` provides deterministic identity keying. No connections, port/catalog logic, device placement constraints, serialization, simulator, transport, or hardware behavior are present.
- Candidate evidence preserves the initial rustfmt failure, the mechanical `cargo fmt --all` correction, the post-format focused rerun, and the final fail-fast composite. No failure is hidden or relabeled.
- Scope is bounded to the authorized routing crate/test/evidence paths. Root `Cargo.toml`/`Cargo.lock` harness changes are explicitly parent-owned by the packet amendment; no `apps/**`, `native/**`, or `device-packs/**` changes were made by the worker. `cargo tree -p topology-routing` is dependency-free.

## Independent reproduction

All commands were rerun from `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1` without source edits:

```text
cargo test -p topology-routing moving_a_node_preserves_its_stable_identity -- --exact --nocapture  # exit 0
cargo test -p topology-routing                                                                        # exit 0
cargo fmt --all -- --check                                                                            # exit 0
cargo clippy -p topology-routing --all-targets -- -D warnings                                       # exit 0
```

The focused rerun reports one passing `moving_a_node_preserves_its_stable_identity` test; the package suite and doc-tests report no failures, and clippy emits no warnings.

## Verification-label audit and gaps

After the parent lands the candidate and reruns the required sweep in the integration worktree, the packet may claim `UNIT_VERIFIED` for this node identity behavior only. `SIMULATOR_VERIFIED` and `HARDWARE_VERIFIED` remain unavailable. This review does not approve connections, cycle detection, traversal, catalog, serialization, device compatibility, transport, or platform claims.

Remaining integration evidence gap: no public commit contains the candidate graph source yet, and the required post-landing integration rerun/status update is still pending. Preserve this review with the candidate evidence and update the packet/index only after that rerun passes.
