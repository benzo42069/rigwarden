# Test Quality

## A good test

- names one behavior;
- asserts observable results;
- uses independent expected values;
- fails before the change;
- isolates controlled time/randomness/filesystem/network;
- has a justified tolerance;
- includes failure-path assertions;
- remains readable enough to diagnose;
- maps to requirements;
- proves only its layer.

## Reject implementation duplication

A test should not calculate expected output using the same algorithm as production.

Prefer:

- literal independent vectors;
- hand-verified small examples;
- published vectors;
- property invariants independent of the implementation strategy;
- cross-implementation comparison with provenance.

## Determinism

Control:

- clock;
- random seed;
- scheduling;
- locale;
- timezone;
- filesystem;
- network;
- device enumeration;
- IDs.

Use timeouts as safety limits, not assertions of arbitrary speed unless the requirement is performance.

## Tolerances

Every tolerance states:

- units;
- reason;
- expected platform variation;
- threshold source;
- what failure means.

Do not broaden a tolerance to hide a failure.

## Golden tests

A golden is accepted only when:

- source/provenance is documented;
- review confirms it;
- update is a separate visible decision;
- test would catch material regression.

Never auto-update goldens in ordinary test runs.

## Mocks and fakes

Use fakes for:

- isolated contracts;
- deterministic failure paths;
- simulator behavior.

Do not use a fake to claim:

- CoreMIDI works;
- BLE large SysEx works;
- AM4/FM3 works;
- VoiceOver/TalkBack works;
- a provider supports structured output;
- a store build installs.

## Skips

A required test that skips is not pass.

Allowed optional/gated tests must:

- be named;
- state required resource;
- fail clearly when explicitly invoked without it;
- not affect clean-clone suite;
- never be counted as evidence unless actually run.

## Flakiness

Do not mark flaky tests ignored as a fix.

Investigate:

- uncontrolled time;
- shared state;
- order;
- races;
- resource leaks;
- simulator nondeterminism;
- OS lifecycle;
- hardware instability.

Record rerun evidence, but repeated pass does not erase a known race.

## Mutation sanity

For critical validators/parsers, use mutation testing or deliberate negative variants when feasible to confirm tests detect:

- removed check;
- inverted bound;
- fallback enabled;
- stale response accepted;
- write allowed in read-only;
- unknown field discarded;
- semantic action missing.

## Review questions

1. Would this test pass with no meaningful implementation?
2. Does it assert the requested behavior rather than a helper?
3. Is expected data independent?
4. Is the failure meaningful?
5. Is the layer claim accurate?
6. Are negative/error cases covered?
7. Is the test robust across supported platforms?
8. Was anything weakened to make it pass?
