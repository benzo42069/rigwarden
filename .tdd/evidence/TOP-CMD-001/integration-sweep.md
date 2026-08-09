# TOP-CMD-001 immutable integration sweep

Candidate commit: `a4cbb8da85654721a55f7ad869fd71777893c4ae`
Date: 2026-08-09

The parent first matched public GitHub blobs for the workspace manifest/lock,
command validator/test, bridge configuration, and both source packets to the
local reviewed tree. It then reran the focused test and every required CMD
sweep; all exited `0`:

```text
cargo test -p topology-command-engine valid_parameter_mutation_is_typed_but_not_encoded -- --exact --nocapture
cargo test -p topology-command-engine
cargo test -p topology-device-registry
cargo fmt --all -- --check
cargo clippy -p topology-command-engine --all-targets -- -D warnings
```

This earns `UNIT_VERIFIED` only for the literal profile-bound semantic
mutation. It does not verify bytes, protocol, transport, simulator, platform,
hardware, AI, UI, or accessibility behavior.
