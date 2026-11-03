use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    /// Optional UI automation selector that switches the model inside the client.
    /// If None, the orchestrator will only display the model name.
    pub switch_selector: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Human readable name (e.g. "Kimi Desktop")
    pub display_name: String,
    /// Path to the executable (used for focus / launching)
    pub exe_path: String,
    /// Window class or partial title for matching the main window.
    pub window_match: String,
    /// List of models this provider offers.
    pub models: Vec<ModelConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// Map of provider id -> provider settings. Flattened so `providers.json`
    /// can be a plain map without a top-level `providers` wrapper.
    #[serde(flatten)]
    pub providers: HashMap<String, ProviderConfig>,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new("providers.json");
        let txt = fs::read_to_string(path)?;
        let cfg: Self = serde_json::from_str(&txt)?;
        Ok(cfg)
    }
}
