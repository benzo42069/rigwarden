# Accessibility Contract for Work Items

## Applicability

A packet affects accessibility when it adds or changes:

- screen/view/dialog;
- control;
- navigation;
- status/error;
- live update;
- graph/routing behavior;
- gesture;
- image-backed state;
- stage workflow;
- tuner/meter;
- import/export flow.

The packet must list accessibility behavior in the same cycle or declare a tightly linked dependency that blocks visual completion until integrated.

## Required assertions

As applicable:

- accessible name;
- role;
- value/unit/range;
- selected/toggled/disabled/read-only;
- pending/confirmed/error;
- actions;
- focus order;
- focus restoration;
- announcement;
- large text/reflow;
- reduced motion;
- non-color state;
- keyboard/switch;
- direct navigation;
- nonvisual equivalent.

## Graph-specific

A visual graph feature is incomplete without:

- structured nodes;
- connections;
- splits/merges;
- route order;
- validation errors;
- equivalent mutation actions.

## Dynamic updates

Throttle high-frequency data such as tuner/meters. The user must control announcement frequency. Do not make the screen reader unusable.

## PNG assets

Image controls need semantic state independent of pixels. Do not rely on filename/alt text alone when the control has value/range/actions.

## Verification labels

- Unit formatter: `UNIT_VERIFIED`
- Flutter semantics: `SEMANTICS_VERIFIED`
- iOS/Android simulator integration: `PLATFORM_SIMULATOR_VERIFIED`
- Physical VoiceOver/TalkBack task: `PLATFORM_DEVICE_VERIFIED`
- Blind user + physical modeler workflow: record both platform and hardware evidence.

## Reviewer rejection examples

- visually operable only;
- one generic label for whole routing canvas;
- state only by color/halo;
- drag-only connection;
- no announcement for failed write;
- focus jumps on live refresh;
- text clips at large scale;
- semantics test but claim says VoiceOver verified.
