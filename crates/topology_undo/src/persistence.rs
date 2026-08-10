use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::{Journal, JournalSnapshotError};

// Keep file reads bounded to the same limit enforced by the reviewed
// in-memory snapshot codec. The adapter must not allocate for an unbounded
// caller-selected file before the codec can reject it.
const MAX_SNAPSHOT_BYTES: usize = 1024 * 1024;
static NEXT_TEMP_PATH_ID: AtomicU64 = AtomicU64::new(0);

/// File operation that failed while persisting or loading a journal snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JournalFileOperation {
    CreateTemporary,
    WriteTemporary,
    FlushTemporary,
    ReplaceSnapshot,
    ReadSnapshot,
}

impl fmt::Display for JournalFileOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operation = match self {
            Self::CreateTemporary => "create temporary snapshot",
            Self::WriteTemporary => "write temporary snapshot",
            Self::FlushTemporary => "flush temporary snapshot",
            Self::ReplaceSnapshot => "replace snapshot",
            Self::ReadSnapshot => "read snapshot",
        };
        formatter.write_str(operation)
    }
}

/// Structured failures returned by the local journal snapshot adapter.
#[derive(Debug)]
pub enum JournalPersistenceError {
    /// The caller-selected path could not be represented as a sibling file.
    InvalidPath,
    /// The local filesystem rejected one bounded adapter operation.
    Io {
        operation: JournalFileOperation,
        kind: io::ErrorKind,
    },
    /// The local file was read but did not contain a valid journal snapshot.
    Snapshot(JournalSnapshotError),
}

impl fmt::Display for JournalPersistenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath => formatter.write_str("journal snapshot path has no file name"),
            Self::Io { operation, kind } => {
                write!(formatter, "{operation} failed with {kind}")
            }
            Self::Snapshot(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for JournalPersistenceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Snapshot(error) => Some(error),
            Self::InvalidPath | Self::Io { .. } => None,
        }
    }
}

impl Journal {
    /// Save confirmed journal history through a caller-selected local path.
    ///
    /// The snapshot is first written to a sibling temporary file and then
    /// renamed over the destination. This is a local write/reload boundary;
    /// it does not claim fsync or power-loss durability.
    pub fn save_snapshot_to(&self, path: impl AsRef<Path>) -> Result<(), JournalPersistenceError> {
        save_snapshot(path.as_ref(), self)
    }

    /// Load a fresh journal from a caller-selected local snapshot path.
    pub fn load_snapshot_from(path: impl AsRef<Path>) -> Result<Self, JournalPersistenceError> {
        load_snapshot(path.as_ref())
    }
}

fn save_snapshot(path: &Path, journal: &Journal) -> Result<(), JournalPersistenceError> {
    let snapshot = journal
        .encode_snapshot()
        .map_err(JournalPersistenceError::Snapshot)?;
    let temporary_path = sibling_temporary_path(path)?;

    let mut temporary_created = false;
    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
            .map_err(|error| JournalPersistenceError::Io {
                operation: JournalFileOperation::CreateTemporary,
                kind: error.kind(),
            })?;
        temporary_created = true;
        file.write_all(&snapshot)
            .map_err(|error| JournalPersistenceError::Io {
                operation: JournalFileOperation::WriteTemporary,
                kind: error.kind(),
            })?;
        file.flush().map_err(|error| JournalPersistenceError::Io {
            operation: JournalFileOperation::FlushTemporary,
            kind: error.kind(),
        })?;
        fs::rename(&temporary_path, path).map_err(|error| JournalPersistenceError::Io {
            operation: JournalFileOperation::ReplaceSnapshot,
            kind: error.kind(),
        })?;
        temporary_created = false;
        Ok(())
    })();

    if result.is_err() && temporary_created {
        let _ = fs::remove_file(&temporary_path);
    }
    result
}

fn load_snapshot(path: &Path) -> Result<Journal, JournalPersistenceError> {
    let file = File::open(path).map_err(|error| JournalPersistenceError::Io {
        operation: JournalFileOperation::ReadSnapshot,
        kind: error.kind(),
    })?;
    let mut bounded = file.take((MAX_SNAPSHOT_BYTES as u64).saturating_add(1));
    let mut snapshot = Vec::new();
    bounded
        .read_to_end(&mut snapshot)
        .map_err(|error| JournalPersistenceError::Io {
            operation: JournalFileOperation::ReadSnapshot,
            kind: error.kind(),
        })?;
    if snapshot.len() > MAX_SNAPSHOT_BYTES {
        return Err(JournalPersistenceError::Snapshot(
            JournalSnapshotError::SnapshotTooLarge {
                length: snapshot.len(),
                maximum: MAX_SNAPSHOT_BYTES,
            },
        ));
    }

    Journal::decode_snapshot(&snapshot).map_err(JournalPersistenceError::Snapshot)
}

fn sibling_temporary_path(path: &Path) -> Result<PathBuf, JournalPersistenceError> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or(JournalPersistenceError::InvalidPath)?;
    let process_id = std::process::id();
    let path_id = NEXT_TEMP_PATH_ID.fetch_add(1, Ordering::Relaxed);
    Ok(path.with_file_name(format!(".{file_name}.{process_id}.{path_id}.tmp")))
}
