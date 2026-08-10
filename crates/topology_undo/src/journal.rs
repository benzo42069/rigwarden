use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

/// Magic prefix identifying a RigWarden journal snapshot.
const SNAPSHOT_MAGIC: &[u8; 4] = b"RWJS";

/// Explicit schema version for the internal journal snapshot format.
pub const JOURNAL_SNAPSHOT_SCHEMA_VERSION: u16 = 1;

const MAX_SNAPSHOT_BYTES: usize = 1024 * 1024;
const MAX_SNAPSHOT_STRING_BYTES: usize = 4 * 1024;
const MAX_SNAPSHOT_BRANCHES: usize = 1024;
const MAX_SNAPSHOT_ENTRIES_PER_BRANCH: usize = 16 * 1024;
const MAX_SNAPSHOT_ENTRIES_TOTAL: usize = 64 * 1024;

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

/// Structured failures produced while encoding or decoding a journal snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JournalSnapshotError {
    /// A mutation is awaiting confirmation and cannot become history yet.
    PendingMutation,
    /// An undo restoration is awaiting confirmation and cannot become history yet.
    PendingRestoration,
    /// The first four bytes do not identify a journal snapshot.
    InvalidMagic { found: [u8; 4] },
    /// The bytes use a schema version not supported by this build.
    UnsupportedSchemaVersion(u16),
    /// A field ended before its complete value was available.
    Truncated { field: &'static str },
    /// A length-prefixed string is not valid UTF-8.
    InvalidUtf8 { field: &'static str },
    /// A field exceeds the bounded size for its kind.
    LengthTooLarge {
        field: &'static str,
        length: usize,
        maximum: usize,
    },
    /// A collection count exceeds the bounded item count.
    CountTooLarge {
        field: &'static str,
        count: usize,
        maximum: usize,
    },
    /// The complete encoded snapshot exceeds the bounded size.
    SnapshotTooLarge { length: usize, maximum: usize },
    /// A branch name appears more than once in the snapshot.
    DuplicateBranch,
    /// The active branch name is not present in the branch table.
    ActiveBranchMissing,
    /// Input contains bytes after the final field.
    TrailingBytes { count: usize },
}

impl fmt::Display for JournalSnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PendingMutation => {
                write!(formatter, "journal has a mutation awaiting confirmation")
            }
            Self::PendingRestoration => {
                write!(formatter, "journal has a restoration awaiting confirmation")
            }
            Self::InvalidMagic { found } => {
                write!(formatter, "invalid journal snapshot magic: {found:02x?}")
            }
            Self::UnsupportedSchemaVersion(version) => {
                write!(
                    formatter,
                    "unsupported journal snapshot schema version: {version}"
                )
            }
            Self::Truncated { field } => {
                write!(formatter, "truncated journal snapshot field: {field}")
            }
            Self::InvalidUtf8 { field } => {
                write!(
                    formatter,
                    "invalid UTF-8 in journal snapshot field: {field}"
                )
            }
            Self::LengthTooLarge {
                field,
                length,
                maximum,
            } => write!(
                formatter,
                "journal snapshot field {field} length {length} exceeds {maximum}"
            ),
            Self::CountTooLarge {
                field,
                count,
                maximum,
            } => write!(
                formatter,
                "journal snapshot field {field} count {count} exceeds {maximum}"
            ),
            Self::SnapshotTooLarge { length, maximum } => {
                write!(
                    formatter,
                    "journal snapshot length {length} exceeds {maximum}"
                )
            }
            Self::DuplicateBranch => {
                write!(formatter, "journal snapshot contains a duplicate branch")
            }
            Self::ActiveBranchMissing => {
                write!(formatter, "journal snapshot active branch is not present")
            }
            Self::TrailingBytes { count } => {
                write!(formatter, "journal snapshot has {count} trailing byte(s)")
            }
        }
    }
}

impl std::error::Error for JournalSnapshotError {}

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

    /// Encode confirmed branch history as a bounded deterministic snapshot.
    ///
    /// Pending mutations and restorations are deliberately rejected: neither
    /// represents confirmed state that can safely become persistent history.
    pub fn encode_snapshot(&self) -> Result<Vec<u8>, JournalSnapshotError> {
        if !self.pending.is_empty() {
            return Err(JournalSnapshotError::PendingMutation);
        }
        if self.pending_restoration.is_some() {
            return Err(JournalSnapshotError::PendingRestoration);
        }

        let mut writer = SnapshotWriter::default();
        writer.append(SNAPSHOT_MAGIC)?;
        writer.u16(JOURNAL_SNAPSHOT_SCHEMA_VERSION)?;
        writer.count("branches", self.branches.len(), MAX_SNAPSHOT_BRANCHES)?;
        writer.string("active_branch", &self.current_branch)?;

        let mut total_entries = 0_usize;
        for (branch_name, entries) in &self.branches {
            writer.string("branch.name", branch_name)?;
            writer.count(
                "branch.entries",
                entries.len(),
                MAX_SNAPSHOT_ENTRIES_PER_BRANCH,
            )?;
            total_entries = total_entries.checked_add(entries.len()).ok_or(
                JournalSnapshotError::CountTooLarge {
                    field: "entries.total",
                    count: usize::MAX,
                    maximum: MAX_SNAPSHOT_ENTRIES_TOTAL,
                },
            )?;
            if total_entries > MAX_SNAPSHOT_ENTRIES_TOTAL {
                return Err(JournalSnapshotError::CountTooLarge {
                    field: "entries.total",
                    count: total_entries,
                    maximum: MAX_SNAPSHOT_ENTRIES_TOTAL,
                });
            }
            for entry in entries {
                writer.string("entry.target", &entry.target)?;
                writer.u64(entry.previous_value.to_bits())?;
                writer.u64(entry.new_value.to_bits())?;
            }
        }

        Ok(writer.bytes)
    }

    /// Decode a bounded deterministic snapshot into a fresh journal.
    pub fn decode_snapshot(input: &[u8]) -> Result<Self, JournalSnapshotError> {
        if input.len() > MAX_SNAPSHOT_BYTES {
            return Err(JournalSnapshotError::SnapshotTooLarge {
                length: input.len(),
                maximum: MAX_SNAPSHOT_BYTES,
            });
        }

        let mut reader = SnapshotReader::new(input);
        let magic = reader.take_exact(SNAPSHOT_MAGIC.len(), "magic")?;
        if magic != SNAPSHOT_MAGIC {
            let mut found = [0_u8; 4];
            found.copy_from_slice(magic);
            return Err(JournalSnapshotError::InvalidMagic { found });
        }

        let schema_version = reader.u16("schema_version")?;
        if schema_version != JOURNAL_SNAPSHOT_SCHEMA_VERSION {
            return Err(JournalSnapshotError::UnsupportedSchemaVersion(
                schema_version,
            ));
        }

        let branch_count = reader.count("branches", MAX_SNAPSHOT_BRANCHES)?;
        let current_branch = reader.string("active_branch")?;
        let mut branches = BTreeMap::new();
        let mut total_entries = 0_usize;

        for _ in 0..branch_count {
            let branch_name = reader.string("branch.name")?;
            if branches.contains_key(&branch_name) {
                return Err(JournalSnapshotError::DuplicateBranch);
            }
            let entry_count = reader.count("branch.entries", MAX_SNAPSHOT_ENTRIES_PER_BRANCH)?;
            total_entries = total_entries.checked_add(entry_count).ok_or(
                JournalSnapshotError::CountTooLarge {
                    field: "entries.total",
                    count: usize::MAX,
                    maximum: MAX_SNAPSHOT_ENTRIES_TOTAL,
                },
            )?;
            if total_entries > MAX_SNAPSHOT_ENTRIES_TOTAL {
                return Err(JournalSnapshotError::CountTooLarge {
                    field: "entries.total",
                    count: total_entries,
                    maximum: MAX_SNAPSHOT_ENTRIES_TOTAL,
                });
            }

            let mut entries = Vec::with_capacity(entry_count);
            for _ in 0..entry_count {
                let target = reader.string("entry.target")?;
                let previous_value = f64::from_bits(reader.u64("entry.previous_value")?);
                let new_value = f64::from_bits(reader.u64("entry.new_value")?);
                entries.push(UndoEntry::new(target, previous_value, new_value));
            }
            branches.insert(branch_name, entries);
        }

        reader.finish()?;
        if !branches.contains_key(&current_branch) {
            return Err(JournalSnapshotError::ActiveBranchMissing);
        }

        Ok(Self {
            next_id: 0,
            owner: Arc::new(()),
            pending: BTreeMap::new(),
            branches,
            current_branch,
            pending_restoration: None,
        })
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

#[derive(Default)]
struct SnapshotWriter {
    bytes: Vec<u8>,
}

impl SnapshotWriter {
    fn append(&mut self, bytes: &[u8]) -> Result<(), JournalSnapshotError> {
        let new_length = self.bytes.len().checked_add(bytes.len()).ok_or(
            JournalSnapshotError::SnapshotTooLarge {
                length: usize::MAX,
                maximum: MAX_SNAPSHOT_BYTES,
            },
        )?;
        if new_length > MAX_SNAPSHOT_BYTES {
            return Err(JournalSnapshotError::SnapshotTooLarge {
                length: new_length,
                maximum: MAX_SNAPSHOT_BYTES,
            });
        }
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }

    fn u16(&mut self, value: u16) -> Result<(), JournalSnapshotError> {
        self.append(&value.to_le_bytes())
    }

    fn u32(&mut self, value: u32) -> Result<(), JournalSnapshotError> {
        self.append(&value.to_le_bytes())
    }

    fn u64(&mut self, value: u64) -> Result<(), JournalSnapshotError> {
        self.append(&value.to_le_bytes())
    }

    fn string(&mut self, field: &'static str, value: &str) -> Result<(), JournalSnapshotError> {
        let bytes = value.as_bytes();
        if bytes.len() > MAX_SNAPSHOT_STRING_BYTES {
            return Err(JournalSnapshotError::LengthTooLarge {
                field,
                length: bytes.len(),
                maximum: MAX_SNAPSHOT_STRING_BYTES,
            });
        }
        let length =
            u32::try_from(bytes.len()).map_err(|_| JournalSnapshotError::LengthTooLarge {
                field,
                length: bytes.len(),
                maximum: u32::MAX as usize,
            })?;
        self.u32(length)?;
        self.append(bytes)
    }

    fn count(
        &mut self,
        field: &'static str,
        count: usize,
        maximum: usize,
    ) -> Result<(), JournalSnapshotError> {
        if count > maximum {
            return Err(JournalSnapshotError::CountTooLarge {
                field,
                count,
                maximum,
            });
        }
        let count = u32::try_from(count).map_err(|_| JournalSnapshotError::CountTooLarge {
            field,
            count,
            maximum: u32::MAX as usize,
        })?;
        self.u32(count)
    }
}

struct SnapshotReader<'a> {
    input: &'a [u8],
    offset: usize,
}

impl<'a> SnapshotReader<'a> {
    const fn new(input: &'a [u8]) -> Self {
        Self { input, offset: 0 }
    }

    fn take_exact(
        &mut self,
        length: usize,
        field: &'static str,
    ) -> Result<&'a [u8], JournalSnapshotError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(JournalSnapshotError::Truncated { field })?;
        if end > self.input.len() {
            return Err(JournalSnapshotError::Truncated { field });
        }
        let bytes = &self.input[self.offset..end];
        self.offset = end;
        Ok(bytes)
    }

    fn u16(&mut self, field: &'static str) -> Result<u16, JournalSnapshotError> {
        let bytes = self.take_exact(2, field)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn u32(&mut self, field: &'static str) -> Result<u32, JournalSnapshotError> {
        let bytes = self.take_exact(4, field)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn u64(&mut self, field: &'static str) -> Result<u64, JournalSnapshotError> {
        let bytes = self.take_exact(8, field)?;
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    fn string(&mut self, field: &'static str) -> Result<String, JournalSnapshotError> {
        let bytes = self.variable(field, MAX_SNAPSHOT_STRING_BYTES)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| JournalSnapshotError::InvalidUtf8 { field })
    }

    fn variable(
        &mut self,
        field: &'static str,
        maximum: usize,
    ) -> Result<&'a [u8], JournalSnapshotError> {
        let length = self.u32(field)? as usize;
        if length > maximum {
            return Err(JournalSnapshotError::LengthTooLarge {
                field,
                length,
                maximum,
            });
        }
        self.take_exact(length, field)
    }

    fn count(
        &mut self,
        field: &'static str,
        maximum: usize,
    ) -> Result<usize, JournalSnapshotError> {
        let count = self.u32(field)? as usize;
        if count > maximum {
            return Err(JournalSnapshotError::CountTooLarge {
                field,
                count,
                maximum,
            });
        }
        Ok(count)
    }

    fn finish(&self) -> Result<(), JournalSnapshotError> {
        let count = self.input.len() - self.offset;
        if count == 0 {
            Ok(())
        } else {
            Err(JournalSnapshotError::TrailingBytes { count })
        }
    }
}
