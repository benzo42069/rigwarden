# Release Plan and Definition of Done

## 1. Release progression

1. **Reproducible simulator builds**
   - clean-clone Rust/Flutter builds;
   - deterministic simulator E2E;
   - no hardware claim.

2. **Private hardware alpha**
   - direct tests on AM4 and FM3;
   - fixture capture and reconciliation;
   - crash/reconnect/undo testing;
   - blind-accessibility test preparation.

3. **GitHub APK and TestFlight alpha**
   - signed test builds;
   - explicit experimental labels;
   - contributor feedback and capture workflow.

4. **Google Play open test**
   - Android transport coverage;
   - permission and device matrix;
   - migration and update testing.

5. **Public beta**
   - complete advertised workflows;
   - AM4 and FM3 end-to-end hardware verified;
   - other profiles truthfully labeled;
   - no placeholder controls/screens counted as complete;
   - accessibility beta tasks pass.

6. **Production App Store, Play, and F-Droid**
   - burn-in period;
   - store compliance;
   - reproducible release process;
   - privacy/security review;
   - migration stability.

7. **Desktop**
   - macOS, Windows, Linux clients from the same Rust core and Flutter architecture.

## 2. Work-item done

A work item is done only when:

- intended RED observed;
- minimum GREEN observed;
- required sweeps pass;
- evidence exists;
- independent review approved;
- patch integrated;
- integration worktree rerun passes;
- traceability updated;
- verification label accurate;
- higher unavailable claims listed;
- no unexplained warnings/skips.

## 3. Feature done

A feature is done only when:

- every advertised action works;
- error, cancellation, disconnect, and read-only behavior work;
- state is persistent where required;
- accessibility equivalent exists;
- tests prove each relevant layer;
- documentation exists;
- no placeholder visual/control remains;
- telemetry/privacy behavior is covered;
- compatibility status is accurate.

## 4. Device-profile done

A profile can be:

### Simulator verified

- complete schema;
- provenance-approved fixtures;
- byte tests;
- replay/simulator E2E;
- no physical claim.

### Capture verified

- lawful real-device captures;
- expected semantics corroborated;
- no end-to-end physical write claim unless performed.

### Community confirmed

- contributor ran a declared matrix;
- evidence reviewed;
- scope of confirmation recorded.

### Hardware verified

- required physical matrix passed;
- device, firmware, OS, transport, adapter, app commit recorded;
- read and write workflows completed;
- reconnect/partial-failure behavior tested;
- sanitized evidence retained.

## 5. Public beta gate

All must be true:

- AM4 and FM3 vertical slices are hardware verified.
- Preset browsing/read/save, complete routing, block/parameter editing, scenes/channels/modifiers, tuner/tempo/looper where supported, cab/DynaCab, FC editing, performance panels, offline library, import/export, persistent undo, and staged AI framework are implemented for the advertised scope.
- At least Preset Doctor, Tone Architect, and Scene Composer meet their local validation/preview contracts. Provider availability may be marked beta.
- Six targeted transport families are implemented or a reviewed decision changes scope.
- Simulator and Capture Lab are usable by contributors.
- Blind-user task suite passes for the primary workflows.
- Store/debug builds do not contain Node/local HTTP/WebView editor architecture.
- Pack signature/update path works.
- F-Droid flavor satisfies network/telemetry policy.
- Security and privacy review passes.
- No known data-loss bug.
- No unexplained crash, warning, or skipped required test.
- Compatibility report is generated from evidence.
- Installation/build/contribution docs are current.

## 6. Stable-release gate

In addition to beta:

- burn-in period with no unresolved critical issues;
- migration from supported beta versions;
- broader hardware/community matrix;
- release rollback;
- SBOM and dependency policy;
- signed reproducible release documentation;
- localization readiness;
- performance budgets met;
- maintainer/security response process established.

## 7. Honest blockers

Valid blockers include:

- unavailable physical device/adapter;
- missing lawful fixture;
- signing credentials;
- app-store action requiring the account owner;
- legal uncertainty;
- OS limitation;
- upstream toolchain defect;
- a product decision that cannot safely be inferred.

A blocked work item does not block independent packets. Mark it, document it, continue.
