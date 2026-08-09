# TOP-REG-003 independent review

Reviewer: `/root/reg003_review` (`topology_reviewer`, OpenAI `gpt-5.6-luna/max`)
Review date: 2026-08-09
Review basis: `TOP-REG-001` integrated registry API in the shared worktree,
the complete TOP-REG-003 candidate source/test/evidence, and independent
reruns from `/Users/benzo/Desktop/Topology_Codex_Starter_Kit_v1`.
Decision: `REVIEW_APPROVED` (candidate; parent integration rerun remains required)

## Dependency and scope audit

- `TOP-REG-001` is the declared dependency and its exact typed registry
  resolver is present in the candidate worktree.
- Candidate production/test edits are limited to
  `crates/topology_device_registry/src/lib.rs`,
  `crates/topology_device_registry/src/resolve.rs`,
  `crates/topology_device_registry/tests/numeric_parameter_metadata.rs`, and
  this evidence directory. No protocol, transport, Dart/Flutter, native,
  device-pack, root manifest, lockfile, or work-item index paths were changed
  by the candidate.
- The packet is a pure in-memory Rust unit slice. No JSON loading, signature,
  byte, simulator, platform, accessibility, or hardware claim is made.

## Behavior and implementation findings

- `NumericParameterMetadata` (resolve.rs:8-39) stores `min_stored` and
  `max_stored` as `i32` and `decimal_places` as `u8`, derives `Eq`, and never
  uses floating point or NaN semantics. Its accessors return the literal stored
  values without range inference.
- `DeviceProfile` retains an additive private catalog (resolve.rs:83-92),
  while `DeviceProfile::new` keeps the existing five-argument constructor
  (resolve.rs:94-110). `with_numeric_parameter`/`add_numeric_parameter`
  populate a deterministic exact `(block_id, parameter_id)` key, and
  `numeric_parameter` returns `None` for a missing key (resolve.rs:113-160).
- Exact resolution clones the matched profile, so its metadata remains
  profile-owned. `resolve_session` first performs exact matching and only then
  creates a new read-only profile for a known family/model unknown firmware
  (resolve.rs:291-319). That constructor starts with an empty numeric catalog;
  the focused test asserts that an unknown `1.1` session cannot see the exact
  `1.0` `amp-1/gain` metadata. No nearest-firmware or catalog inheritance path
  was added.
- No API accepts protocol bytes, transport handles, device-pack data, or
  floating-point values, so the packet's non-goals and trust boundary remain
  intact. Duplicate additions replace the same exact key; because the catalog
  is private, lookup cannot observe duplicate entries from ordinary use.

## TDD and evidence audit

- The canonical RED is valid: `.tdd/evidence/TOP-REG-003/red.log` reaches
  `topology-device-registry` and fails only on the deliberately absent
  `NumericParameterMetadata`, `with_numeric_parameter`, and
  `numeric_parameter` APIs (exit `101`). The test fixture timestamp precedes
  the production source timestamp, and the source was not present before this
  intended failure.
- The canonical GREEN passes the one focused test (exit `0`). That test uses
  independent literal bounds (`0`, `100`, `1`), checks a missing parameter, and
  checks that unknown firmware gets no inherited definition.
- Independent reruns from the shared worktree all passed:

  ```text
  cargo test -p topology-device-registry exact_numeric_parameter_metadata_is_profile_owned -- --exact --nocapture  # exit 0
  cargo test -p topology-device-registry                                                               # exit 0
  cargo test -p topology-domain                                                                        # exit 0
  cargo fmt --all -- --check                                                                            # exit 0
  cargo clippy -p topology-device-registry --all-targets -- -D warnings                                # exit 0
  ```

  No required test is skipped, and clippy reported no warnings.
- Evidence hygiene note: the separate command-not-found shell mistake in
  `red-invalid-harness.*` is correctly marked
  `accepted_as_intended_red=no` after correction by the implementation agent.
  It is excluded from the TDD proof; the canonical `red.log` above is the
  valid RED.

## Verification-label audit and integration conditions

After the parent publishes this bounded candidate and reruns the focused test
and every packet sweep from the immutable integration commit, it may claim
`UNIT_VERIFIED` for exact profile-owned numeric metadata and unknown-firmware
non-inheritance. `PACK_SIGNATURE_VERIFIED`, `BYTE_FIXTURE_VERIFIED`,
`HARDWARE_VERIFIED`, protocol/transport, simulator, platform, accessibility,
and UI claims remain unavailable. Parent integration must preserve this review,
add post-landing command evidence, and only then promote the packet/index to
`INTEGRATED`.
