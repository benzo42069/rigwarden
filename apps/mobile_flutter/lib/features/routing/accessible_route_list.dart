import 'package:flutter/material.dart';
import 'package:flutter/semantics.dart';
import 'package:rigwarden/core/bridge/generated/api.dart';

typedef AccessibleRouteRemoveCallback =
    void Function(SerialRouteConnection connection);

/// A nonvisual representation of the Rust-authored route snapshot.
///
/// The snapshot already contains the Rust traversal order and connection
/// context. This widget only formats that typed data for Flutter semantics; it
/// does not inspect or rebuild a routing graph.
class AccessibleRouteList extends StatelessWidget {
  const AccessibleRouteList({
    required this.snapshot,
    required this.onRemoveConnection,
    super.key,
  });

  final SerialRouteSnapshot snapshot;
  final AccessibleRouteRemoveCallback onRemoveConnection;

  @override
  Widget build(BuildContext context) {
    return Semantics(
      container: true,
      explicitChildNodes: true,
      label: 'Accessible signal route',
      child: ListView(
        key: const ValueKey('accessible-route-list'),
        padding: const EdgeInsets.all(16),
        children: [
          for (var index = 0; index < snapshot.nodes.length; index += 1)
            _AccessibleRouteNode(
              key: ValueKey('accessible-route-node-$index'),
              node: snapshot.nodes[index],
              index: index,
              allConnections: snapshot.connections,
              onRemoveConnection: onRemoveConnection,
            ),
        ],
      ),
    );
  }
}

class _AccessibleRouteNode extends StatelessWidget {
  const _AccessibleRouteNode({
    required this.node,
    required this.index,
    required this.allConnections,
    required this.onRemoveConnection,
    super.key,
  });

  final SerialRouteNode node;
  final int index;
  final List<SerialRouteConnection> allConnections;
  final AccessibleRouteRemoveCallback onRemoveConnection;

  @override
  Widget build(BuildContext context) {
    return Semantics(
      container: true,
      explicitChildNodes: true,
      focusable: true,
      label: _nodeContextLabel(node),
      sortKey: OrdinalSortKey(index.toDouble()),
      child: Padding(
        padding: const EdgeInsets.only(bottom: 16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            ExcludeSemantics(child: Text(node.id)),
            for (var connectionIndex = 0;
                connectionIndex < node.outgoingConnections.length;
                connectionIndex += 1)
              _AccessibleRouteConnection(
                connection: node.outgoingConnections[connectionIndex],
                key: ValueKey(
                  'accessible-route-connection-${allConnections.indexOf(node.outgoingConnections[connectionIndex])}',
                ),
                onRemoveConnection: onRemoveConnection,
              ),
          ],
        ),
      ),
    );
  }

}

class _AccessibleRouteConnection extends StatelessWidget {
  const _AccessibleRouteConnection({
    required this.connection,
    required this.onRemoveConnection,
    super.key,
  });

  static const _removeAction = CustomSemanticsAction(label: 'Remove connection');

  final SerialRouteConnection connection;
  final AccessibleRouteRemoveCallback onRemoveConnection;

  @override
  Widget build(BuildContext context) {
    return Semantics(
      container: true,
      focusable: true,
      button: true,
      label: _connectionLabel(connection),
      customSemanticsActions: <CustomSemanticsAction, VoidCallback>{
        _removeAction: () => onRemoveConnection(connection),
      },
      child: Padding(
        padding: const EdgeInsets.only(top: 8),
        child: ExcludeSemantics(
          child: Text(
            '${connection.sourceNodeId} ${connection.sourcePortId} → '
            '${connection.destinationNodeId} ${connection.destinationPortId}',
          ),
        ),
      ),
    );
  }
}

String _nodeContextLabel(SerialRouteNode node) {
  final incoming = _connectionContext(node.incomingConnections);
  final outgoing = _connectionContext(node.outgoingConnections);
  return '${node.id}. Incoming: $incoming. Outgoing: $outgoing.';
}

String _connectionContext(List<SerialRouteConnection> connections) {
  if (connections.isEmpty) {
    return 'none';
  }
  return connections
      .map(
        (connection) =>
            '${connection.sourceNodeId} ${connection.sourcePortId} to '
            '${connection.destinationNodeId} ${connection.destinationPortId}',
      )
      .join('; ');
}

String _connectionLabel(SerialRouteConnection connection) {
  return 'Connection from ${connection.sourceNodeId} ${connection.sourcePortId} '
      'to ${connection.destinationNodeId} ${connection.destinationPortId}';
}
