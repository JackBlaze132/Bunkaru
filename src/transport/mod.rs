use std::error::Error;
use std::fmt;

pub mod uia;
pub mod cdp;
pub mod keyboard;

/// Common capabilities that a transport can advertise.
#[derive(Debug, Default, Clone, Copy)]
pub struct Capabilities {
    pub can_set_text: bool,
    pub can_invoke_send: bool,
    pub can_stream: bool,
    pub can_read_reply: bool,
}

/// Result of a capability probe.
#[derive(Debug)]
pub struct ProbeResult {
    pub caps: Capabilities,
    /// Human readable reason for any missing capability.
    pub notes: Vec<String>,
}

/// Trait implemented by each concrete transport (UIA, CDP, Keyboard).
pub trait Transport: Sized {
    /// Returns the provider name (e.g., "Kimi").
    fn provider_name(&self) -> &'static str;

    /// Probe the target client for supported patterns.
    fn probe(&self) -> Result<ProbeResult, TransportError>;

    /// Submit a prompt to the client. Must be non‑blocking; later you call `wait_reply`.
    fn submit(&self, prompt: &str) -> Result<(), TransportError>;

    /// Block until a reply is ready (or timeout). Returns the raw text.
    fn wait_reply(&self) -> Result<String, TransportError>;
}

#[derive(Debug)]
pub enum TransportError {
    UiAutomation(String),
    Cdp(String),
    Keyboard(String),
    Other(String),
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for TransportError {}
