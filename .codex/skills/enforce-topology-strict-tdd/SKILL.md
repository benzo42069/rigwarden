---
name: enforce-topology-strict-tdd
description: Enforce observed red-green-refactor test-driven development for every Topology production behavior, bug fix, refactor, protocol change, device-profile change, transport integration, UI behavior, accessibility contract, AI mutation rule, build change, and release-tooling change. Use whenever Codex or a subagent writes or changes Topology Rust, Dart/Flutter, Swift, Kotlin, profile data, protocol fixtures, simulator behavior, CI, packaging, signing, or documentation that makes a testable product claim. Also use when defining the first executable work item or auditing whether claimed implementation followed strict TDD.
---

# Enforce Topology Strict TDD

Treat TDD as an evidence protocol, not as the presence of tests after implementation.

No agent may write production behavior until a focused test has been executed and observed failing for the intended reason.

A work item is not implemented merely because:

- source code exists;
- a test was written but not run;
- a mock passed;
- a simulator passed;
- compilation succeeded;
- another platform passed;
- a device profile looks correct on paper;
- an AI agent reports that the code should work;
- the final command in a non-fail-fast shell returned zero.

Every claim must name the test layer and evidence that actually proves it.

## Binding Scope

This policy applies to:

- Rust domain and protocol crates;
- Flutter application behavior;
- Swift CoreMIDI and platform bridge code;
- Kotlin Android MIDI, USB host, and BLE code;
- desktop transports and network bridge code;
- device and firmware definition packs;
- preset parsing, preservation, editing, and serialization;
- command planning and hardware-write behavior;
- undo, redo, snapshots, branching, and recovery;
- routing validation;
- simulator and replay behavior;
- Capture Lab tooling;
- accessibility semantics and nonvisual workflows;
- AI provider adapters and AI mutation-plan validation;
- local storage, file import/export, and migration;
- cryptographic pack verification;
- build, CI, release, packaging, and store tooling;
- bugs, regressions, and refactors in any of those areas.

Documentation-only prose that makes no executable product claim does not require a RED cycle. Documentation generators, validators, links, schemas, compatibility reports, and release manifests do.

## Read the Topology Contracts

Before beginning executable work, read:

- `references/cycle-contract.md`
- `references/evidence-record.md`
- `references/topology-test-ladder.md`
- `references/test-quality.md`
- `references/subagent-work-packet.md`
- `references/protocol-fixture-provenance.md`
- `references/accessibility-contract.md`
- `references/hardware-verification.md`

Do not continue when a required reference is absent or internally contradictory. Report the work item as `BLOCKED_CONTRACT` and identify the exact missing or conflicting rule.

## Establish the Smallest Executable Harness

Before the first behavior in a language, package, or platform:

1. Confirm the repository root.
2. Record the current branch, commit, and worktree state.
3. Confirm that the assigned work item is `READY`.
4. Confirm that all declared dependencies are `INTEGRATED`.
5. Confirm the compiler, SDK, package manager, lockfile, and platform environment.
6. Identify the narrowest executable test command.
7. Confirm that fixtures required by the behavior exist and have valid provenance.
8. Confirm whether the task requires a simulator, emulator, attached device, physical modeler, signing credential, or network access.
9. Create only the minimum manifest, module, target, and test harness needed to execute the first behavior.
10. Run the empty or baseline harness and record the result.

Bootstrap work may precede the first behavior test only when it contains no product behavior.

Do not:

- create every crate from the architecture diagram;
- create empty future modules;
- add placeholder implementations;
- add speculative dependencies;
- create fake product APIs merely to make the repository appear complete;
- build a generic framework before an observable vertical slice requires it.

Create packages incrementally when the next executable behavior needs them.

If the required test cannot run in the current environment, stop that work item after test design or harness preparation and mark it `BLOCKED_ENVIRONMENT`.

Do not write unrun production code and promise that TDD will happen later.

Blocking one hardware or platform task does not block unrelated work items whose tests are executable.

## Required Work-Item Statement

Before writing a test, state:

- Work-item ID.
- Requirement ID.
- One observable behavior.
- Why the behavior matters.
- Layer being tested.
- Preconditions.
- Inputs.
- Expected output, state transition, error, or tolerance.
- Explicit non-goals.
- Allowed write paths.
- Forbidden or shared paths.
- Fixture and provenance requirements.
- Exact narrow test command.
- Expected RED reason.
- Required adjacent and matrix tests.
- Verification claim available after GREEN.
- Verification claims that remain unavailable.

Do not accept broad behaviors such as:

- “Implement the protocol layer.”
- “Build the routing editor.”
- “Add accessibility.”
- “Support FM3.”
- “Implement Android.”
- “Add AI.”
- “Finish the offline editor.”

Split those into independently testable leaf behaviors.

## Run One Observed Red-Green-Refactor Cycle

For each behavior:

1. Write the smallest test that would fail if the behavior were absent or regressed.
2. Run only that focused test.
3. Capture:
   - exact command;
   - working directory;
   - relevant environment;
   - toolchain versions;
   - stdout and stderr;
   - exit status;
   - test duration;
   - current commit.
4. Confirm the failure is the intended `RED`.
5. Reject the RED when it is caused by:
   - syntax failure unrelated to the intended missing API;
   - missing unrelated dependency;
   - broken fixture;
   - wrong test selector;
   - stale generated files;
   - unavailable emulator or device;
   - network failure;
   - permission failure;
   - unrelated compiler error;
   - pre-existing failing tests;
   - test timeout unrelated to the requirement.
6. Implement the minimum production behavior required for that test.
7. Run the focused test and observe `GREEN`.
8. Run all adjacent package tests.
9. Run all required cross-boundary and platform matrix tests.
10. Refactor only while the focused and adjacent tests remain green.
11. Rerun affected tests after every meaningful refactor.
12. Record discoveries, pitfalls, deferred behaviors, and the next executable work item.
13. Submit the patch and evidence for independent review.
14. Do not mark the work item `INTEGRATED` until the integration agent has applied it and rerun the required sweep in the integration worktree.

A deliberately missing symbol or type is a valid first RED when:

- the test reaches the intended package;
- unrelated dependencies compile;
- the diagnostic precisely names the missing API;
- the missing API is part of the intended behavior;
- no knowingly incorrect production implementation was added just to manufacture another failure.

Unrelated compilation failure is never TDD evidence.

## Use Fail-Fast Evidence Sweeps

Composite commands must use fail-fast behavior:

```bash
set -euo pipefail
```

When commands cannot safely share one shell, run and record each separately.

Never infer that tests, formatting, linting, auditing, code generation, accessibility checks, and packaging all passed because the final command returned zero.

Record the exit status of every required command.

## Use the Correct Topology Test Layer

Use the cheapest layer that proves the requirement, then add higher layers wherever the claim crosses a boundary.

### Layer 1: Pure unit and property tests

Use for:

- Rust value objects;
- state transitions;
- routing invariants;
- parameter bounds;
- enum mappings;
- profile selection;
- command planning;
- diffing;
- undo and redo;
- storage migrations;
- deterministic AI-plan validation;
- pure Dart presentation logic.

This layer does not prove protocol-byte compatibility, FFI behavior, operating-system behavior, visual accessibility, or hardware support.

### Layer 2: Byte-exact protocol and file-codec tests

Use for:

- message encoding;
- message decoding;
- checksums;
- framing;
- escaping;
- fragmentation;
- unknown-field preservation;
- lossless preset round trips;
- malformed input rejection;
- firmware-specific mappings;
- exact device/profile selection.

Use provenance-approved golden fixtures and independent expected bytes.

A round-trip test alone is insufficient because an encoder and decoder may share the same mistake.

Require independent known input or expected output for compatibility claims.

### Layer 3: Simulator and replay integration

Use for:

- discovery flows;
- request and response sequencing;
- acknowledgements;
- read-back verification;
- timeouts;
- retries;
- disconnects;
- reconnection;
- partial batch completion;
- cancellation;
- transport fragmentation;
- stale response rejection;
- multiple endpoint behavior;
- session recovery.

Simulator success proves simulator compatibility only.

It does not prove physical hardware compatibility.

### Layer 4: Flutter widget, layout, semantics, and golden tests

Use for:

- adaptive phone and tablet layouts;
- orientation changes;
- focus order;
- keyboard and switch navigation;
- semantic labels;
- state announcements;
- large text;
- reduced motion;
- non-color state communication;
- routing-list accessibility;
- import and edit flows;
- stage-control locking;
- PNG asset state selection.

A screenshot or golden image does not prove interaction or accessibility.

A semantics test does not, by itself, prove VoiceOver or TalkBack behavior on a real operating system.

### Layer 5: FFI and native-platform integration

Use for:

- Rust-to-Dart bindings;
- Dart-to-Swift bridge behavior;
- Dart-to-Kotlin bridge behavior;
- CoreMIDI adapter contracts;
- Android MIDI APIs;
- Android USB host behavior;
- BLE adapter behavior;
- permission handling;
- endpoint enumeration;
- hotplug and unplug behavior;
- app lifecycle transitions;
- cancellation and shutdown.

A fake operating-system adapter proves only the adapter contract.

Platform claims require the relevant SDK, emulator or simulator, and native integration suite.

### Layer 6: End-to-end deterministic application replay

Use for complete paths such as:

```text
UI action
→ typed application command
→ Rust validation
→ deterministic command plan
→ protocol encoding
→ simulated transport
→ simulated device mutation
→ acknowledgement or read-back
→ application state update
→ persistent undo journal
```

This proves the complete application path against the simulator fixture.

It does not establish physical-device compatibility.

### Layer 7: Physical hardware-in-the-loop verification

Use for claims involving:

- real modeler discovery;
- real transport behavior;
- actual firmware compatibility;
- complete preset reads;
- parameter writes;
- scene and channel changes;
- block insertion and routing;
- tuner and tempo streaming;
- cab transfer;
- disconnect recovery;
- sustained editing sessions;
- throughput and rate limiting;
- specific adapters;
- specific mobile devices.

Record:

- modeler model and hardware revision;
- firmware version;
- mobile device;
- mobile operating-system version;
- connection path;
- adapter model where applicable;
- test procedure;
- observed result;
- sanitized logs;
- fixture checksum where a capture is retained.

Only this layer may grant `HARDWARE_VERIFIED`.

### Layer 8: Release and distribution verification

Use for:

- clean-clone builds;
- reproducible builds;
- signed application packages;
- TestFlight;
- App Store;
- Play testing;
- Play production;
- F-Droid flavor;
- platform permissions;
- privacy manifests;
- entitlements;
- store metadata;
- upgrade and migration behavior.

A local debug build does not prove store-distribution behavior.

## Verification Labels

Use these labels precisely:

- `UNIT_VERIFIED`
- `BYTE_FIXTURE_VERIFIED`
- `SIMULATOR_VERIFIED`
- `CAPTURE_VERIFIED`
- `SEMANTICS_VERIFIED`
- `PLATFORM_SIMULATOR_VERIFIED`
- `PLATFORM_DEVICE_VERIFIED`
- `HARDWARE_VERIFIED`
- `COMMUNITY_CONFIRMED`
- `EXPERIMENTAL`
- `READ_ONLY`
- `UNSUPPORTED`
- `BLOCKED`

Never promote a label based on confidence, code inspection, or an agent’s opinion.

A modern device profile may ship as `EXPERIMENTAL` when its architecture, schema, byte fixtures, and simulator paths are complete but physical validation is absent.

The public beta requires the specifically selected flagship device vertical slices to satisfy their hardware-verification matrix.

## Protocol and Fixture Integrity

Never guess protocol bytes and present the result as compatibility.

Every protocol fixture must have a sidecar provenance record containing:

- fixture ID;
- device family;
- exact device model;
- firmware version;
- transport;
- message direction;
- feature or command represented;
- capture or source date;
- source category;
- source reference;
- sanitization performed;
- redistribution permission;
- fixture checksum;
- expected parser result;
- confidence and verification status.

Allowed source categories include:

- published vendor specification;
- permissively licensed open-source project;
- user-owned hardware capture;
- independently generated simulator fixture;
- community-contributed capture with explicit permission.

Do not distribute:

- vendor application binaries;
- extracted vendor artwork;
- copied proprietary layouts;
- credentials;
- serial numbers;
- personal paths;
- unrelated MIDI traffic;
- fixtures whose redistribution rights are unknown.

For every decoder:

- test minimum and maximum valid lengths;
- reject truncated input;
- reject invalid checksum or framing;
- preserve unknown data when lossless behavior requires it;
- avoid panics on arbitrary bytes;
- include property or fuzz testing when the parser handles untrusted input.

For every encoder:

- test exact expected bytes independently;
- test bounds and unsupported values;
- test firmware-specific differences;
- test that an unsupported mapping cannot silently fall back to a nearby firmware profile;
- test that read-only sessions cannot produce write commands.

## Hardware-Write Safety

Every write-capable behavior must prove:

- capability validation occurs before encoding;
- the exact device and firmware profile is selected;
- invalid parameters are rejected locally;
- write ordering is deterministic;
- timeouts and retries are bounded;
- stale acknowledgements cannot complete a newer command;
- partial completion is represented truthfully;
- cancellation leaves the session in a known state;
- acknowledgement or read-back is used where supported;
- the undo journal records the confirmed previous state;
- unknown firmware does not silently inherit writable mappings;
- AI-generated plans pass through the same validator as manual edits.

Do not claim a batch is atomic when the hardware protocol cannot make it atomic.

## Existing Behavior, Bugs, and Refactors

For a bug:

1. Reproduce the defect with a focused failing regression test.
2. Observe and record the RED.
3. Implement the smallest fix.
4. Observe GREEN.
5. Run adjacent and regression suites.

For existing untested behavior:

1. Add characterization tests.
2. Run them against the current behavior.
3. Write a separate failing test for the intended change.
4. Perform the normal RED-GREEN-REFACTOR cycle.

For a pure refactor:

1. Establish characterization tests and invariants.
2. Record the pre-refactor green baseline.
3. Refactor without intentionally changing observable behavior.
4. Rerun focused and full affected suites.

Do not label a behavior-changing rewrite as a refactor.

## Accessibility Is Production Behavior

Accessibility is not a final audit or documentation claim.

Every visual editing workflow must have a semantic and nonvisual contract.

Tests must cover, where applicable:

- accessible name;
- role;
- current value;
- units;
- minimum and maximum;
- enabled or disabled state;
- selected state;
- available actions;
- focus order;
- announcement after mutation;
- error announcement;
- routing description;
- connection creation and removal without drag gestures;
- operation with large text;
- operation without color;
- operation with reduced motion;
- keyboard or switch navigation;
- screen-reader traversal.

The routing canvas cannot be the only representation of a preset graph.

The structured routing view must expose:

- inputs and outputs;
- block instances;
- rows and columns;
- incoming and outgoing connections;
- split and merge points;
- bypass and channel states;
- connection actions;
- validation errors.

Flutter semantics tests may establish `SEMANTICS_VERIFIED`.

Real VoiceOver or TalkBack testing is required for `PLATFORM_DEVICE_VERIFIED`.

A visually working screen is incomplete when its corresponding nonvisual workflow is absent.

## Visual Asset Policy

Production decorative and control artwork must follow the approved Topology asset policy:

- approved PNG assets for icons, switches, buttons, sockets, panels, tabs, halos, and decorative surfaces;
- procedurally rendered knobs are permitted;
- dynamically rendered functional graphics are permitted for routing cables, graphs, meters, curves, waveforms, selection regions, focus indicators, and other live data;
- no SVG production assets;
- no procedural replacement for an absent approved decorative asset;
- no copied vendor or competitor assets;
- no placeholder art may be counted as feature completion.

A work item dependent on an unavailable approved production asset may:

- implement and test its semantic or state logic independently;
- use a clearly test-only fixture inside test code;
- remain blocked for visual completion.

It may not ship a fabricated production asset and report the visual feature as finished.

## AI Mutation Safety

AI providers never receive raw transport or protocol-send capabilities.

The only accepted model output is a schema-validated, provider-independent mutation plan.

Tests must prove:

- malformed model output is rejected;
- unknown operation types are rejected;
- unsupported block or parameter references are rejected;
- out-of-range values are rejected;
- unavailable firmware capabilities are rejected;
- routing cycles or invalid graph structures are rejected;
- writes cannot bypass the normal command engine;
- destructive or multi-parameter changes require the configured preview and approval path;
- provider keys are not logged;
- serial numbers and unrelated library data are excluded;
- cost and provider restrictions are enforced;
- cancellation produces no hardware writes;
- a provider timeout produces no partial mutation;
- AI and manual edits use the same deterministic validation and execution path.

Default tests must not depend on a live AI provider or network.

Use deterministic fake-provider responses for contract and failure-path testing.

Real-provider tests must be explicitly gated, named, and excluded from clean-clone CI.

A fake provider does not prove a real provider integration.

## Test Integrity

Reject tests that:

- duplicate the production implementation;
- merely assert that code ran;
- inspect private implementation details without behavioral value;
- pass before and after the intended production change;
- rely accidentally on test order;
- rely on uncontrolled clocks, randomness, network, or filesystem state;
- use broad tolerances without a requirement;
- update a golden automatically;
- silently skip when required fixtures are missing;
- treat a missing physical device as a passing hardware test;
- test only a fake and claim real integration;
- weaken a threshold to create GREEN;
- ignore a target platform without an explicit product decision;
- accept any nonempty output instead of the specified behavior.

Formatting and lint findings in test code are refactor-phase failures, not behavior RED evidence.

Fix test form without altering already-green production behavior, then rerun the focused test and complete affected sweep.

Do not:

- weaken acceptance criteria;
- update a golden;
- skip a platform;
- mark a flaky test ignored;
- add a waiver;
- broaden a tolerance;
- replace an independent fixture with encoder-generated output;

merely to turn RED into GREEN.

Such changes require a separate reviewed QA or product decision.

## Subagent Work Isolation

The parent orchestrator must assign only executable leaf work items.

Every implementation subagent receives:

- one or a very small set of tightly related behaviors;
- immutable requirement IDs;
- dependencies;
- read scope;
- exclusive write scope;
- forbidden shared files;
- exact test commands;
- expected RED;
- minimum GREEN behavior;
- required sweeps;
- evidence destination;
- completion and block conditions.

Do not assign overlapping write scopes to concurrent agents.

Use isolated branches or worktrees.

Shared ownership files such as root manifests, lockfiles, generated registries, release manifests, and global requirement matrices are owned by the integration agent unless a work item explicitly grants access.

A subagent may propose a dependency or shared-manifest change in its handoff without editing the shared file.

Roles:

- Explorer: read-only discovery and dependency mapping.
- Fixture researcher: read-only provenance and test-vector preparation.
- Implementer: executes one strict TDD packet.
- Reviewer: independently audits behavior, test quality, evidence, and scope.
- Integration agent: lands approved patches and reruns integration sweeps.
- Hardware verifier: performs the declared physical-device matrix.
- Release verifier: performs clean-clone and distribution checks.

The implementer may not self-approve its evidence.

The parent waits for all agents in a declared parallel group, consolidates their results, resolves conflicts, and only then advances dependent work.

## Required Evidence Layout

Store evidence under:

```text
.tdd/evidence/<work-item-id>/
```

Required files:

```text
work-item.yaml
environment.txt
red-command.txt
red.log
red-exit-status.txt
green-command.txt
green.log
green-exit-status.txt
sweep-commands.txt
sweep.log
sweep-exit-statuses.txt
files-changed.txt
review.md
handoff.md
```

When applicable, also include:

```text
fixture-provenance.yaml
simulator-transcript.log
platform-matrix.yaml
hardware-matrix.yaml
accessibility-results.md
benchmark-results.json
screenshots/
sanitized-captures/
```

Do not edit evidence logs after the fact to make them cleaner.

Secrets, serial numbers, personal paths, and unrelated traffic must be redacted without altering the behavioral evidence.

## Work-Item Statuses

Use only:

- `PLANNED`
- `READY`
- `BLOCKED_CONTRACT`
- `BLOCKED_DEPENDENCY`
- `BLOCKED_ENVIRONMENT`
- `BLOCKED_FIXTURE`
- `BLOCKED_HARDWARE`
- `IN_PROGRESS`
- `RED_OBSERVED`
- `GREEN_OBSERVED`
- `REVIEW_FAILED`
- `REVIEW_APPROVED`
- `INTEGRATION_FAILED`
- `INTEGRATED`
- `VERIFIED`

Production source existing in a branch is not `INTEGRATED`.

A skipped test is not `VERIFIED`.

## Close the Work Item

A work item may close only when:

- the intended RED was observed and recorded;
- the focused GREEN was observed and recorded;
- required adjacent and matrix suites passed;
- test quality was independently reviewed;
- changed files remained within authorized scope or deviations were reviewed;
- no relevant warning or failure remains unexplained;
- fixture provenance is complete;
- accessibility evidence is present where applicable;
- the requirement matrix is updated;
- the integration agent reran the required tests after landing;
- the available verification label is accurate;
- unavailable higher-level claims are explicitly listed;
- the next executable behavior or blocker is identified.

Report `BLOCKED` honestly when an executable cycle cannot be completed.

Never report partial, unrun source code as implemented.
