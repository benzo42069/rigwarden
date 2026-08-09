# Initial Threat Model

## 1. Assets to protect

- User’s physical modeler and its stored presets/system state.
- User preset/cab/library files.
- Provider API keys.
- Device serials and personal metadata.
- Integrity of device/firmware packs.
- Integrity of app updates and releases.
- Community fixture provenance.
- Availability of the editor during performance.
- User trust in compatibility labels.

## 2. Trust boundaries

1. Untrusted preset/SysEx/cab/import files.
2. Untrusted device responses and malformed byte streams.
3. USB/BLE/network transport.
4. Community device/firmware packs.
5. AI provider output.
6. External AI/network provider.
7. Local network bridge.
8. Crash/telemetry vendor.
9. OS document providers.
10. Build/dependency supply chain.

## 3. Major threats and controls

### Malformed protocol/file input

Threats:

- parser panic;
- memory exhaustion;
- integer overflow;
- unbounded allocation;
- path traversal;
- malicious metadata;
- infinite loops.

Controls:

- bounded lengths;
- checked arithmetic;
- parser fuzzing;
- timeout/budget;
- no path trust from imported metadata;
- explicit unknown-field handling;
- reject before mutation.

### Wrong device/firmware mapping

Threats:

- writing an invalid parameter;
- corrupting preset state;
- silent nearby-version fallback.

Controls:

- exact identity/profile resolution;
- unknown firmware read-only;
- typed capabilities;
- fixture and hardware evidence;
- explicit confirmation for risky transfers;
- rollback journal based on confirmed prior state.

### Stale or ambiguous responses

Threats:

- acknowledgement from an old request completes a new mutation;
- reconnect session confusion.

Controls:

- connection generation IDs;
- request correlation;
- bounded pending queue;
- session reset on reconnect;
- stale-frame tests.

### Partial batch writes

Threats:

- UI reports success while hardware contains half the change.

Controls:

- operation-by-operation status;
- deterministic sequence;
- stop/continue policy;
- semantic reconciliation;
- journal partial state;
- no false atomicity.

### Malicious profile packs

Threats:

- enabling unsafe writes;
- resource exhaustion;
- spoofed device match;
- malicious strings/assets.

Controls:

- signed normal packs;
- declarative schema only;
- strict size/complexity limits;
- exact match rules;
- signature/trust display;
- sandboxed parsing;
- unsigned packs only in developer mode.

### AI output

Threats:

- hallucinated IDs;
- unsafe routing;
- destructive mass changes;
- hidden prompt injection from preset names/notes;
- cost abuse;
- data leakage.

Controls:

- AI output treated as untrusted data;
- strict schema;
- no raw transport tool;
- stable ID resolution;
- deterministic validation;
- preview/approval;
- prompt-field escaping/separation;
- cost and provider caps;
- data preview/minimization;
- cancellation before writes.

### Secrets

Threats:

- API key in logs/database/export/crash report.

Controls:

- platform secret store;
- redaction tests;
- no key in ordinary state serialization;
- inspectable deletion;
- crash-report scrubbing.

### Network bridge

Threats:

- unauthorized LAN control;
- replay;
- downgrade;
- exposed public port.

Controls:

- explicit pairing;
- mutual authentication;
- encryption;
- protocol/version negotiation;
- replay protection;
- LAN-only default;
- rate limiting;
- no automatic port forwarding;
- device ownership indication.

### Telemetry

Threats:

- accidental preset, key, serial, or capture upload.

Controls:

- opt-in;
- schema allowlist rather than blacklist;
- local preview where practical;
- AI data excluded;
- no raw logs by default;
- deletion controls;
- F-Droid telemetry-free flavor.

### Supply chain

Threats:

- compromised dependency or release artifact.

Controls:

- lockfiles;
- dependency audit;
- minimal dependencies;
- signed releases;
- reproducible-build work;
- SBOM;
- isolated build credentials;
- review of generated code;
- no arbitrary remote executable packs.

## 4. Performance safety

Topology cannot guarantee that every user action is harmless to speakers, hearing, or stage systems. It can reduce risk:

- do not unexpectedly alter output levels;
- preview large multi-parameter operations;
- flag high-risk level changes;
- preserve exact user intent for direct edits;
- avoid automatic preset loads/writes;
- rate limit;
- offer Stage Mode control lock;
- document that users should monitor levels.

## 5. Security testing

Before beta:

- parser fuzzing;
- profile-pack fuzzing;
- hostile mutation-plan suite;
- secret-redaction suite;
- network bridge authentication tests;
- reconnect/stale-response tests;
- dependency audit;
- mobile permission review;
- release-signing review;
- security-focused independent subagent review;
- public `SECURITY.md`.
