# TOP-CMD-003 handoff

Status: `GREEN_OBSERVED` (independent review and integration rerun pending)

## Delivered

Added a semantic-only command planner. `GraphMutation` stores a stable
operation ID and a set of explicit semantic dependency IDs. `plan_graph_mutations`
uses a deterministic Kahn topological sort: ready operations are selected by
lexicographic stable operation ID, so equivalent inputs produce identical order
while dependencies precede dependents. Missing dependencies, duplicate IDs,
and dependency cycles return structured `PlanError` values. No wire bytes,
protocol mapping, retries, transport, profile, or hardware behavior was added.

## TDD and verification

- Canonical RED: focused command reached `topology-command-engine` and failed
  with unresolved `GraphMutation`, `SemanticCommandPlan`, and
  `plan_graph_mutations` imports/types (exit 101).
- Canonical GREEN: focused reversed-insertion mutation/dependency fixture passed
  with the expected `input`, `split`, `branch-a`, `branch-b`, `output` order
  (exit 0).
- Final focused rerun after mechanical formatting correction: exit 0.
- Required package, routing, formatter, and Clippy sweeps: all exit 0.
- Initial formatter-only fail-fast exit 1 is preserved in
  `sweep-initial.log`; the required final sweep was rerun fail-fast to
  completion.

## Claim boundary

Candidate may claim `UNIT_VERIFIED` only after independent review and the
parent's integration rerun. It does not claim `BYTE_FIXTURE_VERIFIED`,
`SIMULATOR_VERIFIED`, `PLATFORM_SIMULATOR_VERIFIED`, `PLATFORM_DEVICE_VERIFIED`,
`HARDWARE_VERIFIED`, protocol, transport, UI, accessibility, AI, or release
behavior.

## Integration notes

- No commit was created; this is a shared dirty worktree candidate. The parent
  must land the bounded source/test/evidence paths and rerun the focused test
  plus all required sweeps from the immutable integration commit.
- The plan API intentionally uses semantic IDs/dependencies only. Later
  simulator/protocol packets may consume `SemanticCommandPlan` but must define
  their own wire and acknowledgement contracts.

## Next packet

`TOP-SIM-001` may proceed after independent review and integration of this
candidate.
