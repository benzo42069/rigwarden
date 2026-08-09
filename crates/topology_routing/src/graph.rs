#[path = "connection.rs"]
mod connection;
#[path = "validation.rs"]
mod validation;
pub use connection::{Connection, Port, PortDirection, PortId, PortRef};

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub use validation::GraphPolicy;

/// Stable identity for a node in a routing graph.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId(String);

impl NodeId {
    /// Construct an identity while preserving the supplied text.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the original identity text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for NodeId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A node's row and column in the editor's logical grid.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GridPosition {
    row: u32,
    column: u32,
}

impl GridPosition {
    /// Construct a grid position.
    pub const fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }

    /// Return the row component.
    pub const fn row(self) -> u32 {
        self.row
    }

    /// Return the column component.
    pub const fn column(self) -> u32 {
        self.column
    }
}

/// A graph node with identity independent from its current position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    id: NodeId,
    position: GridPosition,
    ports: BTreeMap<PortId, Port>,
}

impl Node {
    /// Construct a node from its stable identity and initial position.
    pub fn new(id: NodeId, position: GridPosition) -> Self {
        Self {
            id,
            position,
            ports: BTreeMap::new(),
        }
    }

    /// Construct a node and add its ports, rejecting duplicate port IDs.
    pub fn with_ports(
        id: NodeId,
        position: GridPosition,
        ports: impl IntoIterator<Item = Port>,
    ) -> Result<Self, GraphError> {
        let mut node = Self::new(id, position);
        for port in ports {
            node.add_port(port)?;
        }
        Ok(node)
    }

    /// Add a port without replacing an existing port identity.
    pub fn add_port(&mut self, port: Port) -> Result<(), GraphError> {
        let port_id = port.id().clone();
        if self.ports.contains_key(&port_id) {
            return Err(GraphError::DuplicatePortId {
                node_id: self.id.clone(),
                port_id,
            });
        }
        self.ports.insert(port_id, port);
        Ok(())
    }

    /// Return the stable identity.
    pub fn id(&self) -> &NodeId {
        &self.id
    }

    /// Return the current grid position.
    pub fn position(&self) -> GridPosition {
        self.position
    }

    /// Look up one port by its stable identity.
    pub fn port(&self, id: &PortId) -> Option<&Port> {
        self.ports.get(id)
    }

    /// Iterate over the node's ports in stable identity order.
    pub fn ports(&self) -> impl Iterator<Item = &Port> {
        self.ports.values()
    }
}

/// Errors produced while mutating a routing graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphError {
    /// An insertion would replace an existing node with the same identity.
    DuplicateNodeId(NodeId),
    /// A move referred to an identity that is not in the graph.
    NodeNotFound(NodeId),
    /// A node already contains a port with the requested identity.
    DuplicatePortId { node_id: NodeId, port_id: PortId },
    /// A connection referred to a node port that is not present.
    PortNotFound { node_id: NodeId, port_id: PortId },
    /// A connection endpoint has the wrong direction for its role.
    PortDirectionMismatch {
        endpoint: PortRef,
        expected: PortDirection,
        actual: PortDirection,
    },
    /// The exact directed connection already exists.
    DuplicateConnection(Connection),
    /// A connection would introduce a prohibited directed cycle.
    CycleDetected { path: Vec<NodeId> },
}

impl fmt::Display for GraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateNodeId(id) => write!(formatter, "duplicate node ID: {id}"),
            Self::NodeNotFound(id) => write!(formatter, "node not found: {id}"),
            Self::DuplicatePortId { node_id, port_id } => {
                write!(formatter, "duplicate port ID {port_id} on node {node_id}")
            }
            Self::PortNotFound { node_id, port_id } => {
                write!(formatter, "port {port_id} not found on node {node_id}")
            }
            Self::PortDirectionMismatch {
                endpoint,
                expected,
                actual,
            } => write!(
                formatter,
                "port {endpoint} has direction {actual}; expected {expected}"
            ),
            Self::DuplicateConnection(connection) => write!(
                formatter,
                "duplicate connection from {} to {}",
                connection.source(),
                connection.destination()
            ),
            Self::CycleDetected { path } => write!(
                formatter,
                "routing cycle detected: {}",
                path.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" -> ")
            ),
        }
    }
}

impl std::error::Error for GraphError {}

/// A routing graph containing nodes keyed by stable identity.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Graph {
    nodes: BTreeMap<NodeId, Node>,
    connections: BTreeSet<Connection>,
    policy: GraphPolicy,
}

impl Graph {
    /// Construct an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct an empty graph with an explicit validation policy.
    pub fn with_policy(policy: GraphPolicy) -> Self {
        Self {
            nodes: BTreeMap::new(),
            connections: BTreeSet::new(),
            policy,
        }
    }

    /// Insert a node without replacing an existing identity.
    pub fn insert(&mut self, node: Node) -> Result<(), GraphError> {
        let id = node.id.clone();
        if self.nodes.contains_key(&id) {
            return Err(GraphError::DuplicateNodeId(id));
        }

        self.nodes.insert(id, node);
        Ok(())
    }

    /// Move a node while retaining its stable identity.
    pub fn move_node(&mut self, id: &NodeId, position: GridPosition) -> Result<(), GraphError> {
        let node = self
            .nodes
            .get_mut(id)
            .ok_or_else(|| GraphError::NodeNotFound(id.clone()))?;
        node.position = position;
        Ok(())
    }

    /// Add one directed output-to-input connection after validating both endpoints.
    ///
    /// Exact duplicates are rejected so callers cannot accidentally count one
    /// signal path more than once.
    pub fn connect(&mut self, source: PortRef, destination: PortRef) -> Result<(), GraphError> {
        let source_direction = self.port_direction(&source)?;
        if source_direction != PortDirection::Output {
            return Err(GraphError::PortDirectionMismatch {
                endpoint: source,
                expected: PortDirection::Output,
                actual: source_direction,
            });
        }

        let destination_direction = self.port_direction(&destination)?;
        if destination_direction != PortDirection::Input {
            return Err(GraphError::PortDirectionMismatch {
                endpoint: destination,
                expected: PortDirection::Input,
                actual: destination_direction,
            });
        }

        let connection = Connection::new(source, destination);
        if self.connections.contains(&connection) {
            return Err(GraphError::DuplicateConnection(connection));
        }

        if self.policy.rejects_cycles()
            && let Some(path) = validation::cycle_path(&self.connections, &connection)
        {
            return Err(GraphError::CycleDetected { path });
        }

        self.connections.insert(connection);
        Ok(())
    }

    /// Return all connections leaving the given endpoint in stable order.
    pub fn outgoing_connections(&self, source: &PortRef) -> Vec<Connection> {
        self.connections
            .iter()
            .filter(|connection| connection.source() == source)
            .cloned()
            .collect()
    }

    /// Return all connections entering the given endpoint in stable order.
    pub fn incoming_connections(&self, destination: &PortRef) -> Vec<Connection> {
        self.connections
            .iter()
            .filter(|connection| connection.destination() == destination)
            .cloned()
            .collect()
    }

    /// Iterate over every stored connection in stable endpoint order.
    pub fn connections(&self) -> impl Iterator<Item = &Connection> {
        self.connections.iter()
    }

    fn port_direction(&self, endpoint: &PortRef) -> Result<PortDirection, GraphError> {
        let node = self
            .nodes
            .get(endpoint.node_id())
            .ok_or_else(|| GraphError::NodeNotFound(endpoint.node_id().clone()))?;
        node.port(endpoint.port_id())
            .map(Port::direction)
            .ok_or_else(|| GraphError::PortNotFound {
                node_id: endpoint.node_id().clone(),
                port_id: endpoint.port_id().clone(),
            })
    }

    /// Look up a node by stable identity.
    pub fn node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }
}
