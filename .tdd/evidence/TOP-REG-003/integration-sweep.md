# TOP-REG-003 parent integration sweep

Date: 2026-08-09

The parent integration owner reran the reviewed candidate's focused test and
every required sweep before status promotion. All commands exited `0`:

```text
cargo test -p topology-device-registry exact_numeric_parameter_metadata_is_profile_owned -- --exact --nocapture
cargo test -p topology-device-registry
cargo test -p topology-domain
cargo fmt --all -- --check
cargo clippy -p topology-device-registry --all-targets -- -D warnings
```

This records `UNIT_VERIFIED` only for the in-memory exact numeric metadata
lookup and unknown-firmware non-inheritance. It does not verify a device pack,
protocol, fixture, simulator, platform, or hardware.
