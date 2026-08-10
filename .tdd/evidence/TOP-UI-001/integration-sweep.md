# TOP-UI-001 integration sweep

Integration date: 2026-08-10

Parent reran the approved candidate serially from `apps/mobile_flutter`:

1. `flutter test test/features/session/session_shell_layout_test.dart --plain-name "session shell adapts without hiding editor destinations"` — exit 0.
2. `flutter test` — exit 0.
3. `flutter analyze` — exit 0.
4. `dart format --output=none --set-exit-if-changed .` — exit 0.

The only promoted label is `SEMANTICS_VERIFIED` at Flutter L4 for the bounded
adaptive shell. No physical VoiceOver/TalkBack, platform-device, complete
nonvisual editor, visual asset, protocol, transport, or hardware claim is
made.
