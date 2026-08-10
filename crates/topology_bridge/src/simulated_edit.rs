//! Rust-owned composition of one typed synthetic edit and its confirmed undo.
//!
//! The composition deliberately keeps the simulator's synthetic payloads and
//! transport value private. Callers receive only typed state and a sanitized
//! semantic transcript.

use topology_command_engine::{
    validate_parameter_mutation, MutationValidationError, ParameterMutationRequest,
    ValidatedParameterMutation,
};
use topology_device_registry::DeviceProfile;
use topology_simulator::{
    ScriptedRequest, ScriptedResponse, ScriptedTransport, SimulatorIdentity, SyntheticPayload,
};
use topology_undo::Journal;

const TARGET_BLOCK_ID: &str = "amp-1";
const TARGET_PARAMETER_ID: &str = "gain";
const TARGET: &str = "amp-1/gain";
const PRESET: &str = "synthetic-preset";
const EDIT_REQUEST_ID: u64 = 1;
const RESTORATION_REQUEST_ID: u64 = 2;

/// Typed application-state phases emitted by the synthetic composition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditState {
    /// The requested edit has been staged but not confirmed by the peer.
    PendingEdit { stored_value: i32 },
    /// The requested edit has been confirmed and journaled.
    ConfirmedEdit { stored_value: i32 },
    /// The journal restoration has been staged but not confirmed by the peer.
    PendingRestoration { stored_value: i32 },
    /// The journal restoration has been confirmed and consumed.
    ConfirmedRestoration { stored_value: i32 },
}

/// A byte-free semantic event from the scripted exchange transcript.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticTranscriptEntry {
    /// A typed parameter request was sent to the synthetic peer.
    Request { stored_value: i32 },
    /// The synthetic peer confirmed the typed parameter request.
    Confirmed { stored_value: i32 },
}

/// Typed final state for the synthetic parameter.
#[derive(Clone, Debug, PartialEq)]
pub struct TypedParameterState {
    target: String,
    stored_value: i32,
    decimal_places: u8,
}

impl TypedParameterState {
    fn new(target: impl Into<String>, stored_value: i32, decimal_places: u8) -> Self {
        Self {
            target: target.into(),
            stored_value,
            decimal_places,
        }
    }

    /// Return the semantic parameter target.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Return the exact profile-stored integer value.
    pub const fn stored_value(&self) -> i32 {
        self.stored_value
    }

    /// Return the profile-owned decimal precision.
    pub const fn decimal_places(&self) -> u8 {
        self.decimal_places
    }

    /// Return the display value derived from the profile precision.
    pub fn display_value(&self) -> f64 {
        self.stored_value as f64 / 10_f64.powi(i32::from(self.decimal_places))
    }
}

/// The confirmed journal entry retained while undo restoration is pending.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfirmedJournalEntry {
    target: String,
    previous_stored_value: i32,
    new_stored_value: i32,
}

impl ConfirmedJournalEntry {
    fn new(target: impl Into<String>, previous_stored_value: i32, new_stored_value: i32) -> Self {
        Self {
            target: target.into(),
            previous_stored_value,
            new_stored_value,
        }
    }

    /// Return the semantic target recorded by the journal.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Return the confirmed value captured before the edit.
    pub const fn previous_stored_value(&self) -> i32 {
        self.previous_stored_value
    }

    /// Return the confirmed value captured after the edit.
    pub const fn new_stored_value(&self) -> i32 {
        self.new_stored_value
    }
}

/// Sanitized result of the complete synthetic edit and undo path.
#[derive(Clone, Debug, PartialEq)]
pub struct SimulatedEditSummary {
    simulator_identity: SimulatorIdentity,
    target: String,
    final_state: TypedParameterState,
    states: Vec<EditState>,
    transcript: Vec<SemanticTranscriptEntry>,
    exchange_count: usize,
    journal_entry_before_undo: ConfirmedJournalEntry,
    journal_entries_before_undo: usize,
    journal_entries_remaining: usize,
}

impl SimulatedEditSummary {
    /// Return the explicitly synthetic peer identity.
    pub const fn simulator_identity(&self) -> SimulatorIdentity {
        self.simulator_identity
    }

    /// Return the semantic target edited by this composition.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Return the final confirmed typed state.
    pub const fn final_state(&self) -> &TypedParameterState {
        &self.final_state
    }

    /// Return the ordered pending/confirmed state transitions.
    pub fn states(&self) -> &[EditState] {
        &self.states
    }

    /// Return the ordered, byte-free semantic exchange transcript.
    pub fn transcript(&self) -> &[SemanticTranscriptEntry] {
        &self.transcript
    }

    /// Return the number of private synthetic exchange attempts.
    pub const fn exchange_count(&self) -> usize {
        self.exchange_count
    }

    /// Return the exact confirmed journal entry retained while undo was pending.
    pub const fn journal_entry_before_undo(&self) -> &ConfirmedJournalEntry {
        &self.journal_entry_before_undo
    }

    /// Return the number of completed entries retained while undo was pending.
    pub const fn journal_entries_before_undo(&self) -> usize {
        self.journal_entries_before_undo
    }

    /// Return the number of completed entries after the restoration confirms.
    pub const fn journal_entries_remaining(&self) -> usize {
        self.journal_entries_remaining
    }
}

/// Stage at which an exchange failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExchangeStage {
    /// The requested edit exchange.
    Edit,
    /// The prepared undo restoration exchange.
    Restoration,
}

/// Stage at which the in-memory journal contract failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JournalStage {
    /// Recording the pending edit.
    BeginEdit,
    /// Confirming the edit.
    ConfirmEdit,
    /// Preparing the restoration.
    PrepareRestoration,
    /// Confirming the restoration.
    ConfirmRestoration,
}

/// Sanitized failures from the typed composition path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SimulatedEditError {
    /// Profile capability or range validation failed before any exchange.
    Validation {
        source: MutationValidationError,
        exchange_count: usize,
    },
    /// The fixed synthetic script rejected an exchange or its correlation.
    ExchangeRejected {
        stage: ExchangeStage,
        exchange_count: usize,
    },
    /// The journal could not perform the expected confirmed transition.
    JournalFailure {
        stage: JournalStage,
        exchange_count: usize,
    },
    /// The profile did not preserve one target/precision across the path.
    ProfileInconsistent { exchange_count: usize },
}

impl SimulatedEditError {
    /// Return the number of private synthetic exchange attempts before failure.
    pub const fn exchange_count(&self) -> usize {
        match self {
            Self::Validation { exchange_count, .. }
            | Self::ExchangeRejected { exchange_count, .. }
            | Self::JournalFailure { exchange_count, .. }
            | Self::ProfileInconsistent { exchange_count } => *exchange_count,
        }
    }
}

/// Compose one typed synthetic edit, confirmation, prepared undo, and
/// restoration confirmation.
///
/// The profile validator runs for the initial and requested stored values
/// before either scripted exchange. The simulator payloads and transport are
/// intentionally local implementation details; this function returns only a
/// typed state summary and a byte-free semantic transcript.
pub fn compose_synthetic_parameter_edit(
    profile: &DeviceProfile,
    confirmed_stored_value: i32,
    requested_stored_value: i32,
) -> Result<SimulatedEditSummary, SimulatedEditError> {
    let mut exchange_count = 0;
    let confirmed = validate(profile, confirmed_stored_value, exchange_count)?;
    let requested = validate(profile, requested_stored_value, exchange_count)?;
    if confirmed.decimal_places() != requested.decimal_places() {
        return Err(SimulatedEditError::ProfileInconsistent { exchange_count });
    }

    let mut journal = Journal::new_with_preset(PRESET);
    let mut states = vec![EditState::PendingEdit {
        stored_value: requested.stored_value(),
    }];
    let mut transcript = Vec::with_capacity(4);

    let pending_edit = journal.begin_parameter_change(
        TARGET,
        display_value(confirmed.stored_value(), confirmed.decimal_places()),
        display_value(requested.stored_value(), requested.decimal_places()),
    );

    exchange(
        EDIT_REQUEST_ID,
        requested.stored_value(),
        ExchangeStage::Edit,
        &mut exchange_count,
    )?;
    transcript.push(SemanticTranscriptEntry::Request {
        stored_value: requested.stored_value(),
    });
    transcript.push(SemanticTranscriptEntry::Confirmed {
        stored_value: requested.stored_value(),
    });
    journal
        .confirm(pending_edit)
        .map_err(|_| SimulatedEditError::JournalFailure {
            stage: JournalStage::ConfirmEdit,
            exchange_count,
        })?;
    states.push(EditState::ConfirmedEdit {
        stored_value: requested.stored_value(),
    });
    let entry = journal
        .completed_entries()
        .last()
        .ok_or(SimulatedEditError::JournalFailure {
            stage: JournalStage::ConfirmEdit,
            exchange_count,
        })?;
    let previous_stored_value = stored_value(entry.previous_value(), confirmed.decimal_places())
        .ok_or(SimulatedEditError::ProfileInconsistent { exchange_count })?;
    let new_stored_value = stored_value(entry.new_value(), requested.decimal_places())
        .ok_or(SimulatedEditError::ProfileInconsistent { exchange_count })?;
    if entry.target() != TARGET
        || previous_stored_value != confirmed.stored_value()
        || new_stored_value != requested.stored_value()
    {
        return Err(SimulatedEditError::ProfileInconsistent { exchange_count });
    }
    let journal_entry_before_undo =
        ConfirmedJournalEntry::new(TARGET, previous_stored_value, new_stored_value);
    let journal_entries_before_undo = journal.completed_entries().len();

    let proposal = journal
        .prepare_undo()
        .ok_or(SimulatedEditError::JournalFailure {
            stage: JournalStage::PrepareRestoration,
            exchange_count,
        })?;
    let restoration_stored_value =
        stored_value(proposal.restoration_value(), confirmed.decimal_places())
            .ok_or(SimulatedEditError::ProfileInconsistent { exchange_count })?;
    if proposal.target() != TARGET || restoration_stored_value != confirmed.stored_value() {
        return Err(SimulatedEditError::ProfileInconsistent { exchange_count });
    }
    states.push(EditState::PendingRestoration {
        stored_value: restoration_stored_value,
    });

    exchange(
        RESTORATION_REQUEST_ID,
        restoration_stored_value,
        ExchangeStage::Restoration,
        &mut exchange_count,
    )?;
    transcript.push(SemanticTranscriptEntry::Request {
        stored_value: restoration_stored_value,
    });
    transcript.push(SemanticTranscriptEntry::Confirmed {
        stored_value: restoration_stored_value,
    });
    journal
        .confirm_undo(proposal)
        .map_err(|_| SimulatedEditError::JournalFailure {
            stage: JournalStage::ConfirmRestoration,
            exchange_count,
        })?;
    states.push(EditState::ConfirmedRestoration {
        stored_value: restoration_stored_value,
    });

    Ok(SimulatedEditSummary {
        simulator_identity: SimulatorIdentity::SyntheticScripted,
        target: TARGET.to_owned(),
        final_state: TypedParameterState::new(
            TARGET,
            restoration_stored_value,
            confirmed.decimal_places(),
        ),
        states,
        transcript,
        exchange_count,
        journal_entry_before_undo,
        journal_entries_before_undo,
        journal_entries_remaining: journal.completed_entries().len(),
    })
}

fn validate(
    profile: &DeviceProfile,
    stored_value: i32,
    exchange_count: usize,
) -> Result<ValidatedParameterMutation, SimulatedEditError> {
    validate_parameter_mutation(
        profile,
        ParameterMutationRequest::new(TARGET_BLOCK_ID, TARGET_PARAMETER_ID, stored_value),
    )
    .map_err(|source| SimulatedEditError::Validation {
        source,
        exchange_count,
    })
}

fn exchange(
    request_id: u64,
    stored_value: i32,
    stage: ExchangeStage,
    exchange_count: &mut usize,
) -> Result<(), SimulatedEditError> {
    *exchange_count = exchange_count.saturating_add(1);
    let request = ScriptedRequest::new(request_id, synthetic_payload("request", stored_value));
    let response = ScriptedResponse::new(request_id, synthetic_payload("confirmed", stored_value));
    let mut transport = ScriptedTransport::new(request.clone(), response);
    let response =
        transport
            .exchange(request)
            .map_err(|_| SimulatedEditError::ExchangeRejected {
                stage,
                exchange_count: *exchange_count,
            })?;
    if response.request_id != request_id {
        return Err(SimulatedEditError::ExchangeRejected {
            stage,
            exchange_count: *exchange_count,
        });
    }
    Ok(())
}

fn synthetic_payload(kind: &str, stored_value: i32) -> SyntheticPayload {
    SyntheticPayload::from_bytes(format!("synthetic:{kind}:{TARGET}:{stored_value}").as_bytes())
}

fn display_value(stored_value: i32, decimal_places: u8) -> f64 {
    stored_value as f64 / 10_f64.powi(i32::from(decimal_places))
}

fn stored_value(display_value: f64, decimal_places: u8) -> Option<i32> {
    if !display_value.is_finite() {
        return None;
    }
    let scaled = display_value * 10_f64.powi(i32::from(decimal_places));
    if !scaled.is_finite() || scaled.fract() != 0.0 {
        return None;
    }
    (scaled >= f64::from(i32::MIN) && scaled <= f64::from(i32::MAX)).then_some(scaled as i32)
}
