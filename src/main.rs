mod app;
mod logic;
mod theme;
mod ui;
mod utils;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: app::viewport_builder(),
        ..Default::default()
    };
    eframe::run_native(
        theme::APP_NAME,
        options,
        Box::new(|creation_context| Ok(Box::new(app::ClipboardApp::new(&creation_context.egui_ctx)))),
    )
}
