use crate::gui::views::object_view::ObjectViewer;
use egui::Ui;
use pdf::object::InfoDict;
use pdf::primitive::Date;

/// Display the summary
pub struct SummaryView {}

impl SummaryView {
    /// Display the summary
    pub fn show(ui: &mut Ui, file_size: u128, file_path: &String, pdf_info: &Option<InfoDict>) {
        ui.heading("Summary");
        ui.label(format!("File path: {}", file_path));
        ui.label(format!("File size: {} (bytes)", file_size));
        ui.label("File format: PDF");
        // display metadata

        if let Some(pdf_info) = &pdf_info {
            if let Some(title) = pdf_info.title.as_ref() {
                ObjectViewer::display_string("Title", title, ui);
            }

            if let Some(author) = pdf_info.author.as_ref() {
                ObjectViewer::display_string("Author", author, ui);
            }

            if let Some(creation_date) = pdf_info.creation_date.as_ref() {
                ui.label(format!(
                    "Creation date: {}",
                    Self::print_date(creation_date)
                ));
            }

            if let Some(creator) = pdf_info.creator.as_ref() {
                ObjectViewer::display_string("Creator", creator, ui);
            }

            if let Some(keywords) = pdf_info.keywords.as_ref() {
                ObjectViewer::display_string("Keywords", keywords, ui);
            }

            if let Some(mod_date) = pdf_info.mod_date.as_ref() {
                ui.label(format!("Modified data: {}", Self::print_date(mod_date)));
            }

            if let Some(producer) = pdf_info.producer.as_ref() {
                ObjectViewer::display_string("Producer", producer, ui);
            }

            if let Some(subject) = pdf_info.subject.as_ref() {
                ObjectViewer::display_string("Subject", subject, ui);
            }

            if let Some(trapped) = pdf_info.trapped.as_ref() {
                ui.label(format!("Trapped: {:?}", trapped));
            }
        }
    }

    /// Print a date
    fn print_date(date: &Date) -> String {
        format!(
            "{}-{:02}-{:02}T{:02}:{:02}:{:02}",
            date.year, date.month, date.day, date.tz_hour, date.tz_minute, date.second
        )
    }
}
