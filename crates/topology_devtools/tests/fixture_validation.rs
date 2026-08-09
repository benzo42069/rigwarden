use topology_devtools::fixture::validate_yaml;

fn sidecar(redistribution: &str) -> String {
    format!(
        r#"fixture_id: fm3.identity.v1
device:
  family: Fractal
  model: FM3
  variant: null
firmware: 1.0
transport: usb
direction: device_to_host
feature: identity
captured_at: null
source:
  category: simulator_fixture
  reference: test-fixture
  license: null
  commit: null
derivation: null
sanitization: []
redistribution:
{redistribution}
sha256: "0000000000000000000000000000000000000000000000000000000000000000"
expected: {{}}
confidence: simulator_only
verification_status: PLANNED
contributor: null
review:
  status: pending
  reviewer: null
  notes: null
"#,
        redistribution = redistribution,
    )
}

#[test]
fn fixture_without_redistribution_permission_is_rejected() {
    let denied = sidecar("  permitted: false\n  basis: Contributor has not granted redistribution");
    let errors = validate_yaml(&denied).expect_err("denied redistribution must be rejected");
    assert_eq!(errors[0].code, "redistribution_permission_required");
    assert_eq!(errors[0].path, "redistribution.permitted");

    let missing_basis = sidecar("  permitted: true");
    let errors = validate_yaml(&missing_basis).expect_err("missing basis must be rejected");
    assert_eq!(errors[0].code, "redistribution_permission_required");
    assert_eq!(errors[0].path, "redistribution.basis");

    let permitted = sidecar("  permitted: true\n  basis: Contributor grants public redistribution");
    validate_yaml(&permitted).expect("complete redistribution permission must be accepted");
}
