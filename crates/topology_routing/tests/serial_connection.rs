use topology_routing::graph::{
    Graph, GraphError, GridPosition, Node, NodeId, Port, PortDirection, PortId, PortRef,
};

#[test]
fn valid_serial_connection_is_queryable_from_both_ends() {
    let input_node_id = NodeId::new("input-1");
    let amp_node_id = NodeId::new("amp-1");
    let input_port_id = PortId::new("out");
    let amp_port_id = PortId::new("in");

    let input_node = Node::with_ports(
        input_node_id.clone(),
        GridPosition::new(0, 0),
        [Port::new(input_port_id.clone(), PortDirection::Output)],
    )
    .expect("input node ports should be unique");
    let amp_node = Node::with_ports(
        amp_node_id.clone(),
        GridPosition::new(0, 1),
        [Port::new(amp_port_id.clone(), PortDirection::Input)],
    )
    .expect("amp node ports should be unique");

    let mut graph = Graph::new();
    graph
        .insert(input_node)
        .expect("input node insertion should succeed");
    graph
        .insert(amp_node)
        .expect("amp node insertion should succeed");

    let source = PortRef::new(input_node_id, input_port_id);
    let destination = PortRef::new(amp_node_id, amp_port_id);
    graph
        .connect(source.clone(), destination.clone())
        .expect("output-to-input connection should succeed");

    let outgoing = graph.outgoing_connections(&source);
    assert_eq!(outgoing.len(), 1);
    assert_eq!(outgoing[0].source(), &source);
    assert_eq!(outgoing[0].destination(), &destination);

    let incoming = graph.incoming_connections(&destination);
    assert_eq!(incoming.len(), 1);
    assert_eq!(incoming[0].source(), &source);
    assert_eq!(incoming[0].destination(), &destination);

    let duplicate = graph.connect(source.clone(), destination.clone());
    assert!(
        matches!(duplicate, Err(GraphError::DuplicateConnection(_))),
        "an exact duplicate must be rejected with a structured error"
    );
    assert_eq!(graph.outgoing_connections(&source).len(), 1);
}

#[test]
fn connection_direction_is_enforced_before_mutation() {
    let input_node_id = NodeId::new("input-1");
    let amp_node_id = NodeId::new("amp-1");
    let input_port_id = PortId::new("out");
    let amp_port_id = PortId::new("in");

    let input_node = Node::with_ports(
        input_node_id.clone(),
        GridPosition::new(0, 0),
        [Port::new(input_port_id.clone(), PortDirection::Output)],
    )
    .expect("input node ports should be unique");
    let amp_node = Node::with_ports(
        amp_node_id.clone(),
        GridPosition::new(0, 1),
        [Port::new(amp_port_id.clone(), PortDirection::Input)],
    )
    .expect("amp node ports should be unique");

    let mut graph = Graph::new();
    graph
        .insert(input_node)
        .expect("input node insertion should succeed");
    graph
        .insert(amp_node)
        .expect("amp node insertion should succeed");

    let source = PortRef::new(input_node_id, input_port_id);
    let destination = PortRef::new(amp_node_id, amp_port_id);
    let reversed = graph.connect(destination.clone(), source.clone());

    assert!(
        matches!(reversed, Err(GraphError::PortDirectionMismatch { .. })),
        "input-to-output connections must be rejected with a direction error"
    );
    assert!(
        graph.connections().next().is_none(),
        "rejected connections must not mutate the graph"
    );
}

#[test]
fn output_to_output_connection_is_rejected_before_mutation() {
    let source_node_id = NodeId::new("source-1");
    let destination_node_id = NodeId::new("destination-1");
    let source_port_id = PortId::new("out");
    let destination_port_id = PortId::new("out");

    let source_node = Node::with_ports(
        source_node_id.clone(),
        GridPosition::new(0, 0),
        [Port::new(source_port_id.clone(), PortDirection::Output)],
    )
    .expect("source node ports should be unique");
    let destination_node = Node::with_ports(
        destination_node_id.clone(),
        GridPosition::new(0, 1),
        [Port::new(
            destination_port_id.clone(),
            PortDirection::Output,
        )],
    )
    .expect("destination node ports should be unique");

    let mut graph = Graph::new();
    graph
        .insert(source_node)
        .expect("source node insertion should succeed");
    graph
        .insert(destination_node)
        .expect("destination node insertion should succeed");

    let source = PortRef::new(source_node_id, source_port_id);
    let destination = PortRef::new(destination_node_id, destination_port_id);
    let result = graph.connect(source, destination.clone());

    assert!(
        matches!(
            result,
            Err(GraphError::PortDirectionMismatch {
                expected: PortDirection::Input,
                actual: PortDirection::Output,
                endpoint,
            }) if endpoint == destination
        ),
        "output destination must produce a structured expected-input error"
    );
    assert!(
        graph.connections().next().is_none(),
        "rejected connections must not mutate the graph"
    );
}
