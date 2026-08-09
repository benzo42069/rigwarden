use std::collections::BTreeMap;
use std::fmt;

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
}

impl Node {
    /// Construct a node from its stable identity and initial position.
    pub fn new(id: NodeId, position: GridPosition) -> Self {
        Self { id, position }
    }

    /// Return the stable identity.
    pub fn id(&self) -> &NodeId {
        &self.id
    }

    /// Return the current grid position.
    pub fn position(&self) -> GridPosition {
        self.position
    }
}

/// Errors produced while mutating a routing graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphError {
    /// An insertion would replace an existing node with the same identity.
    DuplicateNodeId(NodeId),
    /// A move referred to an identity that is not in the graph.
    NodeNotFound(NodeId),
}

impl fmt::Display for GraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateNodeId(id) => write!(formatter, "duplicate node ID: {id}"),
            Self::NodeNotFound(id) => write!(formatter, "node not found: {id}"),
        }
    }
}

impl std::error::Error for GraphError {}

/// A routing graph containing nodes keyed by stable identity.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Graph {
    nodes: BTreeMap<NodeId, Node>,
}

impl Graph {
    /// Construct an empty graph.
    pub fn new() -> Self {
        Self::default()
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

    /// Look up a node by stable identity.
    pub fn node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }
}
