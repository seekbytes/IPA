use ipa::gui::app::AnalyzerApp;

/// Entry point of the app
fn main() -> eframe::Result {
    env_logger::init();
    let icon = include_bytes!("../data/icon.png");
    let image = image::load_from_memory(icon)
        .expect("Failed to open icon path")
        .to_rgba8();
    let (icon_width, icon_height) = image.dimensions();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(egui::IconData {
            rgba: image.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        ..Default::default()
    };

    eframe::run_native(
        "Interactive PDF Analyzer",
        native_options,
        Box::new(|cc| Ok(Box::new(AnalyzerApp::new(cc)))),
    )
}
