use crossbeam_channel::{unbounded, Sender, Receiver};
use std::thread::{self, JoinHandle};
use crate::transport::Transport;
use crate::transport::uia;

/// A very light wrapper around a background thread that will receive prompts
/// and (later) forward them to the selected provider's transport.
pub struct WorkerHandle {
    tx: Sender<WorkerCmd>,
    _handle: JoinHandle<()>,
}

#[derive(Debug)]
pub enum WorkerCmd {
    Prompt { provider: String, model: String, text: String },
    // Future: Cancel, ReloadConfig, etc.
}

impl WorkerHandle {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        let handle = thread::spawn(move || Self::worker_loop(rx));
        Self { tx, _handle: handle }
    }

    pub fn send_prompt(&self, provider: String, model: String, text: String) {
        let _ = self.tx.send(WorkerCmd::Prompt { provider, model, text });
    }

    fn worker_loop(rx: Receiver<WorkerCmd>) {
        // Simple dispatcher – for now only Kimi via UIA is implemented.
        // In a real version each Provider would have its own struct implementing
        // the Transport trait.
        for cmd in rx {
            match cmd {
                WorkerCmd::Prompt { provider, model: _model, text } => {
                    // Very naive match – extend later.
                    if provider.eq_ignore_ascii_case("Kimi") {
                        if let Err(e) = uia::KimiTransport::new().and_then(|t| t.submit(&text)) {
                            eprintln!("Kimi transport error: {}", e);
                        }
                    } else {
                        eprintln!("Provider '{}' not implemented yet", provider);
                    }
                }
            }
        }
    }
}
