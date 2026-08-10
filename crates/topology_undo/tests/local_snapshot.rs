use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use topology_undo::{Journal, JournalPersistenceError, JournalSnapshotError};

#[cfg(unix)]
use std::os::unix::fs as unix_fs;

static NEXT_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

#[test]
fn confirmed_entry_reloads_from_a_local_snapshot_after_restart_simulation() {
    let directory = temporary_directory();
    let snapshot_path = directory.0.join("journal.rwjs");

    let mut journal = Journal::new_with_preset("preset-a");
    let mutation = journal.begin_parameter_change("amp-1/gain", 3.0, 4.5);
    journal.confirm(mutation).expect("mutation should confirm");
    journal.switch_preset("preset-b");
    journal.switch_preset("preset-a");

    journal
        .save_snapshot_to(&snapshot_path)
        .expect("confirmed journal should save");
    let snapshot_bytes = fs::read(&snapshot_path).expect("saved snapshot should be readable");

    let saved_paths = fs::read_dir(&directory.0)
        .expect("temporary directory should be readable")
        .map(|entry| {
            entry
                .expect("directory entry should be readable")
                .file_name()
        })
        .collect::<Vec<_>>();
    assert_eq!(saved_paths, vec![std::ffi::OsString::from("journal.rwjs")]);

    drop(journal);
    let restored = Journal::load_snapshot_from(&snapshot_path).expect("snapshot should reload");
    assert_eq!(restored.current_branch_name(), "preset-a");
    let entries = restored
        .branch_entries("preset-a")
        .expect("active branch should survive");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].target(), "amp-1/gain");
    assert_eq!(entries[0].previous_value().to_bits(), 3.0f64.to_bits());
    assert_eq!(entries[0].new_value().to_bits(), 4.5f64.to_bits());
    assert!(restored.branch_entries("preset-b").is_some());

    fs::write(&snapshot_path, b"not-a-journal")
        .expect("test should be able to write malformed snapshot bytes");
    assert!(matches!(
        Journal::load_snapshot_from(&snapshot_path),
        Err(JournalPersistenceError::Snapshot(
            JournalSnapshotError::InvalidMagic { .. }
        ))
    ));

    fs::write(&snapshot_path, &snapshot_bytes).expect("valid snapshot should be restorable");
    let mut pending_mutation = Journal::new_with_preset("preset-a");
    pending_mutation.begin_parameter_change("amp-1/gain", 3.0, 4.5);
    assert!(matches!(
        pending_mutation.save_snapshot_to(&snapshot_path),
        Err(JournalPersistenceError::Snapshot(
            JournalSnapshotError::PendingMutation
        ))
    ));
    assert_eq!(
        fs::read(&snapshot_path).expect("failed save must preserve prior snapshot"),
        snapshot_bytes
    );

    let mut pending_restoration = Journal::new_with_preset("preset-a");
    let mutation = pending_restoration.begin_parameter_change("amp-1/gain", 3.0, 4.5);
    pending_restoration
        .confirm(mutation)
        .expect("mutation should confirm");
    pending_restoration
        .prepare_undo()
        .expect("confirmed entry should prepare undo");
    assert!(matches!(
        pending_restoration.save_snapshot_to(&snapshot_path),
        Err(JournalPersistenceError::Snapshot(
            JournalSnapshotError::PendingRestoration
        ))
    ));
    assert_eq!(
        fs::read(&snapshot_path).expect("failed save must preserve prior snapshot"),
        snapshot_bytes
    );
}

#[cfg(unix)]
#[test]
fn preexisting_snapshot_temp_symlink_never_overwrites_external_target() {
    let directory = temporary_directory();
    let external_directory = temporary_directory();
    let snapshot_path = directory.0.join("journal.rwjs");
    let external_target = external_directory.0.join("external.bin");
    let external_before = b"external state must remain unchanged";
    fs::write(&external_target, external_before).expect("external fixture should be writable");
    // The focused selector starts a fresh test process, so `.0.tmp` is the
    // exact first generated sibling. The package sweep can run the restart
    // test first and consume `.0.tmp`; pre-create that next candidate too so
    // this regression still attacks whichever current counter is allocated.
    let temporary_paths = (0..=1)
        .map(|path_id| {
            directory.0.join(format!(
                ".journal.rwjs.{}.{}.tmp",
                std::process::id(),
                path_id
            ))
        })
        .collect::<Vec<_>>();
    for temporary_path in &temporary_paths {
        unix_fs::symlink(&external_target, temporary_path)
            .expect("test should be able to create a temporary-path symlink");
    }

    let mut journal = Journal::new_with_preset("preset-a");
    let mutation = journal.begin_parameter_change("amp-1/gain", 3.0, 4.5);
    journal.confirm(mutation).expect("mutation should confirm");

    let result = journal.save_snapshot_to(&snapshot_path);

    assert_eq!(
        fs::read(&external_target).expect("external target should remain readable"),
        external_before,
        "a temporary-path symlink must never receive snapshot bytes"
    );
    for temporary_path in &temporary_paths {
        assert!(
            fs::symlink_metadata(temporary_path)
                .expect("pre-existing temporary path must remain present")
                .file_type()
                .is_symlink(),
            "cleanup must not follow or remove a pre-existing symlink"
        );
    }

    assert!(matches!(
        result,
        Err(JournalPersistenceError::Io {
            operation: topology_undo::JournalFileOperation::CreateTemporary,
            ..
        })
    ));
    assert!(
        fs::symlink_metadata(&snapshot_path).is_err(),
        "a rejected unsafe save must not create or replace the destination"
    );
}

struct TemporaryDirectory(PathBuf);

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn temporary_directory() -> TemporaryDirectory {
    loop {
        let id = NEXT_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rigwarden-top-undo-005-{}-{id}",
            std::process::id()
        ));
        match fs::create_dir(&path) {
            Ok(()) => return TemporaryDirectory(path),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => panic!("temporary directory setup failed: {error}"),
        }
    }
}
