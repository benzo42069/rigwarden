# Accessibility Contract

## 1. Foundational promise

Blind and low-vision access is a core product requirement, not a final audit. A workflow is incomplete when it works only through visual spatial interpretation.

The routing canvas is one representation of the preset. It is never the sole representation.

## 2. Required nonvisual information architecture

The app must provide direct navigation to:

- active device/session;
- presets;
- inputs and outputs;
- routing paths;
- block instances;
- block categories and models;
- parameters;
- channels;
- scenes;
- controllers/modifiers;
- validation problems;
- performance controls;
- library versions and diffs.

A user should be able to search for “Amp 1 Input Drive,” focus it, hear the current value and units, change it precisely, and confirm whether the device accepted the change without traversing a canvas.

## 3. Nonvisual routing editor

The structured routing view must expose:

- source inputs;
- destination outputs;
- block stable ID, type, instance, row, and column;
- incoming and outgoing connections;
- serial order;
- split count and branch descriptions;
- merge points;
- bypass/channel state;
- invalid or disconnected state;
- add/remove/move/connect actions;
- proposed destination choices filtered by capability;
- confirmation and error announcements.

Example concise announcement:

> Drive 1. Column 3. Receives Input 1. Splits to Amp 1 and Amp 2. Bypassed: no. Channel A.

Example connection:

> Connection from Drive 1 output to Amp 2 input. Actions: remove, inspect source, inspect destination.

## 4. Control semantics

Every interactive control exposes:

- accessible name;
- role;
- block/scene context;
- current value;
- unit;
- minimum/maximum;
- step and available precision modes;
- selected/toggled state;
- pending/confirmed/error/read-only state;
- available actions;
- help or description where needed.

Do not bake essential labels into PNG assets without an accessible semantic equivalent.

## 5. Mutation feedback

After an edit, announce the correct state:

- “Input Drive 4.50, pending.”
- “Input Drive confirmed at 4.50.”
- “Change failed: device disconnected.”
- “Three of five changes completed. Review partial result.”
- “Read-only: firmware profile not verified.”

Do not announce optimistic state as confirmed hardware state.

## 6. Navigation and focus

Requirements:

- deterministic focus order;
- no focus traps;
- focus restored after dialogs and orientation changes;
- stable focus after live device updates;
- headings/landmarks for major regions;
- direct search and jump;
- adjustable concise versus verbose announcements;
- keyboard, switch-control, and external-input operation where platform supports it;
- logical traversal in both visual and nonvisual editors.

## 7. Visual accessibility

- scalable text without clipping;
- responsive reflow;
- high contrast;
- no color-only status;
- visible focus;
- reduced motion;
- large touch targets;
- stage-readable mode;
- no essential information only in a halo, cable color, meter color, or spatial position.

## 8. Tuner accessibility

Tuner state should provide:

- note name;
- octave where useful;
- cents sharp/flat;
- in-tune status;
- signal/no-signal;
- optional adjustable spoken cadence;
- optional haptic/audio cues;
- complete textual/semantic equivalent.

Avoid flooding screen readers with unthrottled updates. Test debouncing and user control.

## 9. Testing ladder

1. Pure semantic formatter/unit tests.
2. Flutter widget/semantics tests.
3. Keyboard/focus integration tests.
4. iOS simulator accessibility inspection where meaningful.
5. Android emulator accessibility inspection where meaningful.
6. Physical iPhone/iPad with VoiceOver.
7. Physical Android device with TalkBack.
8. Blind-user task testing on real modeler hardware.

A semantics-tree test is not equivalent to a blind user completing the workflow.

## 10. Beta task suite

At minimum, a blind tester must be able to:

1. Connect or select a device.
2. Understand verification/read-only status.
3. Browse and load a preset.
4. Describe the complete signal path.
5. Locate a block.
6. Change a parameter precisely.
7. Confirm device acceptance.
8. Change scene and channel.
9. Add/remove a supported connection through nonvisual controls.
10. Undo the change.
11. Create/use a performance panel.
12. Use tuner and tap tempo.
13. Save/export an offline version.
14. Recover from a disconnect.

Failures are release blockers for the affected advertised workflow.

## 11. Contributor policy

- Accessibility issues carry product-bug priority.
- PRs that add visual controls must add semantics in the same work item.
- No “accessibility later” placeholder.
- Blind contributors/testers should have a direct, respectful feedback route.
- Screenshots do not substitute for semantics evidence.
