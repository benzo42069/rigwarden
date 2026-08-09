use crate::graph::{Graph, NodeId};
use std::collections::{BTreeMap, BTreeSet};

impl Graph {
    /// Return every node in deterministic topological order.
    ///
    /// The stable node identity is the tie-breaker whenever multiple nodes are
    /// ready. For an acyclic graph, every dependency therefore precedes each
    /// destination and every node appears exactly once. A graph policy that
    /// permits cycles can still be traversed, but no topological order exists
    /// for its cyclic portion; unresolved nodes are appended in stable identity
    /// order without changing the graph's existing cycle policy.
    pub fn topological_traversal(&self) -> Vec<NodeId> {
        let mut indegree = self
            .node_ids()
            .cloned()
            .map(|id| (id, 0_usize))
            .collect::<BTreeMap<_, _>>();
        let mut outgoing = BTreeMap::<NodeId, BTreeMap<NodeId, usize>>::new();

        for connection in self.connections() {
            let source = connection.source().node_id().clone();
            let destination = connection.destination().node_id().clone();
            *indegree
                .get_mut(&destination)
                .expect("stored connections must reference graph nodes") += 1;
            *outgoing
                .entry(source)
                .or_default()
                .entry(destination)
                .or_default() += 1;
        }

        let mut ready = indegree
            .iter()
            .filter_map(|(id, degree)| (*degree == 0).then_some(id.clone()))
            .collect::<BTreeSet<_>>();
        let mut visited = BTreeSet::new();
        let mut traversal = Vec::with_capacity(indegree.len());

        while let Some(node) = ready.pop_first() {
            visited.insert(node.clone());
            traversal.push(node.clone());

            if let Some(destinations) = outgoing.get(&node) {
                for (destination, edge_count) in destinations {
                    let degree = indegree
                        .get_mut(destination)
                        .expect("stored connections must reference graph nodes");
                    *degree -= edge_count;
                    if *degree == 0 {
                        ready.insert(destination.clone());
                    }
                }
            }
        }

        // Acyclic graphs are fully consumed above. If the graph policy allows
        // cycles, retain the one-appearance invariant without inventing a new
        // cycle error or changing the existing insertion behavior.
        for id in indegree.keys() {
            if visited.insert(id.clone()) {
                traversal.push(id.clone());
            }
        }

        traversal
    }
}
