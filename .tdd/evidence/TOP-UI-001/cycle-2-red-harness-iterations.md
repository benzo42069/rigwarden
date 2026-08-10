# Second-cycle RED harness iterations

The first two attempts were rejected as harness issues and are not RED evidence:

- `flutter test ...`: compile error from asserting a nonexistent `SemanticsRole.button` enum member; corrected to the button flag.
- `flutter test ...`: guarded-function conflict because `tester.tap` was not awaited; corrected by awaiting both semantic taps.

The accepted RED is preserved verbatim in `cycle-2-red.log` and fails for the intended missing focus policy.
