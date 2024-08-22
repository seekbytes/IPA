use crate::core::heuristics::analyzer::Analyzer;
use egui::Ui;

/// Show the result of the heuristics applied
pub struct HeuristicView {}

impl HeuristicView {
    /// Show the result of the heuristics
    pub fn show(ui: &mut Ui, results: &Analyzer) {
        ui.heading("Heuristics");

        if results.heuristics.is_empty() {
            ui.label("No heuristics have been applied.".to_string());

            ui.label("Have you any idea about rules that might be applied to identify potential issues for a PDF?");
            ui.hyperlink_to("Create a new issue", "https://github.com/seekbytes/IPA");
        } else {
            for result in &results.heuristics {
                ui.label(result.explain.to_string());
            }
        }
    }
}
