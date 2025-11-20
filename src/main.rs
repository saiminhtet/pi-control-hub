use eframe::egui;
use app::PiControlApp;

mod app;
mod modules;
// TODO: Implement these modules
// mod hardware;
// mod utils;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 320.0])  // 3.5" screen size
            .with_resizable(false)             // Fixed size for touch optimization
            .with_decorations(true),
        vsync: true,                           // Smooth rendering
        centered: true,                        // Center on screen
        ..Default::default()
    };

    eframe::run_native(
        "Pi Control Hub",
        options,
        Box::new(|cc| {
            Box::new(PiControlApp::new(cc))
        }),
    )
}