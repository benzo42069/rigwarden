# TOP-BOOT-004 independent review

Reviewer: `/root/ci_review` (read-only final review)
Decision: `REVIEW_APPROVED`

Verified: packet and frozen evidence copy both `READY` and have identical focused GREEN command (`TOPOLOGY_CI_SELF_TEST_FAIL_STAGE=rust_clippy`, expected exit `97`); amendment provenance is recorded. `.github/workflows/ci.yml` installs `rustfmt` and `clippy` for the minimal Rust toolchain. `scripts/ci-local.sh` uses `set -euo pipefail`, exposes six readable stages, and test-only failure injection. Evidence includes accepted RED, raw intermediate GREEN, normal sweep, shell syntax, trailing-whitespace check, and all-stage fail-fast matrix; environment records dirty state and claim boundary; files-changed documents scope and status/workflow amendments.

Independent rerun: normal `bash scripts/ci-local.sh` passed; injected `rust_clippy` exited `97` after `rust_fmt` and before later stages; `bash -n` passed. No product Rust/Dart/native/device-pack paths changed.

Claim audit: `CI_HARNESS_VERIFIED` only after integration rerun. `RELEASE_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`, and `HARDWARE_VERIFIED` remain unavailable.
