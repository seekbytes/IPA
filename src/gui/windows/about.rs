use eframe::epaint::Color32;
use egui::{include_image, Context, FontId, RichText, Vec2};

/// Helper struct for the about window
pub struct AboutWindow {}

impl AboutWindow {
    /// Show the about window
    pub fn show(about_window: &mut bool, ctx: &Context) {
        // about panel
        egui::Window::new("About IPA")
            .collapsible(false)
            .resizable(false)
            .title_bar(true)
            .open(about_window)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                   ui.add(egui::Image::new(include_image!("../../../data/icon.png")).fit_to_original_size(1.0).max_size(Vec2::new(100f32, 200f32)));

                       ui.vertical(|ui| {
                           ui.label(
                               RichText::new("Interactive PDF Analysis")
                                   .size(16.0)
                                   .color(Color32::from_hex("#fff").unwrap()),
                           );
                           ui.label(
                               RichText::new(
                                   "A proof of concept analyzer that targets PDF file based on pdf-rs.",
                               )
                                   .color(Color32::from_hex("#fff").unwrap()),
                           );

                           ui.horizontal(|ui| {
                               ui.label("Author");
                               ui.label(
                                   RichText::new("Seekbytes")
                                       .font(FontId::monospace(12.0))
                                       .color(Color32::from_hex("#fff").unwrap()),
                               );
                           });

                           ui.horizontal(|ui| {
                               ui.label("Version");
                               ui.label(
                                   RichText::new("0.0.1")
                                       .font(FontId::monospace(12.0))
                                       .color(Color32::from_hex("#fff").unwrap()),
                               );
                           });
                       });

                   });

                ui.separator();

                ui.vertical_centered(|ui| {
                    ui.hyperlink_to("Website", "https://nicolo.dev").on_hover_text("https://nicolo.dev");
                    ui.separator();
                    ui.hyperlink_to("Github", "https://github.com/seekbytes/").on_hover_text("https://github.com/seekbytes/");
                    ui.separator();
                    ui.hyperlink_to("Email", "mailto:seekbytes@protonmail.com").on_hover_text("seekbytes@protonmail.com");
                });

            });
    }
}
