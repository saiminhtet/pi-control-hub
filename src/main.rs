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
            .with_inner_size([480.0, 320.0])  // Default to Pi screen size
            .with_min_inner_size([320.0, 240.0]) // Minimum size
            .with_resizable(true)              // Enable resizing for different displays
            .with_decorations(true)            // Window controls (minimize, maximize, close)
            .with_maximized(false),            // Start windowed
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