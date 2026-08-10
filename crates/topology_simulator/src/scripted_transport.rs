//! A transport-free, deterministic request/response script.

/// Identity of the peer implemented by [`ScriptedTransport`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SimulatorIdentity {
    /// Synthetic test peer; this is not a modeler or a hardware endpoint.
    SyntheticScripted,
}

impl SimulatorIdentity {
    /// Stable, human-readable identity for transcript and diagnostics.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyntheticScripted => "rigwarden.synthetic-scripted-simulator",
        }
    }
}

/// Bytes used by the simulator's synthetic script, never vendor protocol data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntheticPayload(Vec<u8>);

impl SyntheticPayload {
    /// Copies a synthetic, non-vendor payload into the script.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    /// Returns the payload bytes for assertions and diagnostics.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Request expected by a scripted exchange.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScriptedRequest {
    /// Deterministic request correlation identifier.
    pub id: u64,
    /// Synthetic request payload.
    pub payload: SyntheticPayload,
}

impl ScriptedRequest {
    /// Creates a request with a correlation ID and synthetic payload.
    pub fn new(id: u64, payload: SyntheticPayload) -> Self {
        Self { id, payload }
    }
}

/// Response returned by a scripted exchange.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScriptedResponse {
    /// Request ID to which this response is correlated.
    pub request_id: u64,
    /// Synthetic response payload.
    pub payload: SyntheticPayload,
}

impl ScriptedResponse {
    /// Creates a response correlated to a request ID.
    pub fn new(request_id: u64, payload: SyntheticPayload) -> Self {
        Self {
            request_id,
            payload,
        }
    }
}

/// Ordered events recorded by a scripted transport.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TranscriptEntry {
    /// A request sent to the synthetic peer.
    Sent(ScriptedRequest),
    /// A response received from the synthetic peer.
    Received(ScriptedResponse),
}

/// Structured explanation for a request that does not match the script.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScriptMismatch {
    /// ID expected by the script.
    pub expected_id: u64,
    /// Payload expected by the script.
    pub expected_payload: SyntheticPayload,
    /// ID supplied by the caller.
    pub actual_id: u64,
    /// Payload supplied by the caller.
    pub actual_payload: SyntheticPayload,
}

/// In-memory scripted request/response transport.
///
/// It has no clock, network, hardware endpoint, retries, or protocol-byte
/// interpretation. Every matching request receives the configured response.
#[derive(Clone, Debug)]
pub struct ScriptedTransport {
    expected_request: ScriptedRequest,
    response: ScriptedResponse,
    transcript: Vec<TranscriptEntry>,
}

impl ScriptedTransport {
    /// Creates a deterministic one-request script with a synthetic response.
    pub fn new(expected_request: ScriptedRequest, response: ScriptedResponse) -> Self {
        Self {
            expected_request,
            response,
            transcript: Vec::new(),
        }
    }

    /// Identifies this peer as the synthetic simulator, not hardware.
    pub const fn identity(&self) -> SimulatorIdentity {
        SimulatorIdentity::SyntheticScripted
    }

    /// Sends one request through the script and returns its correlated response.
    pub fn exchange(
        &mut self,
        request: ScriptedRequest,
    ) -> Result<ScriptedResponse, ScriptMismatch> {
        self.transcript.push(TranscriptEntry::Sent(request.clone()));

        if request != self.expected_request {
            return Err(ScriptMismatch {
                expected_id: self.expected_request.id,
                expected_payload: self.expected_request.payload.clone(),
                actual_id: request.id,
                actual_payload: request.payload,
            });
        }

        let response = self.response.clone();
        self.transcript
            .push(TranscriptEntry::Received(response.clone()));
        Ok(response)
    }

    /// Returns the ordered send/receive transcript.
    pub fn transcript(&self) -> &[TranscriptEntry] {
        &self.transcript
    }
}
