import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/semantics.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:rigwarden/core/bridge/generated/api.dart';
import 'package:rigwarden/core/bridge/generated/frb_generated.dart';
import 'package:rigwarden/features/routing/accessible_route_list.dart';

void main() {
  testWidgets('serial route is completely navigable without canvas', (
    tester,
  ) async {
    final snapshot = await tester.runAsync<SerialRouteSnapshot>(() async {
      await RustLib.init();
      return readFixtureSerialRouteSnapshot();
    });
    if (snapshot == null) {
      throw StateError('real FFI route snapshot returned no result');
    }
    final semantics = tester.ensureSemantics();
    try {
      final removedConnections = <SerialRouteConnection>[];

      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AccessibleRouteList(
              snapshot: snapshot,
              onRemoveConnection: removedConnections.add,
            ),
          ),
        ),
      );

      expect(
        find.byKey(const ValueKey('accessible-route-list')),
        findsOneWidget,
      );

      const expectedNodeIds = <String>['Input 1', 'Drive 1', 'Output 1'];
      for (var index = 0; index < expectedNodeIds.length; index += 1) {
        final node = tester.getSemantics(
          find.byKey(ValueKey('accessible-route-node-$index')),
        );

        expect(node.label, startsWith(expectedNodeIds[index]));
        expect(node.label, contains('Incoming:'));
        expect(node.label, contains('Outgoing:'));
        expect(node.sortKey, isA<OrdinalSortKey>());
        expect((node.sortKey! as OrdinalSortKey).order, index);
      }

      expect(
        tester
            .getSemantics(find.byKey(const ValueKey('accessible-route-node-0')))
            .label,
        contains('Drive 1'),
      );
      expect(
        tester
            .getSemantics(find.byKey(const ValueKey('accessible-route-node-1')))
            .label,
        allOf(contains('Input 1'), contains('Output 1')),
      );
      expect(
        tester
            .getSemantics(find.byKey(const ValueKey('accessible-route-node-2')))
            .label,
        contains('Drive 1'),
      );

      for (var index = 0; index < snapshot.connections.length; index += 1) {
        final connection = snapshot.connections[index];
        final node = tester.getSemantics(
          find.byKey(ValueKey('accessible-route-connection-$index')),
        );
        final data = node.getSemanticsData();

        expect(
          node.label,
          'Connection from ${connection.sourceNodeId} ${connection.sourcePortId} '
          'to ${connection.destinationNodeId} ${connection.destinationPortId}',
        );
        expect(data.flagsCollection.isButton, isTrue);
        expect(data.flagsCollection.isFocused, isNot(ui.Tristate.none));
        expect(
          data.customSemanticsActionIds
              ?.map(CustomSemanticsAction.getAction)
              .whereType<CustomSemanticsAction>()
              .map((action) => action.label),
          contains('Remove connection'),
        );

        final removeActionId = data.customSemanticsActionIds!.firstWhere(
          (id) =>
              CustomSemanticsAction.getAction(id)?.label == 'Remove connection',
        );
        node.owner!.performAction(
          node.id,
          ui.SemanticsAction.customAction,
          removeActionId,
        );
      }

      expect(removedConnections, snapshot.connections);
    } finally {
      semantics.dispose();
    }
  });
}
