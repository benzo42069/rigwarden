//! Semantic undo journal for confirmed editor mutations.

mod journal;

pub use journal::{Journal, JournalError, PendingMutationId, UndoEntry};
