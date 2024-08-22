use egui::Context;

/// A simple window that explains what objects are, how a PDF file is composed and a few information
pub struct HowItWorksWindow {}

impl HowItWorksWindow {
    /// Show the about window
    pub fn show(how_it_works_window: &mut bool, ctx: &Context) {
        // about panel
        egui::Window::new("How it works")
            .collapsible(false)
            .resizable(false)
            .title_bar(true)
            .open(how_it_works_window)
            .show(ctx, |ui| {
               ui.label("IPA is an application based on pdf-rs, a Rust library designed to provide a high-level interface for interacting with PDF documents. It offers a variety of functionalities, including extracting objects and content from PDF.");

            });
    }
}
