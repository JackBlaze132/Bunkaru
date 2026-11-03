// Very simple fall‑back that uses enigo to type and arboard to copy.
use super::{Transport, TransportError, ProbeResult, Capabilities};
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use arboard::Clipboard;
use std::sync::Mutex;
use std::time::Duration;
use std::thread::sleep;

pub struct KeyboardTransport {
    enigo: Mutex<Enigo>,
    clipboard: Mutex<Clipboard>,
    /// Clipboard contents captured right before we paste the prompt, so that
    /// `wait_reply` does not mistake our own prompt for a reply.
    baseline: Mutex<String>,
}

impl KeyboardTransport {
    pub fn new() -> Result<Self, TransportError> {
        let enigo = Enigo::new(&Settings::default())
            .map_err(|e| TransportError::Keyboard(format!("Enigo init: {:?}", e)))?;
        let clipboard = Clipboard::new()
            .map_err(|e| TransportError::Keyboard(format!("Clipboard init: {:?}", e)))?;
        Ok(Self {
            enigo: Mutex::new(enigo),
            clipboard: Mutex::new(clipboard),
            baseline: Mutex::new(String::new()),
        })
    }
}

impl Transport for KeyboardTransport {
    fn provider_name(&self) -> &'static str { "Keyboard (fallback)" }
    fn probe(&self) -> Result<ProbeResult, TransportError> {
        Ok(ProbeResult { caps: Capabilities { can_set_text: true, can_invoke_send: true, can_stream: false, can_read_reply: true }, notes: vec!["Fallback uses clipboard".into()] })
    }
    fn submit(&self, prompt: &str) -> Result<(), TransportError> {
        // Copy prompt to clipboard then Ctrl+V + Enter.
        {
            let mut clipboard = self.clipboard.lock().unwrap();
            if let Ok(current) = clipboard.get_text() {
                *self.baseline.lock().unwrap() = current;
            }
            clipboard
                .set_text(prompt)
                .map_err(|e| TransportError::Keyboard(format!("Set clipboard: {:?}", e)))?;
        }
        // Assume the target window is already focused (caller should focus).
        let mut enigo = self.enigo.lock().unwrap();
        enigo
            .key(Key::Control, Direction::Press)
            .map_err(|e| TransportError::Keyboard(format!("Key press: {:?}", e)))?;
        enigo
            .key(Key::Unicode('v'), Direction::Click)
            .map_err(|e| TransportError::Keyboard(format!("Key press: {:?}", e)))?;
        enigo
            .key(Key::Control, Direction::Release)
            .map_err(|e| TransportError::Keyboard(format!("Key release: {:?}", e)))?;
        enigo
            .key(Key::Return, Direction::Click)
            .map_err(|e| TransportError::Keyboard(format!("Key press: {:?}", e)))?;
        Ok(())
    }
    fn wait_reply(&self) -> Result<String, TransportError> {
        // Simple poll for clipboard change – not reliable but works for many apps.
        let baseline = self.baseline.lock().unwrap().clone();
        for _ in 0..60 {
            sleep(Duration::from_millis(300));
            let txt = self
                .clipboard
                .lock()
                .unwrap()
                .get_text()
                .map_err(|e| TransportError::Keyboard(format!("Get clipboard: {:?}", e)))?;
            if !txt.is_empty() && txt != baseline {
                return Ok(txt);
            }
        }
        Err(TransportError::Keyboard("Reply timeout".into()))
    }
}
