mod app;
mod config;
mod worker;
mod transport;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Bunkaru – Local AI Orchestrator",
        options,
        Box::new(|cc| Ok(Box::new(app::BunkaruApp::new(cc)))),
    )
}
