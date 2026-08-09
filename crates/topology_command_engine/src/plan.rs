use std::collections::{BTreeMap, BTreeSet};

/// One semantic graph mutation identified independently of insertion order.
///
/// The operation ID is the stable ordering key. Dependencies refer to other
/// operation IDs and describe semantic prerequisites; they do not represent
/// protocol frames or transport work.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphMutation {
    operation_id: String,
    dependencies: BTreeSet<String>,
}

impl GraphMutation {
    /// Construct a mutation with no semantic dependencies.
    pub fn new(operation_id: impl Into<String>) -> Self {
        Self {
            operation_id: operation_id.into(),
            dependencies: BTreeSet::new(),
        }
    }

    /// Add one semantic dependency and return the updated mutation.
    pub fn depends_on(mut self, operation_id: impl Into<String>) -> Self {
        self.dependencies.insert(operation_id.into());
        self
    }

    /// Return the stable operation identity used for ordering.
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    /// Return semantic dependency IDs in stable identity order.
    pub fn dependencies(&self) -> impl Iterator<Item = &str> {
        self.dependencies.iter().map(String::as_str)
    }
}

/// A semantic command plan whose operations have deterministic order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticCommandPlan {
    operations: Vec<GraphMutation>,
}

impl SemanticCommandPlan {
    /// Return the planned semantic operations in execution order.
    pub fn operations(&self) -> &[GraphMutation] {
        &self.operations
    }

    /// Consume the plan and return its semantic operations in execution order.
    pub fn into_operations(self) -> Vec<GraphMutation> {
        self.operations
    }
}

/// Why a semantic mutation set cannot be planned.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanError {
    /// Two mutations supplied the same stable operation ID.
    DuplicateOperationId(String),
    /// A mutation refers to an operation ID that is not in the same set.
    MissingDependency {
        operation_id: String,
        dependency_id: String,
    },
    /// The dependency graph contains a cycle, so no dependency-respecting
    /// order exists for the listed operations.
    DependencyCycle { operation_ids: Vec<String> },
}

/// Plan semantic graph mutations in deterministic dependency order.
///
/// Kahn's algorithm is used for the dependency constraints. Whenever more
/// than one operation is ready, the lexicographically smallest stable
/// operation ID is selected. Consequently equivalent mutation sets produce
/// the same order regardless of insertion order while every dependency still
/// precedes its dependent. The plan contains semantic operations only; it
/// does not encode bytes or perform transport work.
pub fn plan_graph_mutations(
    mutations: impl IntoIterator<Item = GraphMutation>,
) -> Result<SemanticCommandPlan, PlanError> {
    let mut by_id = BTreeMap::<String, GraphMutation>::new();
    for mutation in mutations {
        let operation_id = mutation.operation_id.clone();
        if by_id.insert(operation_id.clone(), mutation).is_some() {
            return Err(PlanError::DuplicateOperationId(operation_id));
        }
    }

    for (operation_id, mutation) in &by_id {
        for dependency_id in mutation.dependencies() {
            if !by_id.contains_key(dependency_id) {
                return Err(PlanError::MissingDependency {
                    operation_id: operation_id.clone(),
                    dependency_id: dependency_id.to_owned(),
                });
            }
        }
    }

    let mut indegree = by_id
        .keys()
        .cloned()
        .map(|operation_id| (operation_id, 0_usize))
        .collect::<BTreeMap<_, _>>();
    let mut dependents = BTreeMap::<String, BTreeSet<String>>::new();

    for (operation_id, mutation) in &by_id {
        for dependency_id in mutation.dependencies() {
            if let Some(degree) = indegree.get_mut(operation_id) {
                *degree += 1;
            }
            dependents
                .entry(dependency_id.to_owned())
                .or_default()
                .insert(operation_id.clone());
        }
    }

    let mut ready = indegree
        .iter()
        .filter_map(|(operation_id, degree)| (*degree == 0).then_some(operation_id.clone()))
        .collect::<BTreeSet<_>>();
    let mut operations = Vec::with_capacity(by_id.len());

    while let Some(operation_id) = ready.pop_first() {
        if let Some(mutation) = by_id.get(&operation_id) {
            operations.push(mutation.clone());
        }

        if let Some(operation_dependents) = dependents.get(&operation_id) {
            for dependent_id in operation_dependents {
                if let Some(degree) = indegree.get_mut(dependent_id) {
                    *degree -= 1;
                    if *degree == 0 {
                        ready.insert(dependent_id.clone());
                    }
                }
            }
        }
    }

    if operations.len() != by_id.len() {
        let operation_ids = indegree
            .into_iter()
            .filter_map(|(operation_id, degree)| (degree > 0).then_some(operation_id))
            .collect();
        return Err(PlanError::DependencyCycle { operation_ids });
    }

    Ok(SemanticCommandPlan { operations })
}
