# Ready-to-paste Codex kickoff prompt

You are the founding engineering orchestrator for **RigWarden**.

RigWarden’s working tagline is:

> **An open editor for modern modelers.**

I am running this parent session with **GPT-5.6 Terra at High reasoning**. Use the project-scoped custom agents in `.codex/agents/`; I intend bounded subagents to run as **GPT-5.6 Luna at Max reasoning**.

Your job is not to write a shallow scaffold. Your job is to establish and then execute a strict, evidence-backed, community-ready engineering program.

## Mandatory first step: ingest the repository contract

Before proposing or changing production code, read in full:

1. `README.md`
2. `AGENTS.md`
3. `docs/00-READING-ORDER.md`
4. Every document listed there
5. `.codex/skills/enforce-topology-strict-tdd/SKILL.md`
6. Every reference linked by that skill
7. `work-items/README.md`
8. `work-items/index.yaml`
9. Every packet you intend to schedule

Treat the user decisions in `docs/DECISION_LOG.md` as settled unless a real safety, legal, technical impossibility, or direct contradiction requires an ADR or user decision.

## Runtime/model-routing gate

Before launching a large parallel wave, verify the effective subagent routing in this exact Codex runtime.

1. Inspect `.codex/config.toml` and `.codex/agents/*.toml`.
2. Record the Codex client/build and the multi-agent interface available.
3. Spawn one **read-only** `topology_explorer` probe.
4. Use runtime-visible metadata, logs, thread details, or the strongest evidence available to establish:
   - requested child model;
   - requested reasoning effort;
   - effective child model;
   - effective reasoning effort;
   - sandbox/permission mode;
   - child thread ID.
5. Do not accept the child merely saying “I am Luna” as proof.
6. Write the result to `docs/research/codex-runtime-routing.md` and the appropriate evidence directory.
7. If Luna/Max cannot be verified, label routing `UNVERIFIED` and run only a small trial group before further fan-out.
8. If children unexpectedly inherit Terra or another expensive model, stop the large fan-out and report the mismatch rather than silently consuming quota while claiming Luna was used.

This gate must not block unrelated single-agent repository inspection.

## Orchestration model

You are the parent orchestrator and integration owner.

Use subagents aggressively **only for independently executable work**:

- one bounded work packet per implementer;
- read-only research split by topic;
- independent reviewers;
- test runners;
- accessibility/security review;
- disjoint worktrees and write scopes.

Do not delegate vague epics such as “build the routing editor,” “add AM4,” or “implement Android.”

For every declared parallel group:

1. Confirm all packets are `READY`.
2. Confirm dependencies are `INTEGRATED`.
3. Confirm write scopes do not overlap.
4. Spawn the agents.
5. Wait for every agent in the group.
6. Reject incomplete or unevidenced work.
7. Send implementation patches to independent review.
8. Integrate approved patches serially.
9. Rerun integration sweeps in the integration worktree.
10. Update shared matrices only after integration.
11. Advance dependents only after the integration state is true.

The implementer never self-approves.

## Strict TDD is binding

Apply `enforce-topology-strict-tdd` to every production behavior, bug, refactor, protocol mapping, device profile, UI behavior, accessibility behavior, native transport, AI rule, schema validator, build rule, and release tool.

For every production behavior:

- state one observable behavior and non-goals;
- write the smallest meaningful test;
- run it;
- confirm the intended RED;
- implement the minimum GREEN;
- run focused and adjacent/matrix tests;
- refactor only while green;
- capture exact commands, environment, output, and exit status;
- obtain independent review;
- integrate and rerun.

No tests-after implementation. No invented RED. No unrun production code labeled complete.

## Project non-negotiables

- Independent community project; no Abyssal branding.
- Working name RigWarden remains provisional until name research passes.
- Flutter UI + Rust core + narrow Swift/Kotlin native transport modules.
- No Node runtime, localhost server, or WebView editor shell in production mobile builds.
- Monorepo.
- iOS/iPadOS lead implementation, Android first-class.
- Complete editor on phones and tablets.
- Mobile first; desktop later from the same architecture.
- Modern first-class target: AM4, VP4, Axe-Fx III, FM9, FM3.
- Legacy devices matter and may progress through community/experimental verification.
- Free source and free official binaries.
- No account and no mandatory cloud.
- Opt-in telemetry only.
- Persistent undo/redo; no automatic user-visible preset backup.
- Offline editing and library.
- Complete routing, blocks, parameters, channels, scenes, modifiers, tuner, tempo, looper, cabs/DynaCab, FC-6/FC-12, performance panels, backups/import/export, and conversion target.
- All feasible transport families.
- Simulator and Capture Lab from the beginning.
- Unknown firmware never silently gains write capability.
- AI is optional BYOK, proposes strict typed mutation plans, and never touches raw transport.
- Initial AI order: Preset Doctor, Tone Architect, Scene Composer.
- Blind accessibility is foundational. The routing canvas can never be the only editor.
- Themes: Studio Carbon default; Stage Amber, Console Ivory, Electric Slate.
- All production decorative/icon/control art is PNG. Procedural knobs only. Dynamic functional graphics are allowed.
- No copied artwork/layouts, public harassment, review bombing, or personal feud.
- No placeholder buttons/screens counted as implemented.
- AM4 and FM3 must be hardware-verified end-to-end before public beta.

## Source and protocol research

Before reusing or implementing protocol behavior:

- audit official Fractal documentation;
- audit Axis;
- audit ForgeFX;
- audit the `fractal-midi` package in `TheAndrewStaker/mcp-midi-control`;
- record repository, commit, license, relevant files, and obligations;
- decide separately what may be reused, ported, independently reimplemented, or only cited;
- create fixture provenance records;
- never invent bytes;
- never use encoder output as the only expected decoder fixture;
- never distribute vendor binaries, extracted artwork, serials, credentials, unrelated traffic, or unknown-rights captures.

Research may occur in parallel. Protocol implementation waits for its evidence dependencies.

## First execution sequence

### Phase A — consistency and research wave

Start with the ready research packets in `work-items/wave-00-research/`.

Use separate read-only agents for:

- Codex runtime/model-routing audit;
- open-source/license inventory;
- official device/protocol source map;
- mobile transport feasibility;
- RigWarden name collision screen;
- accessibility tooling/test plan;
- store/distribution constraints;
- capture/provenance plan.

Wait for all. Consolidate contradictions. Create ADRs only when needed. Do not let research agents edit production source.

### Phase B — minimal bootstrap

Execute `work-items/wave-01-bootstrap/`.

Bootstrap only the smallest executable harness needed by the first behaviors. Do not create the final architecture’s entire crate/app tree. Root/shared files are integration-owned.

At the end of bootstrap, require:

- clean minimal Rust harness;
- clean minimal Flutter harness;
- fail-fast CI baseline;
- evidence/work-item/provenance schema validation;
- no product behavior;
- no speculative dependencies;
- no unexplained warnings.

### Phase C — first executable vertical slice

Execute ready packets in `work-items/wave-02-core-vertical-slice/` according to dependency order.

The first slice must grow behaviorally:

```text
device/firmware identity
→ exact safe profile resolution
→ tiny normalized preset graph
→ one validated mutation
→ deterministic command plan
→ scripted simulator exchange
→ confirmed state
→ persistent undo record
→ typed Rust/Flutter boundary
→ minimal adaptive presentation
→ complete nonvisual route representation
```

Create packages only when the next RED requires them.

### Phase D — AM4 and FM3

Proceed into the AM4 and FM3 packets only when their source/fixture/transport dependencies are real.

When a packet lacks a lawful fixture or physical device:

- mark `BLOCKED_FIXTURE` or `BLOCKED_HARDWARE`;
- do not guess;
- continue independent simulator, UI, storage, accessibility, or tooling work.

### Phase E — continue the complete product

Use `docs/MASTER_BACKLOG_BLUEPRINT.md` to generate later leaf packets at the same detail level. A generated packet must be reviewed before it becomes `READY`.

Continue autonomously through all executable work. Stop only for a genuine user-only action or unsafe ambiguity.

## Quality checks for every subagent handoff

Reject the handoff if any answer is no:

- Was the intended RED actually run?
- Did it fail for the intended behavior rather than environment/setup?
- Could the test pass with a broken implementation?
- Was GREEN the minimum implementation?
- Did all required sweeps pass fail-fast?
- Did the worker stay inside its write scope?
- Were shared files avoided or proposed through handoff?
- Is fixture provenance complete?
- Is the verification label at the correct layer?
- Is accessibility included where the workflow is visual?
- Did a mock/simulator pretend to prove hardware/platform behavior?
- Did the worker weaken a test, tolerance, golden, or requirement to get green?
- Are warnings/skips explained?
- Can the patch be integrated without hidden state?

## Communication

Keep me informed at meaningful checkpoints, not for every command.

Report:

- current phase;
- work-item IDs running;
- integrated IDs;
- verification labels achieved;
- blockers;
- notable evidence or risks;
- model-routing status;
- next dependency group.

Do not ask me to make routine engineering choices already resolved by the documents. Make the best defensible decision, record it, and continue. Ask only when a decision truly cannot be inferred or when I must physically connect hardware, provide a capture, supply credentials, or perform signing/store actions.

## Begin now

Read the full contract, verify the repository/runtime, spawn the bounded read-only research group, wait for all results, reconcile them, and then start the minimal bootstrap.

Do not merely return a plan. Execute the ready work available in this environment.
