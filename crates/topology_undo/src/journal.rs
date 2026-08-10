use std::collections::BTreeMap;

/// Stable handle for a mutation that has been requested but not confirmed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PendingMutationId(u64);

/// One completed semantic change that can be reversed.
#[derive(Clone, Debug, PartialEq)]
pub struct UndoEntry {
    target: String,
    previous_value: f64,
    new_value: f64,
}

impl UndoEntry {
    fn new(target: String, previous_value: f64, new_value: f64) -> Self {
        Self {
            target,
            previous_value,
            new_value,
        }
    }

    /// Return the stable semantic target of the completed change.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Return the confirmed value that existed before the change.
    pub const fn previous_value(&self) -> f64 {
        self.previous_value
    }

    /// Return the value confirmed after the change.
    pub const fn new_value(&self) -> f64 {
        self.new_value
    }
}

/// Why a pending mutation could not be completed or discarded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JournalError {
    /// The supplied handle is not currently pending.
    UnknownPendingMutation(PendingMutationId),
}

/// In-memory semantic journal for pending and confirmed changes.
#[derive(Debug)]
pub struct Journal {
    next_id: u64,
    pending: BTreeMap<PendingMutationId, PendingEntry>,
    branches: BTreeMap<String, Vec<UndoEntry>>,
    current_branch: String,
}

#[derive(Debug)]
struct PendingEntry {
    branch_name: String,
    entry: UndoEntry,
}

impl Journal {
    /// Create an empty journal.
    pub fn new() -> Self {
        Self::new_with_preset("default")
    }

    /// Create an empty journal for the supplied preset context.
    pub fn new_with_preset(preset: impl Into<String>) -> Self {
        let preset = preset.into();
        let mut branches = BTreeMap::new();
        branches.insert(preset.clone(), Vec::new());
        Self {
            next_id: 0,
            pending: BTreeMap::new(),
            branches,
            current_branch: preset,
        }
    }

    /// Switch the active preset context, retaining each context's history in
    /// its own named branch.
    pub fn switch_preset(&mut self, preset: impl Into<String>) {
        let preset = preset.into();
        self.branches.entry(preset.clone()).or_default();
        self.current_branch = preset;
    }

    /// Return the name of the active preset branch.
    pub fn current_branch_name(&self) -> &str {
        &self.current_branch
    }

    /// Return completed entries retained by a named preset branch.
    pub fn branch_entries(&self, branch_name: &str) -> Option<&[UndoEntry]> {
        self.branches.get(branch_name).map(Vec::as_slice)
    }

    /// Stage a parameter change using the caller's confirmed prior value.
    ///
    /// The returned handle remains pending until [`Self::confirm`] or
    /// [`Self::fail`] consumes it. Pending changes are never exposed as undo
    /// entries.
    pub fn begin_parameter_change(
        &mut self,
        target: impl Into<String>,
        confirmed_previous_value: f64,
        requested_new_value: f64,
    ) -> PendingMutationId {
        let id = PendingMutationId(self.next_id);
        self.next_id = self.next_id.saturating_add(1);
        self.pending.insert(
            id,
            PendingEntry {
                branch_name: self.current_branch.clone(),
                entry: UndoEntry::new(target.into(), confirmed_previous_value, requested_new_value),
            },
        );
        id
    }

    /// Mark a pending mutation confirmed and append its undo entry.
    pub fn confirm(&mut self, id: PendingMutationId) -> Result<(), JournalError> {
        let pending = self
            .pending
            .remove(&id)
            .ok_or(JournalError::UnknownPendingMutation(id))?;
        self.branches
            .entry(pending.branch_name)
            .or_default()
            .push(pending.entry);
        Ok(())
    }

    /// Mark a pending mutation failed and discard it without journaling.
    pub fn fail(&mut self, id: PendingMutationId) -> Result<(), JournalError> {
        self.pending
            .remove(&id)
            .map(|_| ())
            .ok_or(JournalError::UnknownPendingMutation(id))
    }

    /// Return the number of mutations that are awaiting confirmation.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Return completed undo entries in confirmation order.
    pub fn completed_entries(&self) -> &[UndoEntry] {
        self.branches
            .get(&self.current_branch)
            .expect("journal always contains its active branch")
    }
}

impl Default for Journal {
    fn default() -> Self {
        Self::new()
    }
}
