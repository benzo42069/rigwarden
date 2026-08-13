import 'dart:async';
import 'dart:ui' show Tristate;

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:rigwarden/core/bridge/generated/api/simulated_parameter_edit.dart';
import 'package:rigwarden/core/bridge/generated/frb_generated.dart';

void main() {
  testWidgets('simulated edit confirmation and undo complete end to end', (
    tester,
  ) async {
    late SimulatedParameterEditSession session;
    late SimulatedParameterEditState initial;
    await _runReal<bool>(tester, () async {
      await _ensureRustInitialized();
      session = await createSimulatedParameterEditSession();
      initial = await session.initialState();
      return true;
    });

    expect(initial.exchangeCount, 0);
    expect(initial.transcript, isEmpty);
    expect(initial.target, 'amp-1/gain');
    expect(initial.context, 'synthetic preset / Amp 1 / gain');
    expect(initial.unit, 'synthetic stored units');
    expect(initial.minStored, 0);
    expect(initial.maxStored, 100);
    expect(initial.stepStored, 1);
    expect(initial.stepDisplay, closeTo(0.1, 0.000001));
    expect(initial.decimalPlaces, 1);
    expect(initial.readOnly, isFalse);
    expect(initial.errorMessage, isNull);

    final editPendingGate = Completer<void>();
    final undoPendingGate = Completer<void>();
    Future<SimulatedParameterEditState>? beginEditFuture;
    Future<SimulatedParameterEditState>? confirmEditFuture;
    Future<SimulatedParameterEditState>? beginUndoFuture;
    Future<SimulatedParameterEditState>? confirmUndoFuture;

    Future<SimulatedParameterEditState> beginEdit() => beginEditFuture =
        _runReal(tester, () => session.beginEdit(requestedStoredValue: 45));
    Future<SimulatedParameterEditState> confirmEdit() =>
        confirmEditFuture = _runReal(tester, session.confirmEdit);
    Future<SimulatedParameterEditState> beginUndo() =>
        beginUndoFuture = _runReal(tester, session.beginUndo);
    Future<SimulatedParameterEditState> confirmUndo() =>
        confirmUndoFuture = _runReal(tester, session.confirmUndo);

    await tester.pumpWidget(
      MaterialApp(
        home: SimulatedParameterEditHarness(
          initialState: initial,
          beginEdit: beginEdit,
          confirmEdit: confirmEdit,
          beginUndo: beginUndo,
          confirmUndo: confirmUndo,
          beforeConfirmEdit: () => editPendingGate.future,
          beforeConfirmUndo: () => undoPendingGate.future,
        ),
      ),
    );
    await tester.pump();

    expect(find.text('Confirmed initial: 3.0'), findsOneWidget);
    expect(find.bySemanticsLabel('Edit Amp 1 gain to 4.5'), findsOneWidget);
    expect(find.bySemanticsLabel('Undo Amp 1 gain edit'), findsOneWidget);

    final initialSemanticsHandle = tester.ensureSemantics();
    final initialStateSemantics = tester.getSemantics(
      find.byKey(const ValueKey('typed-current-state')),
    );
    expect(initialStateSemantics.label, contains('synthetic stored units'));
    expect(initialStateSemantics.label, contains('range 0.0 to 10.0'));
    expect(initialStateSemantics.label, contains('step 0.1'));
    expect(initialStateSemantics.label, contains('precision 1'));
    _expectFullStateSemantics(tester, phase: 'Confirmed initial', value: '3.0');
    final initialEditSemantics = tester.getSemantics(
      find.bySemanticsLabel('Edit Amp 1 gain to 4.5'),
    );
    expect(_isButton(initialEditSemantics), isTrue);
    expect(
      initialEditSemantics.getSemanticsData().hasAction(SemanticsAction.tap),
      isTrue,
    );
    expect(_isEnabled(initialEditSemantics), isTrue);
    initialSemanticsHandle.dispose();

    await tester.tap(find.bySemanticsLabel('Edit Amp 1 gain to 4.5'));
    final pendingEdit = await beginEditFuture!;
    await tester.pump();

    expect(pendingEdit.exchangeCount, 0);
    expect(pendingEdit.transcript, isEmpty);
    expect(pendingEdit.journalEntryCount, 0);

    final pendingSemanticsHandle = tester.ensureSemantics();
    expect(
      tester
          .getSemantics(find.byKey(const ValueKey('typed-current-state')))
          .label,
      contains('Pending edit'),
    );
    _expectFullStateSemantics(tester, phase: 'Pending edit', value: '4.5');
    expect(
      _isEnabled(
        tester.getSemantics(find.bySemanticsLabel('Edit Amp 1 gain to 4.5')),
      ),
      isFalse,
    );
    expect(
      _isEnabled(
        tester.getSemantics(find.bySemanticsLabel('Undo Amp 1 gain edit')),
      ),
      isFalse,
    );
    pendingSemanticsHandle.dispose();

    expect(
      tester
          .widget<Text>(find.byKey(const ValueKey('typed-current-state')))
          .data,
      'Pending edit: 4.5',
    );
    editPendingGate.complete();
    await tester.pump();
    final confirmedEdit = await confirmEditFuture!;
    await tester.pump();
    expect(confirmedEdit.exchangeCount, 1);
    expect(_transcriptValues(confirmedEdit), <int>[45, 45]);
    expect(confirmedEdit.journalEntryCount, 1);
    expect(confirmedEdit.journalPreviousStoredValue, 30);
    expect(confirmedEdit.journalNewStoredValue, 45);
    expect(
      tester
          .widget<Text>(find.byKey(const ValueKey('typed-current-state')))
          .data,
      'Confirmed edit: 4.5',
    );
    final confirmedEditSemanticsHandle = tester.ensureSemantics();
    _expectFullStateSemantics(tester, phase: 'Confirmed edit', value: '4.5');
    confirmedEditSemanticsHandle.dispose();
    expect(
      find.text('Journal: prior 3.0 -> new 4.5 (1 entry)'),
      findsOneWidget,
    );

    await tester.tap(find.bySemanticsLabel('Undo Amp 1 gain edit'));
    final pendingUndo = await beginUndoFuture!;
    await tester.pump();

    expect(pendingUndo.exchangeCount, 1);
    expect(_transcriptValues(pendingUndo), <int>[45, 45]);
    expect(pendingUndo.journalEntryCount, 1);
    expect(pendingUndo.journalPreviousStoredValue, 30);
    expect(pendingUndo.journalNewStoredValue, 45);

    expect(
      tester
          .widget<Text>(find.byKey(const ValueKey('typed-current-state')))
          .data,
      'Pending undo: 3.0',
    );
    final pendingUndoSemanticsHandle = tester.ensureSemantics();
    _expectFullStateSemantics(tester, phase: 'Pending undo', value: '3.0');
    pendingUndoSemanticsHandle.dispose();
    undoPendingGate.complete();
    await tester.pump();
    final confirmedUndo = await confirmUndoFuture!;
    await tester.pump();
    expect(confirmedUndo.exchangeCount, 2);
    expect(_transcriptValues(confirmedUndo), <int>[45, 45, 30, 30]);
    expect(confirmedUndo.journalEntryCount, 0);
    expect(
      tester
          .widget<Text>(find.byKey(const ValueKey('typed-current-state')))
          .data,
      'Confirmed undo: 3.0',
    );
    final confirmedUndoSemanticsHandle = tester.ensureSemantics();
    _expectFullStateSemantics(tester, phase: 'Confirmed undo', value: '3.0');
    confirmedUndoSemanticsHandle.dispose();
    expect(
      find.text('Journal: prior 3.0 -> new 4.5 (0 entries)'),
      findsOneWidget,
    );
    expect(
      find.text('Simulator: rigwarden.synthetic-scripted-simulator'),
      findsOneWidget,
    );
    expect(find.textContaining('vendor'), findsNothing);
    expect(find.textContaining('bytes'), findsNothing);

    final semanticsHandle = tester.ensureSemantics();
    final editSemantics = tester.getSemantics(
      find.bySemanticsLabel('Edit Amp 1 gain to 4.5'),
    );
    expect(_isButton(editSemantics), isTrue);
    expect(
      editSemantics.getSemanticsData().hasAction(SemanticsAction.tap),
      isTrue,
    );
    expect(editSemantics.label, contains('Amp 1 gain'));
    expect(editSemantics.hint, contains('Action: edit'));
    expect(_isEnabled(editSemantics), isTrue);
    final undoSemantics = tester.getSemantics(
      find.bySemanticsLabel('Undo Amp 1 gain edit'),
    );
    expect(_isButton(undoSemantics), isTrue);
    expect(
      undoSemantics.getSemanticsData().hasAction(SemanticsAction.tap),
      isFalse,
    );
    expect(_isEnabled(undoSemantics), isFalse);
    expect(
      tester
          .getSemantics(find.byKey(const ValueKey('typed-current-state')))
          .label,
      contains('Confirmed undo'),
    );
    semanticsHandle.dispose();

    await tester.sendKeyEvent(LogicalKeyboardKey.tab);
    await tester.pump();
    expect(find.bySemanticsLabel('Edit Amp 1 gain to 4.5'), findsOneWidget);
  });

  testWidgets(
    'emitted semantics retain focus order, keyboard activation, and recovery',
    (tester) async {
      late SimulatedParameterEditSession session;
      late SimulatedParameterEditState confirmed;
      await _runReal<bool>(tester, () async {
        await _ensureRustInitialized();
        session = await createSimulatedParameterEditSession();
        await session.beginEdit(requestedStoredValue: 45);
        confirmed = await session.confirmEdit();
        return true;
      });

      var editActions = 0;
      var undoActions = 0;
      Future<SimulatedParameterEditState>? beginEditFuture;
      Future<SimulatedParameterEditState>? confirmEditFuture;
      Future<SimulatedParameterEditState>? beginUndoFuture;
      Future<SimulatedParameterEditState>? confirmUndoFuture;

      Future<SimulatedParameterEditState> beginEdit() {
        editActions += 1;
        return beginEditFuture = _runReal(
          tester,
          () => session.beginEdit(requestedStoredValue: 45),
        );
      }

      Future<SimulatedParameterEditState> confirmEdit() {
        return confirmEditFuture = _runReal(tester, session.confirmEdit);
      }

      Future<SimulatedParameterEditState> beginUndo() {
        undoActions += 1;
        return beginUndoFuture = _runReal(tester, session.beginUndo);
      }

      Future<SimulatedParameterEditState> confirmUndo() {
        return confirmUndoFuture = _runReal(tester, session.confirmUndo);
      }

      await tester.pumpWidget(
        MaterialApp(
          home: SimulatedParameterEditHarness(
            initialState: confirmed,
            beginEdit: beginEdit,
            confirmEdit: confirmEdit,
            beginUndo: beginUndo,
            confirmUndo: confirmUndo,
            beforeConfirmEdit: () async {},
            beforeConfirmUndo: () async {},
          ),
        ),
      );
      await tester.pump();

      final semanticsHandle = tester.ensureSemantics();
      final editButton = find.byKey(const ValueKey('semantic-edit-button'));
      final undoButton = find.byKey(const ValueKey('semantic-undo-button'));
      final editFocus = tester.widget<ElevatedButton>(editButton).focusNode!;
      final undoFocus = tester.widget<ElevatedButton>(undoButton).focusNode!;

      editFocus.requestFocus();
      await tester.pump();
      await tester.pump();
      expect(editFocus.hasFocus, isTrue);
      expect(undoFocus.hasFocus, isFalse);
      expect(tester.binding.focusManager.primaryFocus, same(editFocus));
      _expectFocusSemantics(tester, 'Edit Amp 1 gain to 4.5', focused: true);
      _expectFocusSemantics(tester, 'Undo Amp 1 gain edit', focused: false);

      await tester.sendKeyEvent(LogicalKeyboardKey.tab);
      await tester.pump();
      expect(editFocus.hasFocus, isFalse);
      expect(undoFocus.hasFocus, isTrue);
      expect(tester.binding.focusManager.primaryFocus, same(undoFocus));
      _expectFocusSemantics(tester, 'Edit Amp 1 gain to 4.5', focused: false);
      _expectFocusSemantics(tester, 'Undo Amp 1 gain edit', focused: true);

      await tester.sendKeyEvent(LogicalKeyboardKey.enter);
      await tester.pump();
      expect(undoActions, 1);
      await beginUndoFuture!;
      await tester.pump();
      await confirmUndoFuture!;
      await tester.pump();
      await tester.pump();
      expect(editFocus.hasFocus, isTrue);
      expect(undoFocus.hasFocus, isFalse);
      expect(tester.binding.focusManager.primaryFocus, same(editFocus));
      _expectFocusSemantics(tester, 'Edit Amp 1 gain to 4.5', focused: true);
      _expectFocusSemantics(tester, 'Undo Amp 1 gain edit', focused: false);
      _expectFullStateSemantics(tester, phase: 'Confirmed undo', value: '3.0');

      editFocus.requestFocus();
      await tester.pump();
      await tester.sendKeyEvent(LogicalKeyboardKey.enter);
      await tester.pump();
      expect(editActions, 1);
      await beginEditFuture!;
      await tester.pump();
      await confirmEditFuture!;
      await tester.pump();
      expect(editFocus.hasFocus, isTrue);
      expect(tester.binding.focusManager.primaryFocus, same(editFocus));
      _expectFullStateSemantics(tester, phase: 'Confirmed edit', value: '4.5');
      semanticsHandle.dispose();
    },
  );

  testWidgets('read-only synthetic rejection crosses FFI without exchange', (
    tester,
  ) async {
    late SimulatedParameterEditSession session;
    late SimulatedParameterEditState initial;
    await _runReal<bool>(tester, () async {
      await _ensureRustInitialized();
      session = await createSimulatedReadOnlyParameterEditSession();
      initial = await session.initialState();
      return true;
    });

    expect(initial.readOnly, isTrue);
    expect(initial.exchangeCount, 0);
    expect(initial.transcript, isEmpty);
    expect(initial.errorMessage, contains('read-only'));

    final rejection = await _runReal<Object>(tester, () async {
      try {
        await session.beginEdit(requestedStoredValue: 45);
        return StateError(
          'read-only synthetic session unexpectedly accepted a write',
        );
      } catch (error) {
        return error;
      }
    });
    expect(rejection, isA<SimulatedParameterEditError>());
    final typedRejection = rejection as SimulatedParameterEditError;
    expect(typedRejection.code, SimulatedParameterEditErrorCode.readOnly);
    expect(typedRejection.exchangeCount, 0);

    late SimulatedParameterEditSession writableSession;
    await _runReal<bool>(tester, () async {
      writableSession = await createSimulatedParameterEditSession();
      return true;
    });
    final invalid = await _runReal<Object>(tester, () async {
      try {
        await writableSession.beginEdit(requestedStoredValue: 101);
        return StateError(
          'out-of-range synthetic session unexpectedly accepted a write',
        );
      } catch (error) {
        return error;
      }
    });
    expect(invalid, isA<SimulatedParameterEditError>());
    final typedInvalid = invalid as SimulatedParameterEditError;
    expect(typedInvalid.code, SimulatedParameterEditErrorCode.outOfRange);
    expect(typedInvalid.exchangeCount, 0);

    await tester.pumpWidget(
      MaterialApp(
        home: SimulatedParameterEditHarness(
          initialState: initial,
          beginEdit: () => _runReal(
            tester,
            () => session.beginEdit(requestedStoredValue: 45),
          ),
          confirmEdit: () async => initial,
          beginUndo: () async => initial,
          confirmUndo: () async => initial,
          beforeConfirmEdit: () async {},
          beforeConfirmUndo: () async {},
        ),
      ),
    );
    await tester.pump();

    final semanticsHandle = tester.ensureSemantics();
    _expectFullStateSemantics(
      tester,
      phase: 'Read-only',
      value: '3.0',
      error: 'read-only synthetic profile rejects writes',
    );
    final editSemantics = tester.getSemantics(
      find.bySemanticsLabel('Edit Amp 1 gain to 4.5'),
    );
    expect(_isButton(editSemantics), isTrue);
    expect(_isEnabled(editSemantics), isFalse);
    expect(
      editSemantics.getSemanticsData().hasAction(SemanticsAction.tap),
      isFalse,
    );
    expect(
      tester
          .getSemantics(find.byKey(const ValueKey('typed-current-state')))
          .label,
      contains('Read-only'),
    );
    expect(
      find.text('Error: read-only synthetic profile rejects writes'),
      findsOneWidget,
    );
    semanticsHandle.dispose();
  });

  testWidgets('out-of-range typed error renders through FFI semantics', (
    tester,
  ) async {
    late SimulatedParameterEditSession session;
    late SimulatedParameterEditState initial;
    await _runReal<bool>(tester, () async {
      await _ensureRustInitialized();
      session = await createSimulatedParameterEditSession();
      initial = await session.initialState();
      return true;
    });

    Object? capturedError;
    final invalidCallbackFinished = Completer<void>();
    Future<SimulatedParameterEditState> beginInvalidEdit() async {
      final result = await tester.runAsync<Object>(() async {
        try {
          final state = await session.beginEdit(requestedStoredValue: 101);
          invalidCallbackFinished.complete();
          return state;
        } catch (error) {
          capturedError = error;
          invalidCallbackFinished.complete();
          return error;
        }
      });
      if (result == null) {
        throw StateError('real FFI operation returned no result');
      }
      if (result is SimulatedParameterEditError) {
        throw result;
      }
      return result as SimulatedParameterEditState;
    }

    await tester.pumpWidget(
      MaterialApp(
        home: SimulatedParameterEditHarness(
          initialState: initial,
          beginEdit: beginInvalidEdit,
          confirmEdit: () async => initial,
          beginUndo: () async => initial,
          confirmUndo: () async => initial,
          beforeConfirmEdit: () async {},
          beforeConfirmUndo: () async {},
          errorStateFromFailure: (error) {
            expect(error, isA<SimulatedParameterEditError>());
            final typedError = error as SimulatedParameterEditError;
            expect(typedError.code, SimulatedParameterEditErrorCode.outOfRange);
            expect(typedError.exchangeCount, 0);
            return _stateWithError(initial, typedError.message);
          },
        ),
      ),
    );
    await tester.pump();

    await tester.tap(find.bySemanticsLabel('Edit Amp 1 gain to 4.5'));
    await invalidCallbackFinished.future;
    await tester.pump();
    expect(capturedError, isA<SimulatedParameterEditError>());
    final typedError = capturedError! as SimulatedParameterEditError;
    expect(typedError.code, SimulatedParameterEditErrorCode.outOfRange);
    expect(typedError.exchangeCount, 0);

    final semanticsHandle = tester.ensureSemantics();
    _expectFullStateSemantics(
      tester,
      phase: 'Confirmed initial',
      value: '3.0',
      error: 'synthetic stored value 101 is outside profile range 0..100',
    );
    expect(
      find.text(
        'Error: synthetic stored value 101 is outside profile range 0..100',
      ),
      findsOneWidget,
    );
    semanticsHandle.dispose();
  });
}

bool _rustInitialized = false;

Future<void> _ensureRustInitialized() async {
  if (_rustInitialized) {
    return;
  }
  try {
    await RustLib.init();
  } on StateError catch (error) {
    if (!error.toString().contains('initialize flutter_rust_bridge twice')) {
      rethrow;
    }
  }
  _rustInitialized = true;
}

bool _isButton(SemanticsNode node) {
  return node.getSemanticsData().flagsCollection.isButton;
}

bool _isEnabled(SemanticsNode node) {
  return node.getSemanticsData().flagsCollection.isEnabled == Tristate.isTrue;
}

void _expectFocusSemantics(
  WidgetTester tester,
  String label, {
  required bool focused,
}) {
  final key = label.startsWith('Edit')
      ? const ValueKey('semantic-edit-semantics')
      : const ValueKey('semantic-undo-semantics');
  final data = tester.getSemantics(find.byKey(key)).getSemanticsData();
  expect(data.flagsCollection.isFocused, isNot(Tristate.none));
  expect(
    data.flagsCollection.isFocused,
    focused ? Tristate.isTrue : Tristate.isFalse,
  );
}

void _expectFullStateSemantics(
  WidgetTester tester, {
  required String phase,
  required String value,
  String? error,
}) {
  final expectedLabel =
      '$phase synthetic preset / Amp 1 / gain; $value synthetic stored units; '
      'range 0.0 to 10.0; step 0.1; precision 1'
      '${error == null ? '' : ' error $error'}';
  final node = tester.getSemantics(
    find.byKey(const ValueKey('typed-current-state')),
  );
  expect(node.label, startsWith(expectedLabel));
  expect(node.getSemanticsData().flagsCollection.isLiveRegion, isTrue);
}

SimulatedParameterEditState _stateWithError(
  SimulatedParameterEditState state,
  String errorMessage,
) {
  return SimulatedParameterEditState(
    phase: state.phase,
    target: state.target,
    context: state.context,
    unit: state.unit,
    storedValue: state.storedValue,
    displayValue: state.displayValue,
    decimalPlaces: state.decimalPlaces,
    minStored: state.minStored,
    maxStored: state.maxStored,
    stepStored: state.stepStored,
    stepDisplay: state.stepDisplay,
    readOnly: state.readOnly,
    errorMessage: errorMessage,
    journalPreviousStoredValue: state.journalPreviousStoredValue,
    journalNewStoredValue: state.journalNewStoredValue,
    journalEntryCount: state.journalEntryCount,
    exchangeCount: state.exchangeCount,
    simulatorLabel: state.simulatorLabel,
    transcript: state.transcript,
  );
}

List<int> _transcriptValues(SimulatedParameterEditState state) {
  return state.transcript.map((entry) => entry.storedValue).toList();
}

Future<T> _runReal<T>(
  WidgetTester tester,
  Future<T> Function() operation,
) async {
  final result = await tester.runAsync<T>(operation);
  if (result == null) {
    throw StateError('real FFI operation returned no result');
  }
  return result;
}

/// Test-owned semantic harness. It exposes only typed edit/undo actions and
/// the state returned by Rust; it is not a production parameter editor.
class SimulatedParameterEditHarness extends StatefulWidget {
  const SimulatedParameterEditHarness({
    required this.initialState,
    required this.beginEdit,
    required this.confirmEdit,
    required this.beginUndo,
    required this.confirmUndo,
    required this.beforeConfirmEdit,
    required this.beforeConfirmUndo,
    this.errorStateFromFailure,
    super.key,
  });

  final SimulatedParameterEditState initialState;
  final Future<SimulatedParameterEditState> Function() beginEdit;
  final Future<SimulatedParameterEditState> Function() confirmEdit;
  final Future<SimulatedParameterEditState> Function() beginUndo;
  final Future<SimulatedParameterEditState> Function() confirmUndo;
  final Future<void> Function() beforeConfirmEdit;
  final Future<void> Function() beforeConfirmUndo;
  final SimulatedParameterEditState Function(Object error)?
  errorStateFromFailure;

  @override
  State<SimulatedParameterEditHarness> createState() =>
      _SimulatedParameterEditHarnessState();
}

class _SimulatedParameterEditHarnessState
    extends State<SimulatedParameterEditHarness> {
  late SimulatedParameterEditState _state = widget.initialState;
  late final List<SimulatedParameterEditState> _history =
      <SimulatedParameterEditState>[widget.initialState];
  late final FocusNode _editFocusNode = FocusNode(
    debugLabel: 'synthetic edit control',
  );
  late final FocusNode _undoFocusNode = FocusNode(
    debugLabel: 'synthetic undo control',
  );

  @override
  void initState() {
    super.initState();
    _editFocusNode.addListener(_handleFocusChanged);
    _undoFocusNode.addListener(_handleFocusChanged);
  }

  void _handleFocusChanged() {
    if (mounted) {
      setState(() {});
    }
  }

  void _restoreEditFocus() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted && _editEnabled) {
        _editFocusNode.requestFocus();
      }
    });
  }

  bool get _pending => switch (_state.phase) {
    SimulatedParameterEditPhase.pendingEdit ||
    SimulatedParameterEditPhase.pendingUndo => true,
    _ => false,
  };

  bool get _editEnabled => !_state.readOnly && !_pending;

  bool get _undoEnabled =>
      !_state.readOnly && !_pending && _state.journalEntryCount > 0;

  Future<void> _edit() async {
    late SimulatedParameterEditState pending;
    try {
      pending = await widget.beginEdit();
    } on Object catch (error) {
      final errorState = widget.errorStateFromFailure?.call(error);
      if (errorState == null) {
        rethrow;
      }
      if (!mounted) {
        return;
      }
      setState(() {
        _state = errorState;
        _history.add(errorState);
      });
      return;
    }
    if (!mounted) {
      return;
    }
    setState(() {
      _state = pending;
      _history.add(pending);
    });

    await widget.beforeConfirmEdit();
    final confirmed = await widget.confirmEdit();
    if (!mounted) {
      return;
    }
    setState(() {
      _state = confirmed;
      _history.add(confirmed);
    });
    _restoreEditFocus();
  }

  Future<void> _undo() async {
    final pending = await widget.beginUndo();
    if (!mounted) {
      return;
    }
    setState(() {
      _state = pending;
      _history.add(pending);
    });

    await widget.beforeConfirmUndo();
    final confirmed = await widget.confirmUndo();
    if (!mounted) {
      return;
    }
    setState(() {
      _state = confirmed;
      _history.add(confirmed);
    });
    if (_editEnabled) {
      _restoreEditFocus();
    }
  }

  @override
  void dispose() {
    _editFocusNode.removeListener(_handleFocusChanged);
    _undoFocusNode.removeListener(_handleFocusChanged);
    _editFocusNode.dispose();
    _undoFocusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final editActionLabel = _editEnabled
        ? 'Action: edit ${_state.context} to 4.5'
        : 'Action unavailable: ${_state.readOnly ? 'read-only' : 'pending'}';
    final undoActionLabel = _undoEnabled
        ? 'Action: undo ${_state.context}'
        : 'Action unavailable: ${_state.readOnly ? 'read-only' : 'pending or no journal entry'}';

    return Scaffold(
      appBar: AppBar(title: const Text('Synthetic parameter edit')),
      body: FocusTraversalGroup(
        policy: OrderedTraversalPolicy(),
        child: ListView(
          padding: const EdgeInsets.all(16),
          children: <Widget>[
            FocusTraversalOrder(
              order: const NumericFocusOrder(1),
              child: ListenableBuilder(
                listenable: _editFocusNode,
                builder: (context, child) => Semantics(
                  key: const ValueKey('semantic-edit-semantics'),
                  container: true,
                  button: true,
                  focusable: true,
                  focused: _editFocusNode.hasFocus,
                  enabled: _editEnabled,
                  label: 'Edit Amp 1 gain to 4.5',
                  hint: editActionLabel,
                  onTap: _editEnabled ? _edit : null,
                  child: ExcludeSemantics(child: child!),
                ),
                child: ElevatedButton(
                  key: const ValueKey('semantic-edit-button'),
                  focusNode: _editFocusNode,
                  onPressed: _editEnabled ? _edit : null,
                  child: const Text('Edit gain to 4.5'),
                ),
              ),
            ),
            FocusTraversalOrder(
              order: const NumericFocusOrder(2),
              child: ListenableBuilder(
                listenable: _undoFocusNode,
                builder: (context, child) => Semantics(
                  key: const ValueKey('semantic-undo-semantics'),
                  container: true,
                  button: true,
                  focusable: true,
                  focused: _undoFocusNode.hasFocus,
                  enabled: _undoEnabled,
                  label: 'Undo Amp 1 gain edit',
                  hint: undoActionLabel,
                  onTap: _undoEnabled ? _undo : null,
                  child: ExcludeSemantics(child: child!),
                ),
                child: ElevatedButton(
                  key: const ValueKey('semantic-undo-button'),
                  focusNode: _undoFocusNode,
                  onPressed: _undoEnabled ? _undo : null,
                  child: const Text('Undo gain edit'),
                ),
              ),
            ),
            Semantics(
              container: true,
              liveRegion: true,
              label:
                  '${_phaseLabel(_state)} ${_state.context}; '
                  '${_state.displayValue.toStringAsFixed(_state.decimalPlaces)} '
                  '${_state.unit}; range ${_formatStored(_state.minStored)} '
                  'to ${_formatStored(_state.maxStored)}; '
                  'step ${_state.stepDisplay}; precision ${_state.decimalPlaces}'
                  '${_state.errorMessage == null ? '' : ' error ${_state.errorMessage}'}',
              child: Text(
                '${_phaseLabel(_state)}: '
                '${_state.displayValue.toStringAsFixed(_state.decimalPlaces)}',
                key: const ValueKey('typed-current-state'),
              ),
            ),
            Text(
              'Simulator: ${_state.simulatorLabel}',
              key: const ValueKey('simulator-label'),
            ),
            Text(
              'Journal: prior ${_formatStored(_state.journalPreviousStoredValue)} '
              '-> new ${_formatStored(_state.journalNewStoredValue)} '
              '(${_state.journalEntryCount} ${_state.journalEntryCount == 1 ? 'entry' : 'entries'})',
              key: const ValueKey('typed-journal-state'),
            ),
            if (_state.errorMessage != null)
              Text('Error: ${_state.errorMessage}'),
            for (final state in _history.skip(1))
              Text(
                '${_phaseLabel(state)}: '
                '${state.displayValue.toStringAsFixed(state.decimalPlaces)}',
              ),
          ],
        ),
      ),
    );
  }

  String _phaseLabel(SimulatedParameterEditState state) {
    if (state.readOnly) {
      return 'Read-only';
    }
    return switch (state.phase) {
      SimulatedParameterEditPhase.idle => 'Confirmed initial',
      SimulatedParameterEditPhase.pendingEdit => 'Pending edit',
      SimulatedParameterEditPhase.confirmedEdit => 'Confirmed edit',
      SimulatedParameterEditPhase.pendingUndo => 'Pending undo',
      SimulatedParameterEditPhase.confirmedUndo => 'Confirmed undo',
    };
  }

  String _formatStored(int? storedValue) {
    if (storedValue == null) {
      return 'none';
    }
    return (storedValue / 10).toStringAsFixed(1);
  }
}
