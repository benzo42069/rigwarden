//! Semantic undo journal for confirmed editor mutations.

mod journal;

pub use journal::{
    JOURNAL_SNAPSHOT_SCHEMA_VERSION, Journal, JournalError, JournalSnapshotError,
    PendingMutationId, UndoEntry,
};
