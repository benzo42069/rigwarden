# Architecture Contract

## 1. Architectural goals

Topology needs one deterministic editor engine, small platform-specific transport modules, a touch-first adaptive UI, and a profile system that evolves without app-store releases.

The architecture must optimize for:

- correctness around expensive physical hardware;
- clear verification boundaries;
- offline use;
- lossless preservation of unknown data;
- blind-accessible alternate workflows;
- community device-profile contributions;
- simulator-first development;
- safe AI planning;
- gradual support for additional vendors;
- mobile-first distribution without embedding a server runtime.

## 2. Required high-level architecture

```text
┌─────────────────────────────────────────────────────────────┐
│ Flutter presentation                                       │
│                                                             │
│ Adaptive navigation · routing · parameter editors           │
│ library · stage mode · accessibility · themes               │
└──────────────────────────┬──────────────────────────────────┘
                           │ typed application API/events
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ Rust core                                                   │
│                                                             │
│ identities · profiles · preset document · routing graph     │
│ validators · command engine · undo journal · file codecs    │
│ simulator · replay · storage · AI mutation validation       │
└──────────────────────────┬──────────────────────────────────┘
                           │ framed bytes / transport commands
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ Native transport adapters                                   │
│                                                             │
│ Swift/CoreMIDI · Kotlin Android MIDI/USB/BLE                 │
│ desktop MIDI/serial · network bridge                        │
└─────────────────────────────────────────────────────────────┘
```

Production mobile builds must not contain:

- a Node runtime;
- a localhost HTTP service;
- a WebView-based editor shell;
- protocol state duplicated independently in Dart and Rust;
- an AI agent with direct transport access.

## 3. Monorepo target

Crates/apps should appear only when an executable behavior requires them. This is the target shape, not permission to scaffold everything on day one:

```text
/apps
  /mobile_flutter
  /desktop_flutter                 # later

/crates
  /topology_domain
  /topology_device_registry
  /topology_preset
  /topology_routing
  /topology_command_engine
  /topology_undo
  /topology_protocol_common
  /topology_protocol_am4
  /topology_protocol_gen1
  /topology_protocol_gen2
  /topology_protocol_gen3
  /topology_simulator
  /topology_library
  /topology_ai
  /topology_bridge
  /topology_devtools

/native
  /ios
  /android
  /desktop

/device-packs
  /am4
  /axe-fx-iii
  /fm3
  /fm9
  /vp4
  /legacy

/theme-packs
  /studio-carbon
  /stage-amber
  /console-ivory
  /electric-slate

/tools
  /capture-lab
  /profile-builder
  /compatibility-report
  /release-validator

/docs
/tests
/.tdd
```

Do not create empty crates solely to match this diagram.

## 4. Core boundaries

### 4.1 Transport

Transport moves opaque framed bytes and reports endpoint/lifecycle state. It knows:

- endpoint identity;
- open/close;
- bytes in/out;
- packet boundaries if the platform exposes them;
- cancellation;
- timeout;
- fragmentation;
- hotplug;
- connection errors;
- transport capability.

Transport does not know that a byte sequence means “Amp 1 Input Drive.”

### 4.2 Protocol family

A protocol family knows:

- framing;
- message types;
- checksums;
- payload encoding;
- request/response correlation;
- version/family differences;
- safe parse/encode errors.

It does not choose UI widgets or own platform connections.

### 4.3 Device profile

A declarative device/firmware pack knows:

- identity matching;
- firmware applicability;
- supported transports and feature restrictions;
- routing geometry and constraints;
- block inventory;
- parameter metadata;
- scene/channel/controller capabilities;
- model catalogs;
- display conversions;
- known protocol mappings;
- provenance and verification status.

Unknown firmware never inherits write capability by approximation.

### 4.4 Normalized preset document

The normalized document represents editable intent without pretending all vendors/devices share identical semantics.

It must preserve:

- device/firmware identity;
- original raw payload where lawful and appropriate;
- known typed data;
- opaque unknown segments;
- routing graph;
- block instances and stable IDs;
- parameters and units;
- channels/scenes/controllers;
- metadata;
- versioning;
- loss/capability warnings.

A serializer must refuse a destructive “lossless” claim if required unknown data cannot be preserved.

### 4.5 Command engine

The command engine converts validated mutations into deterministic operations.

Responsibilities:

- capability checks;
- profile selection validation;
- dependency ordering;
- request correlation;
- write throttling;
- acknowledgement/read-back policy;
- timeout/retry;
- partial completion;
- cancellation;
- state reconciliation;
- journal integration;
- sanitized evidence.

The command engine does not believe the optimistic UI until confirmation policy is satisfied.

### 4.6 Undo journal

The journal records semantic mutations and the confirmed prior state.

Properties:

- persistent;
- append-oriented;
- crash recoverable;
- branch-aware;
- able to represent partial application;
- able to explain a proposed undo;
- independent from user-visible preset backup files.

### 4.7 Simulator and replay

The simulator is a deterministic peer implementing explicit device/profile scenarios.

It supports:

- scripted and stateful modes;
- captured exchanges with provenance;
- latency, fragmentation, drop, stale response, reset, and disconnect injection;
- explicit simulator identity;
- E2E assertions.

Simulator behavior must not be used as physical-device evidence.

### 4.8 Flutter presentation

Flutter owns:

- adaptive layout;
- navigation;
- input gestures;
- visual rendering;
- semantic accessibility tree;
- localization-ready strings;
- theme assets;
- state presentation.

Flutter does not independently duplicate protocol mappings, routing rules, parameter constraints, or AI safety logic.

### 4.9 Native platform adapters

Swift/Kotlin modules remain narrow:

- enumerate and open platform endpoints;
- bridge bytes/events to Rust;
- manage permissions and app lifecycle;
- expose OS-specific capability;
- use platform secret stores;
- integrate file/document providers;
- support required background behavior where permitted.

Business logic belongs in Rust unless a platform contract makes that impossible.

### 4.10 AI planner

The provider adapter returns a schema-bound mutation plan. The Rust validator owns meaning and safety. The model never emits directly executable SysEx.

## 5. State model

The application must distinguish at least:

- `OfflineDocumentState`
- `LastReadDeviceState`
- `OptimisticPendingState`
- `ConfirmedLiveState`
- `LastStoredDeviceState`
- `JournalBranchState`

UI must not collapse them into one ambiguous “current preset.”

## 6. Storage

Initial recommendation: SQLite behind a Rust storage interface, plus OS-managed file access for external files.

Required properties:

- transactional migrations;
- deterministic schema versioning;
- crash safety;
- explicit backup/export;
- no cloud dependency;
- key material stored outside ordinary database rows;
- testable in-memory and temporary-file modes;
- corruption detection and recovery guidance.

The bootstrap research may select a specific Rust SQLite stack, but it must document maintenance, mobile-build, concurrency, encryption, and migration implications.

## 7. FFI

Use generated, typed bindings. Initial candidates are `flutter_rust_bridge` for Dart/Rust and narrow platform channels/Pigeon-style APIs for Swift/Kotlin operations.

The exact dependency choice is a research decision, not an excuse to move domain logic into Dart.

FFI requirements:

- versioned API;
- explicit cancellation;
- structured errors;
- no raw pointer ownership leaking into Dart;
- no unbounded callbacks;
- lifecycle tests;
- deterministic serialization;
- performance measurements for large preset state.

## 8. Extensibility

Vendor neutrality means:

- stable domain concepts where genuinely shared;
- explicit capability variation;
- vendor-specific extensions;
- no lowest-common-denominator model;
- no assumption that every device uses a rectangular graph;
- no assumption that all parameters are normalized floats;
- no assumption that transport equals protocol.

Do not implement another vendor until the Fractal-focused vertical slices prove the abstractions.

## 9. Architecture decision process

Material changes require an ADR when they affect:

- language/framework;
- persistence;
- FFI;
- package format;
- signing/update trust;
- protocol provenance;
- public API;
- verification labels;
- accessibility strategy;
- privacy/network behavior.

An ADR must include alternatives, consequences, migration cost, and how the decision is tested.
