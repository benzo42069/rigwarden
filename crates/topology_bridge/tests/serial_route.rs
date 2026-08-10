use topology_bridge::api::{read_fixture_serial_route_snapshot, SerialRouteConnection};

#[test]
fn rust_authored_serial_route_snapshot_is_stable() {
    let first = read_fixture_serial_route_snapshot();
    let second = read_fixture_serial_route_snapshot();

    assert_eq!(first, second, "the Rust snapshot must be deterministic");
    assert_eq!(
        first
            .nodes
            .iter()
            .map(|node| node.id.as_str())
            .collect::<Vec<_>>(),
        vec!["Input 1", "Drive 1", "Output 1"],
        "nodes must follow Rust topological traversal order"
    );

    let input_to_drive = SerialRouteConnection {
        source_node_id: "Input 1".to_owned(),
        source_port_id: "out".to_owned(),
        destination_node_id: "Drive 1".to_owned(),
        destination_port_id: "in".to_owned(),
    };
    let drive_to_output = SerialRouteConnection {
        source_node_id: "Drive 1".to_owned(),
        source_port_id: "out".to_owned(),
        destination_node_id: "Output 1".to_owned(),
        destination_port_id: "in".to_owned(),
    };

    assert_eq!(
        first.connections,
        vec![input_to_drive.clone(), drive_to_output.clone()]
    );
    assert!(first.nodes[0].incoming_connections.is_empty());
    assert_eq!(
        first.nodes[0].outgoing_connections,
        vec![input_to_drive.clone()]
    );
    assert_eq!(first.nodes[1].incoming_connections, vec![input_to_drive]);
    assert_eq!(
        first.nodes[1].outgoing_connections,
        vec![drive_to_output.clone()]
    );
    assert_eq!(first.nodes[2].incoming_connections, vec![drive_to_output]);
    assert!(first.nodes[2].outgoing_connections.is_empty());
}
