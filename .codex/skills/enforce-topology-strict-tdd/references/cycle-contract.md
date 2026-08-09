# Cycle Contract

## Purpose

This contract defines what counts as an observed Topology TDD cycle.

## Before RED

The worker records:

- work-item and requirement IDs;
- repository, branch, commit, and worktree;
- observable behavior;
- non-goals;
- test layer;
- input/fixture;
- expected output/error/tolerance;
- exact test selector;
- expected failure reason;
- allowed and forbidden paths;
- dependencies and their integrated commits;
- required follow-up sweeps;
- claims available and unavailable.

The worker confirms the baseline harness can run. Existing unrelated failure is a blocker.

## RED

A valid RED:

- was executed;
- reached the intended package/module;
- failed because the requested behavior is absent or wrong;
- is reproducible;
- is focused;
- would become green through the required behavior;
- is captured verbatim with exit status.

A missing API/type can be valid when deliberately defining a new interface and unrelated compilation succeeds far enough to name that API precisely.

Invalid RED examples:

- typo/syntax error;
- missing SDK;
- missing unrelated crate/package;
- wrong test name;
- fixture missing or corrupt;
- environment permission;
- stale code generation;
- global suite already red;
- timeout unrelated to behavior;
- test asserts a placeholder known to be false but not the required behavior.

When RED is invalid, fix the harness/test environment without writing production behavior, then rerun.

## GREEN

GREEN requires:

- minimum production change;
- focused test passes;
- no intentional unsupported behavior;
- adjacent tests pass;
- required matrix tests pass;
- warnings explained;
- exact output/status captured.

A focused green with a broken adjacent suite is not green.

## Refactor

Refactor begins only after green.

Allowed:

- naming;
- extraction;
- duplication removal;
- clearer types;
- safe performance improvement;
- test cleanup that preserves behavior;
- documentation synchronized with implemented behavior.

After each meaningful change, rerun affected tests.

Refactor does not include:

- adding a new behavior;
- loosening acceptance;
- changing a golden;
- skipping a platform;
- changing public behavior without a new cycle.

## Review and integration

The implementation cycle produces a candidate patch, not integrated truth.

Independent review checks:

- test validity;
- minimality;
- behavior;
- scope;
- evidence;
- verification labels;
- security/accessibility/provenance as applicable.

Integration then:

- lands patch into integration branch;
- resolves shared-file changes;
- reruns focused and required sweeps;
- updates matrices;
- marks `INTEGRATED`.

Only higher-layer evidence may mark `VERIFIED`.

## Block behavior

Use the narrowest status:

- `BLOCKED_CONTRACT`
- `BLOCKED_DEPENDENCY`
- `BLOCKED_ENVIRONMENT`
- `BLOCKED_FIXTURE`
- `BLOCKED_HARDWARE`

A blocker report includes:

- exact unmet condition;
- evidence;
- safe work completed;
- production behavior not written;
- packet(s) that can proceed independently;
- action needed to unblock.
