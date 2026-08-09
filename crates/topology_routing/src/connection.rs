use super::NodeId;
use std::fmt;

/// Stable identity for one node port.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PortId(String);

impl PortId {
    /// Construct a port identity while preserving the supplied text.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the original identity text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PortId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PortId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Direction of a port in the signal flow.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PortDirection {
    Input,
    Output,
}

impl fmt::Display for PortDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input => formatter.write_str("input"),
            Self::Output => formatter.write_str("output"),
        }
    }
}

/// A typed node port and its signal-flow direction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Port {
    id: PortId,
    direction: PortDirection,
}

impl Port {
    /// Construct a port with a stable identity and direction.
    pub fn new(id: PortId, direction: PortDirection) -> Self {
        Self { id, direction }
    }

    /// Return the stable port identity.
    pub fn id(&self) -> &PortId {
        &self.id
    }

    /// Return the signal-flow direction.
    pub const fn direction(&self) -> PortDirection {
        self.direction
    }
}

/// A fully qualified reference to a node port.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PortRef {
    node_id: NodeId,
    port_id: PortId,
}

impl PortRef {
    /// Construct a reference from a node and port identity.
    pub fn new(node_id: NodeId, port_id: PortId) -> Self {
        Self { node_id, port_id }
    }

    /// Return the referenced node identity.
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    /// Return the referenced port identity.
    pub fn port_id(&self) -> &PortId {
        &self.port_id
    }
}

impl fmt::Display for PortRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.node_id, self.port_id)
    }
}

/// A directed connection from one output port to one input port.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Connection {
    source: PortRef,
    destination: PortRef,
}

impl Connection {
    /// Construct a connection between two qualified port references.
    pub fn new(source: PortRef, destination: PortRef) -> Self {
        Self {
            source,
            destination,
        }
    }

    /// Return the output endpoint.
    pub fn source(&self) -> &PortRef {
        &self.source
    }

    /// Return the input endpoint.
    pub fn destination(&self) -> &PortRef {
        &self.destination
    }
}
