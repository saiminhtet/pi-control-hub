use eframe::egui;
use app::PiControlApp;

mod app;
mod modules;
mod hardware;
mod utils;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 320.0)),
        resizable: false,
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "Pi Control Hub",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(PiControlApp::new(cc))
        }),
    )
}