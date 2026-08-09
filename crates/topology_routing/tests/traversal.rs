use topology_routing::graph::{
    Graph, GridPosition, Node, NodeId, Port, PortDirection, PortId, PortRef,
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

fn graph_with_insertion_order(node_order: &[&str], connection_order: &[(&str, &str)]) -> Graph {
    let mut graph = Graph::new();
    for (column, id) in node_order.iter().enumerate() {
        graph
            .insert(node(id, column as u32))
            .expect("node insertion should succeed");
    }
    for (source, destination) in connection_order {
        graph
            .connect(port(source, "out"), port(destination, "in"))
            .expect("test connection should be valid");
    }
    graph
}

#[test]
fn topological_traversal_is_deterministic_across_insertion_order() {
    let first = graph_with_insertion_order(
        &[
            "output", "branch-b", "split", "branch-a", "input", "isolated",
        ],
        &[
            ("input", "split"),
            ("split", "branch-b"),
            ("split", "branch-a"),
            ("branch-b", "output"),
            ("branch-a", "output"),
        ],
    );
    let second = graph_with_insertion_order(
        &[
            "input", "branch-a", "branch-b", "split", "output", "isolated",
        ],
        &[
            ("branch-a", "output"),
            ("split", "branch-a"),
            ("input", "split"),
            ("branch-b", "output"),
            ("split", "branch-b"),
        ],
    );

    let first_order = first.topological_traversal();
    let second_order = second.topological_traversal();

    assert_eq!(
        first_order, second_order,
        "equivalent graphs must produce the same traversal regardless of insertion order"
    );
    assert_eq!(
        first_order,
        vec![
            NodeId::new("input"),
            NodeId::new("isolated"),
            NodeId::new("split"),
            NodeId::new("branch-a"),
            NodeId::new("branch-b"),
            NodeId::new("output"),
        ],
        "stable node identity order is the tie-breaker for ready branches"
    );
    assert_eq!(
        first_order.len(),
        6,
        "a traversal must contain every graph node exactly once"
    );
    let unique_nodes = first_order
        .iter()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(unique_nodes.len(), first_order.len());

    for (source, destination) in [
        ("input", "split"),
        ("split", "branch-a"),
        ("split", "branch-b"),
        ("branch-a", "output"),
        ("branch-b", "output"),
    ] {
        let source_position = first_order
            .iter()
            .position(|id| id == &NodeId::new(source))
            .expect("source must appear in traversal");
        let destination_position = first_order
            .iter()
            .position(|id| id == &NodeId::new(destination))
            .expect("destination must appear in traversal");
        assert!(
            source_position < destination_position,
            "dependency {source} must precede destination {destination}"
        );
    }
}
