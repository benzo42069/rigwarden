# ADR-0002: Narrow exception for the extracted-kit bootstrap RED record

- Status: Accepted for `TOP-BOOT-001` only
- Date: 2026-08-08
- Owners: integration owner
- Requirements: `QA-001`, `SEC-007`
- Work item: `TOP-BOOT-001`

## Context

The extracted starter kit did not begin as a Git worktree. The declared
repository-contract test was directly observed to fail before Git initialization
and before the baseline files were created. The original manifest corroborates
that those files were absent, but there is no immutable pre-baseline commit or
snapshot from which to rerun that exact historical state.

## Decision

Accept this one `OBSERVED_ONCE` / `NON_REPRODUCIBLE_BASELINE` RED record for the
non-production, file-existence-only `TOP-BOOT-001` repository-contract packet.
It is corroborated by the shipped 120-path manifest and the mutable runtime
audit record. It is not `RED_VERIFIED`, does not prove any product behavior, and
must not be used as precedent for later bootstrap, implementation, protocol,
platform, accessibility, or release packets.

## Consequences

- `TOP-BOOT-001` may be reviewed for the narrow
  `REPOSITORY_CONTRACT_ESTABLISHED` claim if its independent reviewer accepts
  all other evidence.
- The strict reproducible RED–GREEN requirement remains unchanged for every
  production behavior and every later executable packet.
- The record retains its mutable/non-reproducible limitation permanently.

## Verification

Review the original manifest's absence of the four baseline paths, the recorded
red command/exit, the focused green command, and this exception's one-packet
scope. Do not upgrade the RED label.

## References

- `.tdd/evidence/TOP-BOOT-001/red.log`
- `MANIFEST.sha256`
- `docs/DECISION_LOG.md` (`DEC-065`)
