use eframe::egui::{self, ComboBox, TextEdit};
use crate::config::AppConfig;
use crate::worker::WorkerHandle;
use std::sync::Arc;

pub struct BunkaruApp {
    config: Arc<AppConfig>,
    // UI state
    selected_provider: Option<String>,
    selected_model: Option<String>,
    input: String,
    chat: Vec<String>,
    worker: WorkerHandle,
}

impl BunkaruApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Load config (panics on error for now)
        let config = Arc::new(AppConfig::load().expect("Failed to load config"));
        // Start background worker (placeholder)
        let worker = WorkerHandle::new();
        Self {
            config,
            selected_provider: None,
            selected_model: None,
            input: String::new(),
            chat: Vec::new(),
            worker,
        }
    }

    fn submit_prompt(&mut self) {
        let prompt = self.input.trim().to_string();
        if prompt.is_empty() {
            return;
        }
        // Append user message to chat
        self.chat.push(format!("**You:** {}", prompt));
        self.input.clear();
        // Send to worker (not implemented yet)
        if let (Some(provider), Some(model)) = (&self.selected_provider, &self.selected_model) {
            self.worker.send_prompt(provider.clone(), model.clone(), prompt);
        } else {
            self.chat
                .push("*Select a provider and model first.*".to_string());
        }
    }
}

impl eframe::App for BunkaruApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |top_ui| {
            top_ui.horizontal(|row_ui| {
                let old_provider = self.selected_provider.clone();
                row_ui.label("Provider:");
                ComboBox::from_id_salt("provider_combo")
                    .selected_text(
                        self.selected_provider
                            .as_deref()
                            .unwrap_or("-- Select --"),
                    )
                    .show_ui(row_ui, |inner_ui| {
                        for prov in self.config.providers.keys() {
                            inner_ui.selectable_value(&mut self.selected_provider, Some(prov.clone()), prov);
                        }
                    });
                    // Reset model when provider changes
                    if old_provider != self.selected_provider {
                        self.selected_model = None;
                    }
                    row_ui.label("Model:");
                ComboBox::from_id_salt("model_combo")
                    .selected_text(
                        self.selected_model
                            .as_deref()
                            .unwrap_or("-- Select --"),
                    )
                    .show_ui(row_ui, |inner_ui| {
                        if let Some(provider) = &self.selected_provider {
                            if let Some(pconf) = self.config.providers.get(provider) {
                                for mdl in &pconf.models {
                                    inner_ui.selectable_value(&mut self.selected_model, Some(mdl.name.clone()), &mdl.name);
                                }
                            }
                        } else {
                            inner_ui.label("Select provider first");
                        }
                    });
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            // Chat history scroll area
            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                for msg in &self.chat {
                    ui.label(msg);
                }
            });
            ui.separator();
            // Input line
            ui.horizontal(|row_ui| {
                let resp = row_ui.add(
                    TextEdit::singleline(&mut self.input)
                        .hint_text("Enter prompt…")
                        .desired_width(row_ui.available_width() - 60.0),
                );
                if resp.lost_focus() && row_ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.submit_prompt();
                }
                if row_ui.button("Send").clicked() {
                    self.submit_prompt();
                }
            });
        });
    }
}
