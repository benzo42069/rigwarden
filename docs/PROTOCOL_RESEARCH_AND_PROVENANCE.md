# Protocol Research and Provenance

## 1. Core rule

RigWarden may implement interoperability. It may not guess protocol behavior, copy proprietary assets, redistribute unknown-rights material, or present a self-consistent codec as proof of device compatibility.

Every material protocol fact must have provenance.

## 2. Research targets

Initial projects and documents to audit:

- Fractal Audio’s published third-party MIDI documentation and current device manuals.
- Axis.
- ForgeFX.
- The `fractal-midi` package inside `TheAndrewStaker/mcp-midi-control`.
- Other permissively licensed code discovered during research.
- User-owned captures generated specifically for RigWarden.
- Community captures submitted with explicit redistribution permission.

Auditing a source does not automatically authorize copying it. Record license, commit, file paths, derivation, and obligations.

## 3. Source categories

Allowed source categories:

1. Published vendor specification.
2. Public vendor manual.
3. Permissively licensed open-source implementation.
4. User-owned hardware capture.
5. Community-contributed capture with explicit permission.
6. Independently generated simulator fixture.
7. Black-box behavioral observation documented without distributing proprietary material.

Material from a proprietary application may be studied only to the extent lawful for interoperability. Do not distribute:

- application binaries;
- extracted artwork;
- copied UI layouts;
- private symbols or strings with no interoperability need;
- encrypted/circumvented protected content;
- credentials, serials, or unrelated user data;
- captures with unknown redistribution rights.

This document is an engineering policy, not jurisdiction-specific legal advice. Material legal uncertainty is a blocker requiring counsel or a narrower implementation.

## 4. Provenance sidecar

Every fixture must have a sidecar containing:

- fixture ID;
- device family/model/variant;
- firmware;
- transport;
- direction;
- command/feature;
- capture date;
- source category;
- source reference;
- source license;
- derivation notes;
- sanitization;
- redistribution permission;
- checksum;
- expected parse result;
- confidence;
- verification status;
- contributor;
- review record.

No sidecar, no merge.

## 5. Independent expectations

A codec test must avoid circular proof.

Insufficient:

```text
bytes = encode(value)
assert decode(bytes) == value
```

That checks internal symmetry, not compatibility.

Required where applicable:

- encoder tested against independently known expected bytes;
- decoder tested against independently sourced bytes and expected values;
- malformed/truncated/checksum cases;
- firmware-specific variants;
- unknown opcode behavior;
- preserved opaque fields;
- property/fuzz tests for panic freedom;
- replay against a simulator;
- physical hardware verification before a hardware claim.

Round-trip tests are still useful as additional invariants.

## 6. Capture Lab policy

Capture Lab must:

- require explicit endpoint selection;
- show when capture is active;
- record only the selected session;
- support read-only guided workflows where feasible;
- annotate user actions and expected effects;
- redact serials, usernames, paths, keys, and unrelated traffic;
- produce a manifest and checksum;
- store raw captures locally until the user explicitly exports;
- require a redistribution statement during contribution;
- never upload automatically;
- distinguish official-editor observation from direct-device sessions;
- make it easy to delete captures.

## 7. Firmware profiles

Profiles are exact, not optimistic.

- Exact firmware mapping may be writable.
- A deliberately verified compatible range may be writable only when evidence supports the range.
- Unknown firmware is read-only or unsupported.
- “Closest version” may be offered for offline inspection only, with a warning and writes disabled.
- A profile must not silently inherit newly added parameter IDs.
- Every firmware pack states source coverage and unknown areas.

## 8. Protocol monitor and danger mode

Developer Mode may expose:

- raw incoming/outgoing frame monitor;
- decoded message view;
- export of sanitized sessions;
- replay into simulator.

Arbitrary raw transmission is separate:

- second danger toggle;
- clear hardware-risk warning;
- no persistence across launches by default;
- no AI access;
- rate limiting;
- local audit trail;
- unavailable in ordinary user mode.

## 9. Third-party notices

For every reused or derived component:

- retain copyright and license notice;
- record exact upstream repository and commit;
- identify copied versus reimplemented behavior;
- list modified files;
- include required NOTICE text;
- expose third-party notices in the repository and app.

Original RigWarden code may be MIT. Apache-2.0 or other permissive dependencies remain under their licenses.

## 10. Research deliverables before protocol implementation

The first protocol wave cannot begin until it has:

- source inventory;
- license/provenance matrix;
- official documentation map;
- transport hypotheses labeled as hypotheses;
- fixture acquisition plan;
- sanitization standard;
- initial device/firmware matrix;
- a decision on what can be reused, ported, independently reimplemented, or merely cited.
