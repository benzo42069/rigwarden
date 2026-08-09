use crate::graph::{Connection, NodeId};
use std::collections::{BTreeMap, BTreeSet};

/// Policy controlling whether a graph may contain directed feedback.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GraphPolicy {
    /// Preserve the historical graph behavior and permit directed cycles.
    #[default]
    AllowCycles,
    /// Reject a connection when it would close a directed cycle.
    RejectCycles,
}

impl GraphPolicy {
    pub(crate) const fn rejects_cycles(self) -> bool {
        matches!(self, Self::RejectCycles)
    }
}

/// Find the deterministic node path created by adding `connection`.
pub(crate) fn cycle_path(
    connections: &BTreeSet<Connection>,
    connection: &Connection,
) -> Option<Vec<NodeId>> {
    let source = connection.source().node_id();
    let destination = connection.destination().node_id();
    if source == destination {
        return Some(vec![source.clone(), destination.clone()]);
    }

    let mut adjacency: BTreeMap<NodeId, BTreeSet<NodeId>> = BTreeMap::new();
    for existing in connections {
        adjacency
            .entry(existing.source().node_id().clone())
            .or_default()
            .insert(existing.destination().node_id().clone());
    }

    let mut visited = BTreeSet::new();
    let return_path = path_to_target(destination, source, &adjacency, &mut visited)?;
    let mut cycle = vec![source.clone()];
    cycle.extend(return_path);
    Some(cycle)
}

fn path_to_target(
    current: &NodeId,
    target: &NodeId,
    adjacency: &BTreeMap<NodeId, BTreeSet<NodeId>>,
    visited: &mut BTreeSet<NodeId>,
) -> Option<Vec<NodeId>> {
    if current == target {
        return Some(vec![current.clone()]);
    }
    if !visited.insert(current.clone()) {
        return None;
    }

    for next in adjacency.get(current).into_iter().flatten() {
        if let Some(mut path) = path_to_target(next, target, adjacency, visited) {
            let mut result = vec![current.clone()];
            result.append(&mut path);
            return Some(result);
        }
    }
    None
}
