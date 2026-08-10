use topology_domain::{
    DeviceFamilyId, DeviceIdentity as DomainDeviceIdentity, DeviceModelId, FirmwareId,
    TransportEndpointId,
};
use topology_routing::graph::{
    Connection, Graph, GridPosition, Node, NodeId, Port, PortDirection, PortId, PortRef,
};

/// Configure flutter_rust_bridge's generated runtime.
#[flutter_rust_bridge::frb(init)]
pub fn init_rigwarden_bridge() {
    flutter_rust_bridge::setup_default_user_utils();
}

/// A read-only bridge handle for the validated domain identity.
///
/// The wrapped domain value remains the source of truth; this type exposes
/// only copied display fields and has no Dart-side constructor or mutation.
#[flutter_rust_bridge::frb(opaque)]
pub struct DeviceIdentityHandle(DomainDeviceIdentity);

impl DeviceIdentityHandle {
    #[flutter_rust_bridge::frb(sync, getter)]
    pub fn family(&self) -> String {
        self.0.family().as_str().to_owned()
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    pub fn model(&self) -> String {
        self.0.model().as_str().to_owned()
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    pub fn firmware(&self) -> String {
        self.0.firmware().as_str().to_owned()
    }

    #[flutter_rust_bridge::frb(sync, getter)]
    pub fn transport_endpoint(&self) -> String {
        self.0.transport_endpoint().as_str().to_owned()
    }
}

/// Return one validated identity fixture through the typed Rust-Dart bridge.
pub fn read_fixture_device_identity() -> DeviceIdentityHandle {
    DeviceIdentityHandle(DomainDeviceIdentity::new(
        DeviceFamilyId::new("AM4").expect("fixture family is nonblank"),
        DeviceModelId::new("AM4").expect("fixture model is nonblank"),
        FirmwareId::new("1.00").expect("fixture firmware is nonblank"),
        TransportEndpointId::new("fixture://am4").expect("fixture endpoint is nonblank"),
    ))
}

/// One read-only connection context in a Rust-authored route snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerialRouteConnection {
    pub source_node_id: String,
    pub source_port_id: String,
    pub destination_node_id: String,
    pub destination_port_id: String,
}

/// One ordered node and its incoming/outgoing connection context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerialRouteNode {
    pub id: String,
    pub incoming_connections: Vec<SerialRouteConnection>,
    pub outgoing_connections: Vec<SerialRouteConnection>,
}

/// A deterministic, read-only route snapshot for the nonvisual Flutter view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerialRouteSnapshot {
    pub nodes: Vec<SerialRouteNode>,
    pub connections: Vec<SerialRouteConnection>,
}

/// Build the synthetic Input 1 -> Drive 1 -> Output 1 graph using Rust's
/// routing APIs and expose only its deterministic read-only snapshot.
pub fn read_fixture_serial_route_snapshot() -> SerialRouteSnapshot {
    let mut graph = Graph::new();
    graph
        .insert(serial_node(
            "Output 1",
            GridPosition::new(0, 2),
            [Port::new(PortId::new("in"), PortDirection::Input)],
        ))
        .expect("fixture output node should insert");
    graph
        .insert(serial_node(
            "Drive 1",
            GridPosition::new(0, 1),
            [
                Port::new(PortId::new("in"), PortDirection::Input),
                Port::new(PortId::new("out"), PortDirection::Output),
            ],
        ))
        .expect("fixture drive node should insert");
    graph
        .insert(serial_node(
            "Input 1",
            GridPosition::new(0, 0),
            [Port::new(PortId::new("out"), PortDirection::Output)],
        ))
        .expect("fixture input node should insert");

    graph
        .connect(serial_port("Input 1", "out"), serial_port("Drive 1", "in"))
        .expect("fixture input-to-drive connection should be valid");
    graph
        .connect(serial_port("Drive 1", "out"), serial_port("Output 1", "in"))
        .expect("fixture drive-to-output connection should be valid");

    let traversal = graph.topological_traversal();
    let connections = traversal
        .iter()
        .flat_map(|node_id| {
            graph
                .connections()
                .filter(move |connection| connection.source().node_id() == node_id)
                .map(serial_route_connection)
        })
        .collect::<Vec<_>>();
    let nodes = traversal
        .into_iter()
        .map(|id| {
            let incoming_connections = connections
                .iter()
                .filter(|connection| connection.destination_node_id == id.as_str())
                .cloned()
                .collect();
            let outgoing_connections = connections
                .iter()
                .filter(|connection| connection.source_node_id == id.as_str())
                .cloned()
                .collect();
            SerialRouteNode {
                id: id.as_str().to_owned(),
                incoming_connections,
                outgoing_connections,
            }
        })
        .collect();

    SerialRouteSnapshot { nodes, connections }
}

fn serial_node<const N: usize>(id: &str, position: GridPosition, ports: [Port; N]) -> Node {
    Node::with_ports(NodeId::new(id), position, ports).expect("fixture node ports should be unique")
}

fn serial_port(node_id: &str, port_id: &str) -> PortRef {
    PortRef::new(NodeId::new(node_id), PortId::new(port_id))
}

fn serial_route_connection(connection: &Connection) -> SerialRouteConnection {
    SerialRouteConnection {
        source_node_id: connection.source().node_id().as_str().to_owned(),
        source_port_id: connection.source().port_id().as_str().to_owned(),
        destination_node_id: connection.destination().node_id().as_str().to_owned(),
        destination_port_id: connection.destination().port_id().as_str().to_owned(),
    }
}
