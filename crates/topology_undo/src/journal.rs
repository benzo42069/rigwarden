use std::collections::BTreeMap;
use std::sync::Arc;

/// Stable handle for a mutation that has been requested but not confirmed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PendingMutationId(u64);

/// Stable handle for a restoration that has been prepared but not confirmed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UndoProposalId(u64);

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

/// The exact semantic restoration proposed for the most recent completed
/// entry.
#[derive(Clone, Debug, PartialEq)]
pub struct UndoProposal {
    id: UndoProposalId,
    owner: Arc<()>,
    target: String,
    restoration_value: f64,
}

impl UndoProposal {
    fn new(id: UndoProposalId, owner: Arc<()>, entry: &UndoEntry) -> Self {
        Self {
            id,
            owner,
            target: entry.target.clone(),
            restoration_value: entry.previous_value,
        }
    }

    /// Return the semantic target that will be restored.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Return the exact confirmed value that will be restored.
    pub const fn restoration_value(&self) -> f64 {
        self.restoration_value
    }
}

/// Why a pending mutation could not be completed or discarded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JournalError {
    /// The supplied handle is not currently pending.
    UnknownPendingMutation(PendingMutationId),
    /// The supplied restoration proposal is not currently pending.
    UnknownUndoProposal(UndoProposalId),
}

/// In-memory semantic journal for pending and confirmed changes.
#[derive(Debug)]
pub struct Journal {
    next_id: u64,
    owner: Arc<()>,
    pending: BTreeMap<PendingMutationId, PendingEntry>,
    branches: BTreeMap<String, Vec<UndoEntry>>,
    current_branch: String,
    pending_restoration: Option<PendingRestoration>,
}

#[derive(Debug)]
struct PendingEntry {
    branch_name: String,
    entry: UndoEntry,
}

#[derive(Debug)]
struct PendingRestoration {
    proposal: UndoProposal,
    branch_name: String,
    entry_index: usize,
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
            owner: Arc::new(()),
            pending: BTreeMap::new(),
            branches,
            current_branch: preset,
            pending_restoration: None,
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

    /// Prepare a restoration for the most recent completed entry without
    /// removing that entry from the active branch.
    pub fn prepare_undo(&mut self) -> Option<UndoProposal> {
        if self.pending_restoration.is_some() {
            return None;
        }

        let entries = self.branches.get(&self.current_branch)?;
        let entry_index = entries.len().checked_sub(1)?;
        let proposal_id = UndoProposalId(self.next_id);
        self.next_id = self.next_id.saturating_add(1);
        let proposal = UndoProposal::new(proposal_id, self.owner.clone(), &entries[entry_index]);
        self.pending_restoration = Some(PendingRestoration {
            proposal: proposal.clone(),
            branch_name: self.current_branch.clone(),
            entry_index,
        });
        Some(proposal)
    }

    /// Confirm a prepared restoration and consume exactly its completed entry.
    pub fn confirm_undo(&mut self, proposal: UndoProposal) -> Result<(), JournalError> {
        let pending = self
            .pending_restoration
            .take()
            .ok_or(JournalError::UnknownUndoProposal(proposal.id))?;
        if !Arc::ptr_eq(&pending.proposal.owner, &proposal.owner) || pending.proposal != proposal {
            let proposal_id = proposal.id;
            self.pending_restoration = Some(pending);
            return Err(JournalError::UnknownUndoProposal(proposal_id));
        }

        let entries = self
            .branches
            .get_mut(&pending.branch_name)
            .expect("journal always retains a branch for a pending restoration");
        let entry = entries
            .get(pending.entry_index)
            .expect("journal entries remain stable while a restoration is pending");
        assert_eq!(
            entry.target, pending.proposal.target,
            "pending restoration target must remain stable"
        );
        assert_eq!(
            entry.previous_value, pending.proposal.restoration_value,
            "pending restoration value must remain stable"
        );
        entries.remove(pending.entry_index);
        Ok(())
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
