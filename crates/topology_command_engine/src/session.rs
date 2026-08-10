//! Connection-generation-aware request correlation for command sessions.

use std::collections::{BTreeMap, BTreeSet};

/// Monotonically increasing identity for one connection lifetime.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct ConnectionGeneration(u64);

impl ConnectionGeneration {
    /// Starts the first connection at generation one.
    pub const INITIAL: Self = Self(1);

    /// Creates a generation from its persisted numeric representation.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the stable numeric representation used in diagnostics.
    pub const fn value(self) -> u64 {
        self.0
    }

    fn next(self) -> Self {
        Self(
            self.0
                .checked_add(1)
                .expect("connection generation exhausted"),
        )
    }
}

/// Correlation key for one request within one connection generation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct RequestKey {
    generation: ConnectionGeneration,
    request_id: u64,
}

impl RequestKey {
    const fn new(generation: ConnectionGeneration, request_id: u64) -> Self {
        Self {
            generation,
            request_id,
        }
    }

    /// Returns the connection generation that owns the request.
    pub const fn generation(self) -> u64 {
        self.generation.value()
    }

    /// Returns the request identifier scoped by this generation.
    pub const fn request_id(self) -> u64 {
        self.request_id
    }
}

/// Response delivered by a transport to the command session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncomingResponse {
    generation: ConnectionGeneration,
    request_id: u64,
    payload: Vec<u8>,
}

impl IncomingResponse {
    /// Creates a response with its connection generation and request ID.
    pub fn new(generation: u64, request_id: u64, payload: impl AsRef<[u8]>) -> Self {
        Self {
            generation: ConnectionGeneration::new(generation),
            request_id,
            payload: payload.as_ref().to_vec(),
        }
    }

    /// Returns the generation attached to the response.
    pub const fn generation(&self) -> u64 {
        self.generation.value()
    }

    /// Returns the request ID attached to the response.
    pub const fn request_id(&self) -> u64 {
        self.request_id
    }

    /// Returns the opaque response payload without interpreting protocol bytes.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

/// Result of attempting to reconcile one incoming response.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseDisposition {
    /// The response belongs to an earlier or otherwise different generation.
    IgnoredStale,
    /// The response matched the current pending request and was confirmed.
    Confirmed,
    /// The response used the current generation but had no pending match.
    RejectedUnmatched,
}

/// In-memory command-session correlation state.
#[derive(Clone, Debug)]
pub struct ConnectionSession {
    current_generation: ConnectionGeneration,
    pending: BTreeSet<RequestKey>,
    confirmed: BTreeMap<RequestKey, IncomingResponse>,
}

impl ConnectionSession {
    /// Creates a session at the first connection generation.
    pub fn new() -> Self {
        Self {
            current_generation: ConnectionGeneration::INITIAL,
            pending: BTreeSet::new(),
            confirmed: BTreeMap::new(),
        }
    }

    /// Returns the generation currently allowed to confirm requests.
    pub const fn current_generation(&self) -> u64 {
        self.current_generation.value()
    }

    /// Advances the connection generation and discards pending work from the
    /// disconnected lifetime.
    pub fn reconnect(&mut self) {
        self.current_generation = self.current_generation.next();
        self.pending.clear();
    }

    /// Registers a request in the current generation.
    pub fn issue_request(&mut self, request_id: u64) -> RequestKey {
        let key = RequestKey::new(self.current_generation, request_id);
        self.pending.insert(key);
        key
    }

    /// Reconciles one response without allowing an old generation to mutate
    /// current or confirmed state.
    pub fn process_response(&mut self, response: IncomingResponse) -> ResponseDisposition {
        if response.generation != self.current_generation {
            return ResponseDisposition::IgnoredStale;
        }

        let key = RequestKey::new(self.current_generation, response.request_id);
        if !self.pending.remove(&key) {
            return ResponseDisposition::RejectedUnmatched;
        }

        self.confirmed.insert(key, response);
        ResponseDisposition::Confirmed
    }

    /// Reports whether a request is still awaiting confirmation.
    pub fn is_pending(&self, request: RequestKey) -> bool {
        self.pending.contains(&request)
    }

    /// Returns the confirmed response for a request, if one was accepted.
    pub fn confirmed_response(&self, request: RequestKey) -> Option<&IncomingResponse> {
        self.confirmed.get(&request)
    }
}

impl Default for ConnectionSession {
    fn default() -> Self {
        Self::new()
    }
}
