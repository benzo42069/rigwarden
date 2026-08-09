use topology_routing::graph::{
    Graph, GraphError, GridPosition, Node, NodeId, Port, PortDirection, PortId, PortRef,
};

#[test]
fn missing_source_connection_is_rejected_without_mutation() {
    let source_node_id = NodeId::new("source-1");
    let destination_node_id = NodeId::new("destination-1");
    let source_port_id = PortId::new("out");
    let missing_source_port_id = PortId::new("missing-out");
    let destination_port_id = PortId::new("in");

    let source_node = Node::with_ports(
        source_node_id.clone(),
        GridPosition::new(0, 0),
        [Port::new(source_port_id, PortDirection::Output)],
    )
    .expect("source node ports should be unique");
    let destination_node = Node::with_ports(
        destination_node_id.clone(),
        GridPosition::new(0, 1),
        [Port::new(destination_port_id.clone(), PortDirection::Input)],
    )
    .expect("destination node ports should be unique");

    let mut graph = Graph::new();
    graph
        .insert(source_node)
        .expect("source node insertion should succeed");
    graph
        .insert(destination_node)
        .expect("destination node insertion should succeed");

    let missing_source = PortRef::new(source_node_id.clone(), missing_source_port_id.clone());
    let destination = PortRef::new(destination_node_id, destination_port_id);
    let result = graph.connect(missing_source, destination);

    assert!(
        matches!(
            result,
            Err(GraphError::PortNotFound { node_id, port_id })
                if node_id == source_node_id && port_id == missing_source_port_id
        ),
        "a missing source port must return its node and port identity"
    );
    assert!(
        graph.connections().next().is_none(),
        "a rejected connection must not mutate the graph"
    );
}
