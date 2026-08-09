use std::{
    fs,
    path::Path,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_TEMP_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

fn temporary_directory() -> std::path::PathBuf {
    let ordinal = NEXT_TEMP_DIRECTORY.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "topology-devtools-fixture-cli-{}-{ordinal}",
        std::process::id()
    ));
    fs::create_dir(&path).expect("temporary fixture directory must be creatable");
    path
}

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

fn run_validate_fixture(path: &Path) -> std::process::Output {
    let binary = std::env::var("CARGO_BIN_EXE_topology-devtools")
        .expect("Cargo must expose the topology-devtools binary to integration tests");
    Command::new(binary)
        .args([
            "validate-fixture",
            path.to_str().expect("UTF-8 fixture path"),
        ])
        .output()
        .expect("topology-devtools binary must run")
}

#[test]
fn fixture_cli_returns_truthful_exit_status() {
    let directory = temporary_directory();
    let valid_path = directory.join("valid.yaml");
    let invalid_path = directory.join("denied.yaml");
    fs::write(
        &valid_path,
        sidecar("  permitted: true\n  basis: Contributor grants public redistribution"),
    )
    .expect("valid fixture sidecar must be writable");
    fs::write(
        &invalid_path,
        sidecar("  permitted: false\n  basis: Contributor has not granted redistribution"),
    )
    .expect("invalid fixture sidecar must be writable");

    let valid = run_validate_fixture(&valid_path);
    assert!(
        valid.status.success(),
        "valid sidecar must exit zero; stderr: {}",
        String::from_utf8_lossy(&valid.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&valid.stdout), "valid\n");
    assert!(
        valid.stderr.is_empty(),
        "valid sidecar must not emit diagnostics"
    );

    let invalid = run_validate_fixture(&invalid_path);
    assert!(
        !invalid.status.success(),
        "invalid sidecar must exit nonzero"
    );
    assert_eq!(String::from_utf8_lossy(&invalid.stdout), "");
    let diagnostic = String::from_utf8_lossy(&invalid.stderr);
    assert!(diagnostic.contains("denied.yaml: redistribution.permitted"));
    assert!(diagnostic.contains("redistribution_permission_required"));

    fs::remove_dir_all(directory).expect("temporary fixture directory must be removable");
}
