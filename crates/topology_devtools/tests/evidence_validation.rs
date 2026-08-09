use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

use topology_devtools::evidence::validate_completed_evidence;

const REQUIRED_FILES_EXCEPT_RED_LOG: &[&str] = &[
    "environment.txt",
    "files-changed.txt",
    "green-command.txt",
    "green-exit-status.txt",
    "green.log",
    "handoff.md",
    "red-command.txt",
    "red-exit-status.txt",
    "review.md",
    "sweep-commands.txt",
    "sweep-exit-statuses.txt",
    "sweep.log",
    "work-item.yaml",
];

static NEXT_TEMP_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

fn temporary_evidence_directory() -> std::path::PathBuf {
    let ordinal = NEXT_TEMP_DIRECTORY.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "topology-devtools-evidence-validation-{}-{ordinal}",
        std::process::id()
    ));
    fs::create_dir(&path).expect("temporary evidence directory must be creatable");
    path
}

fn write_required_files_except_red_log(directory: &Path) {
    for name in REQUIRED_FILES_EXCEPT_RED_LOG {
        fs::write(directory.join(name), b"fixture")
            .expect("evidence fixture file must be writable");
    }
}

#[test]
fn completed_evidence_without_red_log_is_rejected() {
    let directory = temporary_evidence_directory();
    write_required_files_except_red_log(&directory);

    let errors =
        validate_completed_evidence(&directory).expect_err("green-only evidence must be rejected");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].code, "missing_red_log");
    assert_eq!(errors[0].path, "red.log");
    assert_eq!(
        errors[0].message,
        "required evidence file `red.log` is missing"
    );

    fs::write(directory.join("red.log"), b"focused RED output")
        .expect("focused RED log must be writable");
    validate_completed_evidence(&directory).expect("otherwise complete evidence must be accepted");

    fs::remove_file(directory.join("environment.txt")).expect("environment fixture must exist");
    fs::remove_file(directory.join("red.log")).expect("RED fixture must exist");
    fs::remove_file(directory.join("work-item.yaml")).expect("packet fixture must exist");
    let errors = validate_completed_evidence(&directory)
        .expect_err("all missing required files must be reported");
    let paths: Vec<_> = errors.iter().map(|error| error.path.as_str()).collect();
    assert_eq!(paths, ["environment.txt", "red.log", "work-item.yaml"]);

    fs::remove_dir_all(directory).expect("temporary evidence directory must be removable");
}
