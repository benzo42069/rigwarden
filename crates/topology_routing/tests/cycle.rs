use topology_routing::graph::{
    Graph, GraphError, GraphPolicy, GridPosition, Node, NodeId, Port, PortDirection, PortId,
    PortRef,
};

fn node(id: &str, column: u32) -> Node {
    Node::with_ports(
        NodeId::new(id),
        GridPosition::new(0, column),
        [
            Port::new(PortId::new("in"), PortDirection::Input),
            Port::new(PortId::new("out"), PortDirection::Output),
        ],
    )
    .expect("test nodes should have unique ports")
}

fn port(node_id: &str, port_id: &str) -> PortRef {
    PortRef::new(NodeId::new(node_id), PortId::new(port_id))
}

#[test]
fn prohibited_cycle_is_rejected_without_mutating_graph() {
    let mut graph = Graph::with_policy(GraphPolicy::RejectCycles);
    graph.insert(node("A", 0)).expect("A should insert");
    graph.insert(node("B", 1)).expect("B should insert");
    graph.insert(node("C", 2)).expect("C should insert");

    graph
        .connect(port("A", "out"), port("B", "in"))
        .expect("A to B should be acyclic");
    graph
        .connect(port("B", "out"), port("C", "in"))
        .expect("B to C should be acyclic");
    let before_cycle = graph.connections().cloned().collect::<Vec<_>>();

    let result = graph.connect(port("C", "out"), port("A", "in"));

    assert!(
        matches!(
            result,
            Err(GraphError::CycleDetected { path })
                if path == vec![NodeId::new("C"), NodeId::new("A"), NodeId::new("B"), NodeId::new("C")]
        ),
        "a prohibited cycle must return its deterministic node path"
    );
    assert_eq!(
        graph.connections().cloned().collect::<Vec<_>>(),
        before_cycle,
        "a rejected cycle must leave existing connections unchanged"
    );
}
