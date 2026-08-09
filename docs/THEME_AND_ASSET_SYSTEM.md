# Theme and Asset System

## 1. Approved themes

### Studio Carbon — default

Professional near-black and graphite studio surfaces, crisp typography, restrained metal detail, and a user-selectable accent. Designed for extended editing.

### Stage Amber

Black, amber, and warm-white contrast. Large controls, reduced detail, strong focus states, and stage-distance readability.

### Console Ivory

Warm cream panels, charcoal controls, restrained vintage-console influence, and a usable light-theme experience.

### Electric Slate

Dark blue-gray surfaces with cool cyan and restrained violet accents. Modern without neon overload.

## 2. Asset rule

All production decorative, icon, and control artwork is PNG.

PNG assets include:

- navigation icons;
- toolbar icons;
- switches;
- buttons;
- sockets/jacks;
- panels and faceplates;
- tabs;
- halos;
- decorative textures;
- static state illustrations;
- non-knob control caps.

Procedurally rendered knobs are allowed.

Dynamically rendered functional graphics are allowed where the visual must reflect live data:

- routing cables;
- connection handles;
- meters;
- analyzer graphs;
- modifier curves;
- EQ curves;
- waveforms;
- selection/focus regions;
- drag previews;
- tuner motion;
- grid guides.

No production SVG assets.

## 3. Accessibility and PNGs

Every asset-backed control must have independent semantic labels/state. Essential text cannot exist only inside an image.

State must not rely only on:

- color;
- glow/halo;
- texture;
- image difference;
- cable position.

## 4. Asset variants

The design system should define:

- canonical logical size;
- pixel-density variants;
- safe insets;
- stretchable regions when applicable;
- light/dark/theme variants;
- enabled/disabled/pressed/selected/error/pending states;
- contrast requirements;
- asset license/provenance;
- checksum and build manifest.

Prefer lossless PNG for controls requiring sharp alpha and exact edges. Optimize in the build pipeline without degrading visible quality.

## 5. Placeholder policy

Test-only placeholder fixtures may exist inside tests.

Production code may not ship procedurally fabricated decorative replacements merely because final art is missing. The semantic/state behavior may be completed and tested while visual completion remains blocked.

A screen containing placeholder art is not visually complete and cannot satisfy the beta gate.

## 6. Knobs

Procedural knobs must support:

- touch drag;
- vertical/horizontal configurable gesture;
- fine adjustment;
- precise numeric entry;
- reset/default action;
- keyboard/switch actions;
- screen-reader adjustable semantics;
- bipolar and unipolar ranges;
- stepped/enum behavior where represented;
- pending/confirmed/error/read-only state;
- reduced motion.

The rendered knob is a view of typed parameter metadata, never the source of truth.

## 7. Theme-pack architecture

Themes should be data/assets, not executable plugins.

A theme pack includes:

- version;
- asset manifest;
- color/token manifest;
- typography tokens;
- spacing/radius/elevation tokens;
- compatibility version;
- license/provenance;
- checksums.

The app ships the four approved themes. Community theme loading is a later security/product decision and must never allow executable code.
