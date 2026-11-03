// Stub placeholders for CDP‑based transport.
use super::{Transport, TransportError, ProbeResult};

pub struct CdpTransport;

impl CdpTransport {
    pub fn new() -> Result<Self, TransportError> {
        // In a full implementation we would connect to the remote‑debugging port.
        Err(TransportError::Cdp("CDP not implemented yet".into()))
    }
}

impl Transport for CdpTransport {
    fn provider_name(&self) -> &'static str { "Kimi (CDP)" }
    fn probe(&self) -> Result<ProbeResult, TransportError> { Err(TransportError::Cdp("probe not implemented".into())) }
    fn submit(&self, _: &str) -> Result<(), TransportError> { Err(TransportError::Cdp("submit not implemented".into())) }
    fn wait_reply(&self) -> Result<String, TransportError> { Err(TransportError::Cdp("wait_reply not implemented".into())) }
}
