use topology_devtools::work_item::validate_yaml;

#[test]
fn missing_id_is_rejected() {
    let packet_without_id = r#"
kind: implementation
epic: TOP-EXAMPLE
title: Replace with one observable behavior
status: READY
priority: high
requirement_ids: []
depends_on: []
parallel_group: null
agent:
  preferred_role: topology_implementer
  reviewer_role: topology_reviewer
  integrator_role: parent_orchestrator
scope:
  read: []
  write: []
  forbidden: []
behavior:
  why: Test packet
non_goals: []
preconditions: []
acceptance:
  - Test packet is valid
evidence_directory: .tdd/evidence/TOP-EXAMPLE-001
test:
  layer: rust_unit
  file: test.rs
  name: test
  red_command: cargo test
  green_command: cargo test
minimum_green: []
required_sweeps: []
available_claims_after_completion: []
unavailable_claims: []
"#;

    let errors = validate_yaml(packet_without_id).expect_err("missing id must be rejected");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].code, "missing_field");
    assert_eq!(errors[0].path, "id");
}

#[test]
fn valid_companion_packet_is_accepted() {
    let valid_packet = r#"
id: TOP-EXAMPLE-001
kind: implementation
epic: TOP-EXAMPLE
title: Replace with one observable behavior
status: READY
priority: high
requirement_ids: []
depends_on: []
parallel_group: null
agent:
  preferred_role: topology_implementer
  reviewer_role: topology_reviewer
  integrator_role: parent_orchestrator
scope:
  read: []
  write: []
  forbidden: []
behavior:
  why: Test packet
non_goals: []
preconditions: []
acceptance:
  - Test packet is valid
evidence_directory: .tdd/evidence/TOP-EXAMPLE-001
test:
  layer: rust_unit
  file: test.rs
  name: test
  red_command: cargo test
  green_command: cargo test
minimum_green: []
required_sweeps: []
available_claims_after_completion: []
unavailable_claims: []
"#;

    validate_yaml(valid_packet).expect("valid companion packet must be accepted");
}
