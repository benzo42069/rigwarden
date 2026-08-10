# TOP-UI-001 accessibility evidence

Layer: Flutter widget/semantics (L4).

- The phone branch exposes every declared destination through an explicit `BottomNavigationBar` with stable labels and button semantics.
- The tablet branch exposes the same destination list through a persistent `NavigationRail` with labels and actions.
- The active content region is a semantic container labelled `Current editor destination: <label>`; its visual text is not the only accessible name.
- The focused test checks the semantic label prefix for every destination at 390x844 and 1024x768, plus the intentional `BottomNavigationBar`/`NavigationRail` structural change.
- The test resets the surface size in teardown even when an assertion fails.

Earned at this layer: `SEMANTICS_VERIFIED` pending independent accessibility review and integration rerun.

Not earned: physical VoiceOver/TalkBack (`PLATFORM_DEVICE_VERIFIED`), visual asset completeness, or hardware compatibility.
