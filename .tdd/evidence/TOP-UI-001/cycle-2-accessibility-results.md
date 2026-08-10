# TOP-UI-001 amended accessibility evidence

Layer: Flutter widget/layout/semantics (L4).

- Phone and tablet retain the same destination list while intentionally using `BottomNavigationBar` and `NavigationRail` under normal motion/text settings.
- Navigation controls expose activation actions and selected state; the amended test checks button/action flags on phone and focus/action/selected-state flags on tablet.
- The shell wraps navigation/content in an `OrderedTraversalPolicy` focus group, uses ordered static controls for fallback paths, and keeps a stable content `FocusNode`; the test verifies focus restoration across the phone-to-tablet change.
- Current destination is a live semantics region; the test taps Routing and Library and verifies the announced destination label and focus state.
- Large text switches to a wrapping SDK-only destination layout with soft-wrapped labels and explicit ordinal semantics sort keys; the test verifies all labels and keys.
- Reduced-motion mode switches to a non-animated gesture/semantics layout; the test verifies immediate destination update and no scheduled animation frame.

Earned at this layer: candidate evidence for `SEMANTICS_VERIFIED`, pending fresh independent accessibility review and parent integration rerun.

Not earned: `PLATFORM_SIMULATOR_VERIFIED`, physical VoiceOver/TalkBack (`PLATFORM_DEVICE_VERIFIED`), `VISUAL_ASSET_COMPLETE`, or any hardware claim.
