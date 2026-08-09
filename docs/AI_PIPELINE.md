# AI Preset Pipeline

## 1. Goal

AI should make expert modeler workflows approachable without becoming an unsafe remote-control layer.

Topology supports optional bring-your-own-key providers. Core editing remains complete without AI.

## 2. Provider architecture

Initial providers:

- OpenRouter BYOK.
- Generic OpenAI-compatible endpoint.
- Later LAN/local provider using the same interface.

No Topology-owned inference proxy is required.

Provider interface responsibilities:

- model discovery/config;
- structured request;
- strict schema output when available;
- cancellation;
- timeout;
- token/cost accounting;
- provider/model allowlists;
- privacy controls;
- zero-data-retention preference/requirement where supported;
- sanitized errors.

Provider adapters do not receive transport handles.

## 3. Secret handling

- Store keys in iOS Keychain, Android Keystore-backed secure storage, and desktop equivalents.
- Never store keys in logs, database exports, fixtures, crash reports, analytics, or prompt history.
- Redact provider headers.
- Provide delete/replace controls.
- Do not sync keys through project infrastructure.

## 4. Data preview and minimization

Before a request, show:

- provider and model;
- estimated request size/cost where available;
- exact preset fields included;
- whether block names, parameters, routing, scenes, notes, and user prompt are included;
- what is excluded.

Always exclude:

- hardware serial numbers;
- credentials;
- unrelated presets/library;
- raw protocol captures;
- personal filesystem paths;
- telemetry identifiers.

## 5. Mutation-plan schema

The model returns a provider-neutral plan such as:

```json
{
  "schema_version": "1.0",
  "intent": "Tighten low end and reduce fizz",
  "operations": [
    {
      "operation_id": "op-1",
      "type": "set_parameter",
      "target": {
        "block_id": "amp-1",
        "channel": "A",
        "parameter_id": "input_drive"
      },
      "value": {
        "kind": "display_number",
        "number": 4.5
      },
      "reason": "Reduce saturation while preserving attack"
    }
  ],
  "assumptions": [],
  "warnings": []
}
```

The schema must use stable typed IDs from the local profile. Free-form names may be resolved only through an explicit ambiguity flow.

## 6. Validation pipeline

```text
user intent
→ provider request
→ strict JSON parse
→ schema validation
→ resolve stable IDs
→ device/firmware capability validation
→ type/range/enum validation
→ graph validation
→ CPU/resource preflight where available
→ semantic diff
→ approval policy
→ deterministic command plan
→ acknowledgement/read-back
→ journal
```

Any failure before execution produces zero hardware writes.

## 7. Approval policy

Default:

- exact single safe change: configurable fast path after validation;
- multiple parameters: semantic preview;
- graph/scene/controller/cab/FC changes: semantic preview;
- destructive/import/conversion operations: semantic preview plus explicit confirmation;
- unknown firmware/read-only: no writes;
- partial execution: stop and show the confirmed result before continuing.

## 8. Initial features

### Preset Doctor — first

Find and explain:

- disconnected or unreachable blocks;
- invalid/unsupported routes;
- suspicious scene/channel mismatches;
- abrupt level changes where data permits;
- redundant or no-op processing;
- unsupported parameters after firmware/device conversion;
- likely reason a scene is silent;
- unsaved versus live-state confusion.

Deterministic rules should handle everything possible before asking a model. AI explains and prioritizes rather than replacing validation.

### Tone Architect — second

Generate a complete staged mutation plan from a musical intent. It must declare assumptions and device constraints. It may choose blocks/models only from the active profile.

### Scene Composer — third

Derive scene variations from an existing preset while preserving shared routing and reporting which state is scene-specific versus channel/global.

### Later

- Preset Explainer.
- Troubleshooter.
- Performance Panel Generator.
- Cross-Device Translator.
- Reference-audio matching only after separate DSP/measurement work.

## 9. Testing

Default tests use deterministic fake providers.

Test:

- malformed JSON;
- extra/unknown fields;
- unknown operation;
- invalid block;
- ambiguous name;
- unsupported parameter;
- out-of-range value;
- invalid enum;
- prohibited route;
- unknown firmware;
- read-only session;
- provider timeout;
- cancellation;
- cost cap;
- provider restriction;
- key redaction;
- preview generation;
- no-write guarantee before approval;
- same validator path as manual edits.

Live-provider tests are explicitly gated and are not required in clean-clone CI.

A fake provider proves the local contract, not a specific external provider integration.
