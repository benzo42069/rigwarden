//! Typed semantic application session for the deterministic synthetic edit.
//!
//! This module is the only bridge-facing orchestration for the first Flutter
//! vertical slice.  It keeps profile validation, scripted simulator payloads,
//! and the undo journal in Rust and returns only typed state events.  No
//! transport handle or payload bytes cross the generated boundary.

use topology_command_engine::{
    validate_parameter_mutation, MutationValidationError, ParameterMutationRequest,
    ValidatedParameterMutation,
};
use topology_device_registry::{
    DeviceProfile, NumericParameterMetadata, SessionCapabilities, VerificationStatus,
};
use topology_domain::{DeviceFamilyId, DeviceModelId, FirmwareId};
use topology_simulator::{
    ScriptedRequest, ScriptedResponse, ScriptedTransport, SimulatorIdentity, SyntheticPayload,
};
use topology_undo::{Journal, PendingMutationId};

const TARGET_BLOCK_ID: &str = "amp-1";
const TARGET_PARAMETER_ID: &str = "gain";
const TARGET: &str = "amp-1/gain";
const PRESET: &str = "synthetic-preset";
const INITIAL_STORED_VALUE: i32 = 30;
const EDIT_REQUEST_ID: u64 = 1;
const RESTORATION_REQUEST_ID: u64 = 2;
const SYNTHETIC_CONTEXT: &str = "synthetic preset / Amp 1 / gain";
const SYNTHETIC_UNIT: &str = "synthetic stored units";
const STEP_STORED: i32 = 1;

/// Structured rejection returned by the typed bridge for a refused semantic
/// action. It carries only a stable reason and exchange count; no transport
/// bytes or endpoint details cross the boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SimulatedParameterEditErrorCode {
    ReadOnly,
    ProfileNotWritable,
    ParameterNotDeclared,
    OutOfRange,
    PendingAction,
    NoPendingEdit,
    NoPendingUndo,
    JournalFailure,
    ProfileInconsistent,
    SyntheticExchangeRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimulatedParameterEditError {
    pub code: SimulatedParameterEditErrorCode,
    pub message: String,
    pub exchange_count: u32,
}

/// Typed state phases emitted by the application session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SimulatedParameterEditPhase {
    /// Initial confirmed value before an edit.
    Idle,
    /// A validated edit is staged and awaiting synthetic confirmation.
    PendingEdit,
    /// The synthetic peer confirmed the requested edit.
    ConfirmedEdit,
    /// A journal restoration is staged and awaiting synthetic confirmation.
    PendingUndo,
    /// The synthetic peer confirmed the restoration.
    ConfirmedUndo,
}

/// Kind of one byte-free semantic transcript entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SimulatedParameterEditTranscriptKind {
    /// A typed request sent to the synthetic peer.
    Request,
    /// A typed confirmation received from the synthetic peer.
    Confirmed,
}

/// One sanitized semantic transcript entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimulatedParameterEditTranscriptEntry {
    pub kind: SimulatedParameterEditTranscriptKind,
    pub stored_value: i32,
}

/// Typed application state returned after each semantic session action.
#[derive(Clone, Debug, PartialEq)]
pub struct SimulatedParameterEditState {
    pub phase: SimulatedParameterEditPhase,
    pub target: String,
    pub context: String,
    pub unit: String,
    pub stored_value: i32,
    pub display_value: f64,
    pub decimal_places: u8,
    pub min_stored: i32,
    pub max_stored: i32,
    pub step_stored: i32,
    pub step_display: f64,
    pub read_only: bool,
    pub error_message: Option<String>,
    pub journal_previous_stored_value: Option<i32>,
    pub journal_new_stored_value: Option<i32>,
    pub journal_entry_count: u32,
    pub exchange_count: u32,
    pub simulator_label: String,
    pub transcript: Vec<SimulatedParameterEditTranscriptEntry>,
}

/// Rust-owned stateful semantic session for one synthetic parameter edit.
///
/// The opaque value owns the journal and scripted exchange state. Dart can
/// request only the bounded semantic actions below; it cannot construct a
/// profile, send arbitrary data, or access transport endpoints.
#[flutter_rust_bridge::frb(opaque)]
pub struct SimulatedParameterEditSession {
    profile: DeviceProfile,
    current_stored_value: i32,
    phase: SimulatedParameterEditPhase,
    journal: Journal,
    pending_edit: Option<PendingMutationId>,
    pending_edit_stored_value: Option<i32>,
    pending_undo: bool,
    journal_previous_stored_value: Option<i32>,
    journal_new_stored_value: Option<i32>,
    exchange_count: usize,
    transcript: Vec<SimulatedParameterEditTranscriptEntry>,
}

/// Create a fresh session for the exact writable synthetic profile.
pub fn create_simulated_parameter_edit_session() -> SimulatedParameterEditSession {
    create_session(synthetic_profile(true))
}

/// Create a fresh session for the explicitly read-only synthetic profile.
pub fn create_simulated_read_only_parameter_edit_session() -> SimulatedParameterEditSession {
    create_session(synthetic_profile(false))
}

fn create_session(profile: DeviceProfile) -> SimulatedParameterEditSession {
    SimulatedParameterEditSession {
        profile,
        current_stored_value: INITIAL_STORED_VALUE,
        phase: SimulatedParameterEditPhase::Idle,
        journal: Journal::new_with_preset(PRESET),
        pending_edit: None,
        pending_edit_stored_value: None,
        pending_undo: false,
        journal_previous_stored_value: None,
        journal_new_stored_value: None,
        exchange_count: 0,
        transcript: Vec::new(),
    }
}

impl SimulatedParameterEditSession {
    /// Return the current typed state without exposing Rust-owned internals.
    pub fn initial_state(&self) -> SimulatedParameterEditState {
        self.state()
    }

    /// Validate and stage one bounded semantic edit.
    pub fn begin_edit(
        &mut self,
        requested_stored_value: i32,
    ) -> Result<SimulatedParameterEditState, SimulatedParameterEditError> {
        if self.pending_edit.is_some() || self.pending_undo {
            return Err(self.error(
                SimulatedParameterEditErrorCode::PendingAction,
                "another synthetic semantic action is pending",
            ));
        }

        let confirmed = validate(&self.profile, self.current_stored_value)?;
        let requested = validate(&self.profile, requested_stored_value)?;
        if confirmed.decimal_places() != requested.decimal_places() {
            return Err(self.error(
                SimulatedParameterEditErrorCode::ProfileInconsistent,
                "synthetic profile precision changed during validation",
            ));
        }

        let pending = self.journal.begin_parameter_change(
            TARGET,
            display_value(confirmed.stored_value(), confirmed.decimal_places()),
            display_value(requested.stored_value(), requested.decimal_places()),
        );
        self.pending_edit = Some(pending);
        self.pending_edit_stored_value = Some(requested.stored_value());
        self.phase = SimulatedParameterEditPhase::PendingEdit;
        Ok(self.state_with_value(requested, requested_stored_value))
    }

    /// Send the staged edit to the scripted simulator and confirm its journal entry.
    pub fn confirm_edit(
        &mut self,
    ) -> Result<SimulatedParameterEditState, SimulatedParameterEditError> {
        let pending = self.pending_edit.take().ok_or_else(|| {
            self.error(
                SimulatedParameterEditErrorCode::NoPendingEdit,
                "no pending synthetic semantic edit",
            )
        })?;
        let requested_stored_value = self.pending_edit_stored_value.take().ok_or_else(|| {
            self.error(
                SimulatedParameterEditErrorCode::ProfileInconsistent,
                "pending synthetic semantic edit value is missing",
            )
        })?;
        let requested = validate(&self.profile, requested_stored_value)?;
        exchange(
            EDIT_REQUEST_ID,
            requested.stored_value(),
            &mut self.exchange_count,
            &mut self.transcript,
        )?;
        self.journal.confirm(pending).map_err(|_| {
            self.error(
                SimulatedParameterEditErrorCode::JournalFailure,
                "synthetic journal edit confirmation failed",
            )
        })?;

        self.current_stored_value = requested.stored_value();
        self.phase = SimulatedParameterEditPhase::ConfirmedEdit;
        let entry = self.journal.completed_entries().last().ok_or_else(|| {
            self.error(
                SimulatedParameterEditErrorCode::JournalFailure,
                "synthetic journal edit entry missing after confirmation",
            )
        })?;
        self.journal_previous_stored_value = stored_value(entry.previous_value(), &requested);
        self.journal_new_stored_value = stored_value(entry.new_value(), &requested);
        Ok(self.state_with_value(requested, self.current_stored_value))
    }

    /// Stage restoration of the exact confirmed prior journal value.
    pub fn begin_undo(
        &mut self,
    ) -> Result<SimulatedParameterEditState, SimulatedParameterEditError> {
        if self.pending_edit.is_some() || self.pending_undo {
            return Err(self.error(
                SimulatedParameterEditErrorCode::PendingAction,
                "another synthetic semantic action is pending",
            ));
        }
        let entry = self.journal.completed_entries().last().ok_or_else(|| {
            self.error(
                SimulatedParameterEditErrorCode::NoPendingUndo,
                "no confirmed synthetic journal entry to undo",
            )
        })?;
        let current = validate(&self.profile, self.current_stored_value)?;
        let restoration = stored_value(entry.previous_value(), &current).ok_or_else(|| {
            self.error(
                SimulatedParameterEditErrorCode::ProfileInconsistent,
                "synthetic journal restoration value is not profile-aligned",
            )
        })?;
        self.pending_undo = true;
        self.phase = SimulatedParameterEditPhase::PendingUndo;
        Ok(self.state_with_value(current, restoration))
    }

    /// Prepare and send the restoration, then consume the confirmed journal entry.
    pub fn confirm_undo(
        &mut self,
    ) -> Result<SimulatedParameterEditState, SimulatedParameterEditError> {
        if !self.pending_undo {
            return Err(self.error(
                SimulatedParameterEditErrorCode::NoPendingUndo,
                "no pending synthetic semantic undo",
            ));
        }
        let current = validate(&self.profile, self.current_stored_value)?;
        let proposal = self.journal.prepare_undo().ok_or_else(|| {
            self.error(
                SimulatedParameterEditErrorCode::JournalFailure,
                "synthetic journal undo preparation failed",
            )
        })?;
        let restoration =
            stored_value(proposal.restoration_value(), &current).ok_or_else(|| {
                self.error(
                    SimulatedParameterEditErrorCode::ProfileInconsistent,
                    "synthetic journal restoration value is not profile-aligned",
                )
            })?;
        exchange(
            RESTORATION_REQUEST_ID,
            restoration,
            &mut self.exchange_count,
            &mut self.transcript,
        )?;
        self.journal.confirm_undo(proposal).map_err(|_| {
            self.error(
                SimulatedParameterEditErrorCode::JournalFailure,
                "synthetic journal undo confirmation failed",
            )
        })?;

        self.pending_undo = false;
        self.current_stored_value = restoration;
        self.phase = SimulatedParameterEditPhase::ConfirmedUndo;
        Ok(self.state_with_value(current, restoration))
    }

    fn state(&self) -> SimulatedParameterEditState {
        let metadata = self
            .profile
            .numeric_parameter(TARGET_BLOCK_ID, TARGET_PARAMETER_ID)
            .expect("synthetic profile declares amp-1/gain");
        SimulatedParameterEditState {
            phase: self.phase,
            target: TARGET.to_owned(),
            context: SYNTHETIC_CONTEXT.to_owned(),
            unit: SYNTHETIC_UNIT.to_owned(),
            stored_value: self.current_stored_value,
            display_value: display_value(self.current_stored_value, metadata.decimal_places()),
            decimal_places: metadata.decimal_places(),
            min_stored: metadata.min_stored(),
            max_stored: metadata.max_stored(),
            step_stored: STEP_STORED,
            step_display: display_value(STEP_STORED, metadata.decimal_places()),
            read_only: self.read_only(),
            error_message: self
                .read_only()
                .then(|| "read-only synthetic profile rejects writes".to_owned()),
            journal_previous_stored_value: self.journal_previous_stored_value,
            journal_new_stored_value: self.journal_new_stored_value,
            journal_entry_count: self.journal.completed_entries().len() as u32,
            exchange_count: self.exchange_count as u32,
            simulator_label: SimulatorIdentity::SyntheticScripted.as_str().to_owned(),
            transcript: self.transcript.clone(),
        }
    }

    fn state_with_value(
        &self,
        validated: ValidatedParameterMutation,
        stored_value: i32,
    ) -> SimulatedParameterEditState {
        let mut state = self.state();
        state.stored_value = stored_value;
        state.display_value = display_value(stored_value, validated.decimal_places());
        state.decimal_places = validated.decimal_places();
        state
    }

    fn read_only(&self) -> bool {
        !self.profile.capabilities().can_write()
            || self.profile.verification_status() == VerificationStatus::ReadOnly
    }

    fn error(
        &self,
        code: SimulatedParameterEditErrorCode,
        message: impl Into<String>,
    ) -> SimulatedParameterEditError {
        SimulatedParameterEditError {
            code,
            message: message.into(),
            exchange_count: self.exchange_count as u32,
        }
    }
}

fn synthetic_profile(writable: bool) -> DeviceProfile {
    DeviceProfile::new(
        DeviceFamilyId::new("synthetic-family").expect("synthetic family is valid"),
        DeviceModelId::new("synthetic-model").expect("synthetic model is valid"),
        FirmwareId::new("synthetic-1").expect("synthetic firmware is valid"),
        SessionCapabilities::new(writable),
        if writable {
            VerificationStatus::Experimental
        } else {
            VerificationStatus::ReadOnly
        },
    )
    .with_numeric_parameter(
        TARGET_BLOCK_ID,
        TARGET_PARAMETER_ID,
        NumericParameterMetadata::new(0, 100, 1),
    )
}

fn validate(
    profile: &DeviceProfile,
    stored_value: i32,
) -> Result<ValidatedParameterMutation, SimulatedParameterEditError> {
    validate_parameter_mutation(
        profile,
        ParameterMutationRequest::new(TARGET_BLOCK_ID, TARGET_PARAMETER_ID, stored_value),
    )
    .map_err(|error| validation_error(error, 0))
}

fn validation_error(
    error: MutationValidationError,
    exchange_count: usize,
) -> SimulatedParameterEditError {
    let (code, message) = match error {
        MutationValidationError::ReadOnly { .. } => (
            SimulatedParameterEditErrorCode::ReadOnly,
            "read-only synthetic profile rejects writes".to_owned(),
        ),
        MutationValidationError::ProfileNotWritable => (
            SimulatedParameterEditErrorCode::ProfileNotWritable,
            "synthetic profile does not permit writes".to_owned(),
        ),
        MutationValidationError::ParameterNotDeclared {
            block_id,
            parameter_id,
        } => (
            SimulatedParameterEditErrorCode::ParameterNotDeclared,
            format!("synthetic profile does not declare {block_id}/{parameter_id}"),
        ),
        MutationValidationError::OutOfRange {
            requested_stored,
            min_stored,
            max_stored,
        } => (
            SimulatedParameterEditErrorCode::OutOfRange,
            format!(
                "synthetic stored value {requested_stored} is outside profile range {min_stored}..{max_stored}"
            ),
        ),
    };
    SimulatedParameterEditError {
        code,
        message,
        exchange_count: exchange_count as u32,
    }
}

fn exchange(
    request_id: u64,
    stored_value: i32,
    exchange_count: &mut usize,
    transcript: &mut Vec<SimulatedParameterEditTranscriptEntry>,
) -> Result<(), SimulatedParameterEditError> {
    *exchange_count = exchange_count.saturating_add(1);
    let request = ScriptedRequest::new(request_id, synthetic_payload("request", stored_value));
    let response = ScriptedResponse::new(request_id, synthetic_payload("confirmed", stored_value));
    let mut transport = ScriptedTransport::new(request.clone(), response);
    let response = transport
        .exchange(request)
        .map_err(|_| SimulatedParameterEditError {
            code: SimulatedParameterEditErrorCode::SyntheticExchangeRejected,
            message: "synthetic simulator rejected the semantic request".to_owned(),
            exchange_count: *exchange_count as u32,
        })?;
    if response.request_id != request_id {
        return Err(SimulatedParameterEditError {
            code: SimulatedParameterEditErrorCode::SyntheticExchangeRejected,
            message: "synthetic simulator returned an unmatched confirmation".to_owned(),
            exchange_count: *exchange_count as u32,
        });
    }
    transcript.push(SimulatedParameterEditTranscriptEntry {
        kind: SimulatedParameterEditTranscriptKind::Request,
        stored_value,
    });
    transcript.push(SimulatedParameterEditTranscriptEntry {
        kind: SimulatedParameterEditTranscriptKind::Confirmed,
        stored_value,
    });
    Ok(())
}

fn synthetic_payload(kind: &str, stored_value: i32) -> SyntheticPayload {
    SyntheticPayload::from_bytes(format!("synthetic:{kind}:{TARGET}:{stored_value}").as_bytes())
}

fn display_value(stored_value: i32, decimal_places: u8) -> f64 {
    stored_value as f64 / 10_f64.powi(i32::from(decimal_places))
}

fn stored_value(display_value: f64, validated: &ValidatedParameterMutation) -> Option<i32> {
    if !display_value.is_finite() {
        return None;
    }
    let scaled = display_value * 10_f64.powi(i32::from(validated.decimal_places()));
    if !scaled.is_finite() || scaled.fract() != 0.0 {
        return None;
    }
    (scaled >= f64::from(i32::MIN) && scaled <= f64::from(i32::MAX)).then_some(scaled as i32)
}
