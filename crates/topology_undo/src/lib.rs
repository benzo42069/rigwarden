//! Semantic undo journal for confirmed editor mutations.

mod journal;
mod persistence;

pub use journal::{
    JOURNAL_SNAPSHOT_SCHEMA_VERSION, Journal, JournalError, JournalSnapshotError,
    PendingMutationId, UndoEntry,
};
pub use persistence::{JournalFileOperation, JournalPersistenceError};
