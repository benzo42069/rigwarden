use topology_command_engine::{GraphMutation, plan_graph_mutations};

fn mutation(id: &str, dependencies: &[&str]) -> GraphMutation {
    dependencies
        .iter()
        .fold(GraphMutation::new(id), |mutation, dependency| {
            mutation.depends_on(*dependency)
        })
}

fn operation_ids(plan: &topology_command_engine::SemanticCommandPlan) -> Vec<&str> {
    plan.operations()
        .iter()
        .map(GraphMutation::operation_id)
        .collect()
}

#[test]
fn equivalent_mutation_sets_produce_same_operation_order() {
    let first = [
        mutation("output", &["branch-b", "branch-a"]),
        mutation("branch-b", &["split"]),
        mutation("split", &["input"]),
        mutation("branch-a", &["split"]),
        mutation("input", &[]),
    ];
    let second = [
        mutation("input", &[]),
        mutation("branch-a", &["split"]),
        mutation("split", &["input"]),
        mutation("branch-b", &["split"]),
        mutation("output", &["branch-a", "branch-b"]),
    ];

    let first_plan = plan_graph_mutations(first).expect("first semantic set should be valid");
    let second_plan = plan_graph_mutations(second).expect("second semantic set should be valid");

    let first_ids = operation_ids(&first_plan);
    let second_ids = operation_ids(&second_plan);

    assert_eq!(
        first_ids, second_ids,
        "equivalent mutation sets must use the same stable operation order"
    );
    assert_eq!(
        first_ids,
        ["input", "split", "branch-a", "branch-b", "output"]
    );

    for (dependency, dependent) in [
        ("input", "split"),
        ("split", "branch-a"),
        ("split", "branch-b"),
        ("branch-a", "output"),
        ("branch-b", "output"),
    ] {
        let dependency_position = first_ids
            .iter()
            .position(|operation_id| *operation_id == dependency)
            .expect("dependency must be planned");
        let dependent_position = first_ids
            .iter()
            .position(|operation_id| *operation_id == dependent)
            .expect("dependent must be planned");
        assert!(
            dependency_position < dependent_position,
            "dependency {dependency} must precede dependent {dependent}"
        );
    }
}
