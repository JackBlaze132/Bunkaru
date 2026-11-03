use super::{Transport, TransportError, ProbeResult};

pub struct KimiTransport;

impl KimiTransport {
    pub fn new() -> Result<Self, TransportError> {
        Err(TransportError::Other("UIA transport not implemented in this scaffold".into()))
    }
}

impl Transport for KimiTransport {
    fn provider_name(&self) -> &'static str { "Kimi (UIA stub)" }
    fn probe(&self) -> Result<ProbeResult, TransportError> { Err(TransportError::Other("probe not implemented".into())) }
    fn submit(&self, _prompt: &str) -> Result<(), TransportError> { Err(TransportError::Other("submit not implemented".into())) }
    fn wait_reply(&self) -> Result<String, TransportError> { Err(TransportError::Other("wait_reply not implemented".into())) }
}
