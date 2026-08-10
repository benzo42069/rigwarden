import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'dart:ui' as ui;
import 'package:rigwarden/app/topology_app.dart';
import 'package:rigwarden/features/session/session_shell.dart';
import 'package:flutter/semantics.dart';

void main() {
  const session = SessionShellModel(
    title: 'Fixture Session',
    destinations: <SessionDestination>[
      SessionDestination('Session'),
      SessionDestination('Presets'),
      SessionDestination('Routing'),
      SessionDestination('Parameters'),
      SessionDestination('Performance'),
      SessionDestination('Library'),
    ],
  );

  testWidgets('session shell adapts without hiding editor destinations', (
    tester,
  ) async {
    addTearDown(() => tester.binding.setSurfaceSize(null));

    await tester.pumpWidget(
      const MaterialApp(home: TopologyApp(session: session)),
    );

    await tester.binding.setSurfaceSize(const Size(390, 844));
    await tester.pumpAndSettle();

    expect(find.byType(BottomNavigationBar), findsOneWidget);
    expect(find.byType(NavigationRail), findsNothing);
    final navigationFocusGroup = find.byKey(
      const ValueKey('session-navigation-focus-group'),
    );
    expect(navigationFocusGroup, findsOneWidget);
    final phoneNavigation = tester.widget<FocusTraversalGroup>(
      navigationFocusGroup,
    );
    expect(phoneNavigation.policy, isA<OrderedTraversalPolicy>());
    for (var index = 0; index < session.destinations.length; index += 1) {
      final destination = session.destinations[index];
      final destinationNode = tester.getSemantics(
        find.bySemanticsLabel(RegExp('^${RegExp.escape(destination.label)}')),
      );
      final destinationData = destinationNode.getSemanticsData();
      expect(destinationData.flagsCollection.isButton, isTrue);
      expect(
        destinationData.hasAction(ui.SemanticsAction.tap),
        isTrue,
        reason: '${destination.label} must expose activation',
      );
      expect(
        destinationData.flagsCollection.isSelected == ui.Tristate.isTrue,
        index == 0,
        reason: '${destination.label} selected state must be explicit',
      );
    }

    final routing = find.bySemanticsLabel(RegExp('^Routing'));
    await tester.tap(routing);
    await tester.pumpAndSettle();
    expect(
      tester.getSemantics(
        find.bySemanticsLabel('Current editor destination: Routing'),
      ),
      matchesSemantics(
        label: 'Current editor destination: Routing',
        isLiveRegion: true,
        isFocusable: true,
        isFocused: true,
      ),
    );
    final contentFocus = tester.widget<Focus>(
      find.byKey(const ValueKey('session-content-focus')),
    );
    expect(contentFocus.focusNode!.hasFocus, isTrue);
    final contentFocusNode = contentFocus.focusNode;

    await tester.binding.setSurfaceSize(const Size(1024, 768));
    await tester.pumpAndSettle();

    expect(find.byType(NavigationRail), findsOneWidget);
    expect(find.byType(BottomNavigationBar), findsNothing);
    expect(
      tester
          .widget<Focus>(find.byKey(const ValueKey('session-content-focus')))
          .focusNode,
      same(contentFocusNode),
    );
    expect(contentFocusNode!.hasFocus, isTrue);
    for (var index = 0; index < session.destinations.length; index += 1) {
      final destination = session.destinations[index];
      final destinationNode = tester.getSemantics(
        find.bySemanticsLabel(RegExp('^${RegExp.escape(destination.label)}')),
      );
      final destinationData = destinationNode.getSemanticsData();
      expect(
        destinationData.flagsCollection.isButton,
        isTrue,
        reason: '${destination.label} must expose a button role on tablet',
      );
      expect(destinationData.hasAction(ui.SemanticsAction.tap), isTrue);
      expect(
        destinationData.flagsCollection.isSelected != ui.Tristate.none,
        isTrue,
      );
      expect(
        destinationData.flagsCollection.isSelected == ui.Tristate.isTrue,
        index == 2,
      );
    }

    await tester.pumpWidget(
      MediaQuery(
        data: const MediaQueryData(textScaler: TextScaler.linear(2)),
        child: const MaterialApp(home: TopologyApp(session: session)),
      ),
    );
    await tester.binding.setSurfaceSize(const Size(390, 844));
    await tester.pumpAndSettle();
    final largeTextNavigation = find.byKey(
      const ValueKey('session-navigation-large-text'),
    );
    expect(largeTextNavigation, findsOneWidget);
    expect(
      find.descendant(of: largeTextNavigation, matching: find.byType(Wrap)),
      findsOneWidget,
    );
    for (final destination in session.destinations) {
      final destinationText = tester.widget<Text>(
        find.descendant(
          of: largeTextNavigation,
          matching: find.text(destination.label),
        ),
      );
      expect(destinationText.softWrap, isTrue);
      expect(destinationText.overflow, isNull);
      final destinationNode = tester.getSemantics(
        find.bySemanticsLabel(destination.label),
      );
      final sortKey = destinationNode.sortKey;
      expect(sortKey, isA<OrdinalSortKey>());
      expect(
        (sortKey! as OrdinalSortKey).order,
        session.destinations.indexOf(destination),
      );
      expect(
        destinationNode.getSemanticsData().hasAction(ui.SemanticsAction.tap),
        isTrue,
      );
    }

    await tester.pumpWidget(
      MediaQuery(
        data: const MediaQueryData(disableAnimations: true),
        child: const MaterialApp(home: TopologyApp(session: session)),
      ),
    );
    await tester.binding.setSurfaceSize(const Size(390, 844));
    await tester.pumpAndSettle();
    final reducedMotionNavigation = find.byKey(
      const ValueKey('session-navigation-reduced-motion'),
    );
    expect(reducedMotionNavigation, findsOneWidget);
    await tester.tap(
      find.bySemanticsLabel(RegExp('^${RegExp.escape('Library')}')),
    );
    await tester.pump();
    expect(
      tester.getSemantics(
        find.bySemanticsLabel('Current editor destination: Library'),
      ),
      matchesSemantics(
        label: 'Current editor destination: Library',
        isLiveRegion: true,
        isFocusable: true,
        isFocused: true,
      ),
    );
    expect(tester.binding.hasScheduledFrame, isFalse);
  });
}
