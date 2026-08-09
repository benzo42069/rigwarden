# TOP-GRAPH-001 parent harness preparation

Parent integration owner prepared the product-empty Cargo harness before the behavior test:

- Added `crates/topology_routing` as the sole new root workspace member.
- Added a dependency-free manifest and documentation-only library root, plus Cargo's generated package-only lockfile entry.
- Added no graph node, connection, catalog, serialization, simulator, or hardware behavior.

Reason: the packet's focused RED must reach `topology-routing` and fail only for the intended absent graph API; a missing workspace member is explicitly unacceptable RED evidence. The work-item amendment records this mechanical parent-owned scope change.
