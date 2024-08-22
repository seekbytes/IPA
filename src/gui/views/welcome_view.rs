use anyhow::Error;
use egui::Context;

/// Display a gentle welcome on opening the application
pub struct WelcomeView {}

impl WelcomeView {
    /// Show the welcome view
    pub fn show(ctx: &Context, option: &Option<Error>, needs_recovery_mode: &mut bool) {
        egui::CentralPanel::default().show(ctx, |ui| {

            if let Some(err_str) = option {
                ui.heading("Error while reading PDF");
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Interactive PDF analysis is a program that is built on ");
                    ui.hyperlink_to("pdf-rs library", "https://github.com/pdf-rs/pdf");
                    ui.label(" which allows to parse any PDF file without too much headache.");
                });
                ui.label("If you have any troubles in reading this pdf, you may want to open them an issue, and reporting the error to the repository authors.");

                let body = format!("I'm creating this issue because it seems pdf-rs failed to parse a pdf file.\n Some additional info: ```\n{}\n```. **ATTACH THE PDF FILE FOR HIGHER CHANCES of fixing** The description has been generated automatically by [IPA](https://github.com/seekbytes/IPA).", err_str);

                ui.label("You may want to create a new issue with the following link, but be warned that may contain some details about your setup may be posted online.");
                ui.hyperlink_to("Open a new issue", format!("https://github.com/pdf-rs/pdf/issues/new?title=PDF+parsing+failed&body={}", body));

                ui.text_edit_multiline(&mut format!("{}", err_str));
                ui.separator();
                ui.label("You can try reading another pdf file or try another analysis.");

                if ui.button("Don't parse, use a lighter analysis").clicked() {
                    *needs_recovery_mode = true;
                }

            } else {
                ui.heading("Welcome to Interactive PDF Analysis");
                ui.label("Interactive PDF Analysis (also called IPA) allows any researcher to explore the inner details of any PDF file. PDF files may be used to carry malicious payloads that exploit vulnerabilities, and issues of PDF viewer, or may be used in phishing campaigns as social engineering artefacts.");
                ui.label("The goal of this software is to let any analyst to go deep on its own the PDF file. Via IPA, you may extract important payload from PDF files, understand the relationship across objects, and infer elements that may be helpful for triage.");
                ui.label("The main inspiration goes to the fantastic people behind Zynamics, and their excellent product, called PDF dissector.\n");
                ui.hyperlink_to("Author: SeekBytes", "https://github.com/seekbytes/");
            }

            ui.centered_and_justified(|ui| {
                ui.label("Start by dropping a file or open a new one");
            });


        });
    }
}
