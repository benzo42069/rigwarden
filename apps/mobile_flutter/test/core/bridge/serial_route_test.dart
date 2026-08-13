import 'package:flutter_test/flutter_test.dart';
import 'package:rigwarden/core/bridge/generated/api.dart';
import 'package:rigwarden/core/bridge/generated/frb_generated.dart';

void main() {
  test('typed serial route snapshot round trips from Rust', () async {
    await RustLib.init();

    final snapshot = await readFixtureSerialRouteSnapshot();

    expect(snapshot.nodes.map((node) => node.id).toList(), [
      'Input 1',
      'Drive 1',
      'Output 1',
    ]);
    expect(
      snapshot.connections
          .map(
            (connection) =>
                '${connection.sourceNodeId}:${connection.sourcePortId}'
                '->${connection.destinationNodeId}:${connection.destinationPortId}',
          )
          .toList(),
      ['Input 1:out->Drive 1:in', 'Drive 1:out->Output 1:in'],
    );

    expect(snapshot.nodes[0].incomingConnections, isEmpty);
    expect(
      snapshot.nodes[0].outgoingConnections.single.sourceNodeId,
      'Input 1',
    );
    expect(
      snapshot.nodes[0].outgoingConnections.single.destinationNodeId,
      'Drive 1',
    );
    expect(
      snapshot.nodes[1].incomingConnections.single.sourceNodeId,
      'Input 1',
    );
    expect(
      snapshot.nodes[1].incomingConnections.single.destinationNodeId,
      'Drive 1',
    );
    expect(
      snapshot.nodes[1].outgoingConnections.single.sourceNodeId,
      'Drive 1',
    );
    expect(
      snapshot.nodes[1].outgoingConnections.single.destinationNodeId,
      'Output 1',
    );
    expect(
      snapshot.nodes[2].incomingConnections.single.sourceNodeId,
      'Drive 1',
    );
    expect(
      snapshot.nodes[2].incomingConnections.single.destinationNodeId,
      'Output 1',
    );
    expect(snapshot.nodes[2].outgoingConnections, isEmpty);
  });
}
