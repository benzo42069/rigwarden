# Topology Project Brief

## Working identity

**Topology**  
*An open editor for modern modelers.*

Topology is an independent community project. It has no connection to Abyssal Audio and must not inherit Abyssal branding, visual language, accounts, telemetry, infrastructure, or commercial goals.

The public story is constructive:

> Topology exists to give musicians a capable, transparent, free, community-maintained editor that works anywhere.

The private quality motivation may be “prove that AI-assisted software does not have to be slop,” but the repository and community must never become a campaign against a named developer or competitor.

## Mission

Build a polished, local-first, cross-platform editor and preset workstation for professional guitar modelers, beginning with current and legacy Fractal Audio hardware.

Topology must support complete editing on iPhone, iPad, and Android phones/tablets—not merely preset selection or stage control. Desktop clients should later ship from the same architecture.

The project should be complete enough that the community can immediately use it, inspect it, extend device coverage, contribute fixtures, and continue development without reverse-engineering the reverse engineering.

## Primary users

- Guitarists who want full mobile editing.
- Touring and rehearsal musicians who need stage-safe controls.
- Blind and low-vision musicians who are poorly served by graphical routing editors.
- Preset builders who want offline editing, versioning, diffing, and conversion.
- Open-source contributors without physical hardware who need a simulator and replay fixtures.
- Owners of legacy devices who can contribute testing and captures.
- Developers researching interoperable modeler protocols.
- Advanced users who want optional, bring-your-own-key AI preset tools.

## Initial product scope

First-class modern-family target:

- AM4
- VP4
- Axe-Fx III
- FM9
- FM3

Legacy community target:

- Axe-Fx II / XL / XL+
- AX8
- FX8
- architecture-ready experimental support for Standard/Ultra when lawful fixtures and hardware testers appear.

The implementation sequence is:

1. AM4 engineering bootstrap.
2. FM3 complete-grid flagship.
3. FM9 and Axe-Fx III expansion.
4. VP4.
5. Axe-Fx II family.
6. AX8.
7. FX8.
8. Standard/Ultra when supported by evidence and testers.

This order is an engineering sequence, not a permanent product hierarchy.

## Product pillars

### Complete editing

Topology ultimately includes:

- automatic detection and manual endpoint selection;
- preset browsing, rename, save, import, export, backup, and restore;
- complete routing-grid editing;
- block add/remove/move/connect;
- block type and model selection;
- all supported parameters;
- channels;
- scenes;
- bypass;
- modifiers and controllers;
- tuner;
- tempo and tap tempo;
- looper;
- user-cab and DynaCab workflows;
- FC-6 and FC-12 layout editing;
- custom performance panels;
- offline editing;
- semantic diff and version history;
- cross-device conversion.

### Touch-first without being phone-lite

Phones and tablets receive the complete editor. Layouts are adaptive rather than scaled copies.

The phone experience may emphasize a Performance Deck for stage use, but no core editing capability is intentionally withheld from phones.

### Local first

- No account for normal use.
- No mandatory cloud.
- No project-owned AI proxy.
- User files remain local unless explicitly exported.
- System file pickers may access user-selected cloud providers.
- Crash reporting is optional and opt-in.
- AI is optional, BYOK, and independently disabled.

### Truthful compatibility

Public status labels:

- Hardware verified
- Community confirmed
- Capture verified
- Simulator verified
- Experimental
- Read-only
- Unsupported

A device or feature may be architecturally complete without being hardware verified. The label must say so.

### Accessible by design

A blind user must be able to inspect and edit routing, parameters, scenes, channels, and performance controls without interpreting a visual canvas.

### Community-extensible

Device and firmware packs are declarative, versioned, provenance-aware, independently updateable, and signed for normal distribution. Developer mode may load unsigned local packs with an unmistakable warning.

### AI as a safe planner

AI features propose typed changes; deterministic local code decides whether those changes are valid and how they are encoded.

Initial priority:

1. Preset Doctor.
2. Tone Architect.
3. Scene Composer.

Reference-audio matching is deliberately later.

## Success definition

Topology succeeds when:

- it can reliably perform complete workflows rather than display mock screens;
- contributors can develop most behavior without hardware;
- unsupported firmware cannot accidentally inherit writable mappings;
- blind users can complete essential workflows;
- protocol claims are evidence-backed;
- the community can add support without editing the app core;
- the application is free, useful, and honest about what was actually verified.
