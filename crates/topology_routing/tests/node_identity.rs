use topology_routing::graph::{Graph, GridPosition, Node, NodeId};

#[test]
fn moving_a_node_preserves_its_stable_identity() {
    let mut graph = Graph::new();
    let node_id = NodeId::new("amp-1");

    graph
        .insert(Node::new(node_id.clone(), GridPosition::new(1, 2)))
        .expect("first node insertion should succeed");

    graph
        .move_node(&node_id, GridPosition::new(4, 5))
        .expect("moving an existing node should succeed");

    let moved = graph
        .node(&node_id)
        .expect("moved node should remain present");
    assert_eq!(moved.id(), &node_id);
    assert_eq!(moved.position(), GridPosition::new(4, 5));

    let duplicate = graph.insert(Node::new(node_id, GridPosition::new(7, 8)));
    assert!(duplicate.is_err(), "duplicate node IDs must be rejected");
}
