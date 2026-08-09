# TOP-RSCH-006 source notes

Accessed 2026-08-08. Full URLs and claim mapping are in
`docs/research/accessibility-test-plan.md`.

| ID | Official source | Claim used |
| --- | --- | --- |
| F1 | Flutter Accessibility testing | Guideline API covers target size, text contrast, and labels; Inspector/Scanner workflow |
| F2 | Flutter Accessibility | Screen-reader testing is a release checklist item; accessibility is framework + platform concern |
| F3 | Flutter code debugging | `debugDumpSemanticsTree()` diagnoses the tree presented toward system accessibility APIs |
| F4 | Flutter integration tests | `flutter drive` can run on physical devices/emulators and Firebase Test Lab; not speech proof |
| A1 | Apple accessibility testing | Task matrix; physical device required because VoiceOver is unavailable in Simulator; Screen Curtain |
| A2 | Apple accessibility audits | Inspector audit categories and explicit warning that audits do not guarantee full accessibility |
| A3 | Apple Accessibility Inspector | Hierarchy/attributes/actions inspection |
| A4 | Apple VoiceOver | VoiceOver is a gesture-based screen reader; physical user interaction model |
| A5 | Apple Supporting VoiceOver | Focus/order/state checks and Screen Curtain eyes-free technique |
| A6 | Apple `performAccessibilityAudit` | Common audit automation in XCTest UI tests; not VoiceOver task evidence |
| G1 | Android accessibility testing | TalkBack linear/explore-by-touch checks; developer speech output; user-testing guidance |
| G2 | Android Compose semantics | Semantics tree serves accessibility services and tests; custom canvas content needs explicit semantics |
| G3 | Android inspect/debug | Accessibility Suite, Layout Inspector, and TalkBack diagnostics; assistive testing recommended |
| G4 | AndroidX AccessibilityChecks | Espresso automation covers a set of accessibility checks |
| G5 | Google Accessibility Scanner | Labels, target size, clickability, contrast; not a manual-test replacement and no guarantee |
| G6 | Android testing user-testing section | Recruitment channels; used only as guidance, with Topology privacy minimization |
| G7 | Android accessibility testing codelab | TalkBack can be set up on a device or emulator with Accessibility Suite; emulator audio/setup can vary |

No source was used to claim legal compliance, medical suitability, broad
population accessibility, or a specific OEM/firmware compatibility result.
