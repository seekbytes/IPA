use crate::core::context::CoreApp;
use crate::core::recovery_mode::RecoveryMode;
use crate::gui::app::AppView::ObjectView;
use crate::gui::views::heuristics_view::HeuristicView;
use crate::gui::views::object_view::ObjectViewer;
use crate::gui::views::page_view::PageView;
use crate::gui::views::summary_view::SummaryView;
use crate::gui::views::welcome_view::WelcomeView;
use crate::gui::windows::about::AboutWindow;
use crate::gui::windows::how_it_works_view::HowItWorksWindow;
use anyhow::Error;
use eframe::{App, Frame};
use egui::{CollapsingHeader, Context, Sense, Visuals};
use egui_extras::{Column, TableBuilder};
use std::default::Default;

/// The application itself: contains some details used by GUI and Core that keeps the internal data
/// about the pdf. Usually all the settings, state, and variables that need to be memorized are
/// here.
#[derive(Default)]
pub struct AnalyzerApp {
    /// contains the details about pdf
    core: Option<CoreApp>,
    /// contains the picked file (None at opening of the file)
    picked_path: Option<String>,
    /// contains possible dropped file from the GUI
    dropped_files: Vec<egui::DroppedFile>,
    /// error that need to be displayed to GUI ("we encountered a problem while parsing pdf: {err}")
    error: Option<Error>,
    /// true when the about window is opened (state managed by egui)
    about_window: bool,
    /// true when the help window is oepend (state managed by egui)
    help_window: bool,
    /// true when the lighter analysis is used
    lighter_analysis: bool,
    /// recovery mode
    recovery: Option<RecoveryMode>,
}

/// A structure that sums up all the possible views
#[derive(Default, Debug)]
pub enum AppView {
    #[default]
    /// The opening view
    Welcome,
    /// Explains some patterns that might be used to embed the malware for pdf
    Heuristics,
    /// A sum up of elements that is the first view once a pdf file is parsed correctly
    Summary,
    /// The view to show the object
    ObjectView,
    /// The view to show the page
    PageView,
    /// The view to show details of the trailer (if present)
    Trailer,
}

impl AnalyzerApp {
    /// Creates a new instance of AnalyzerApp
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        egui_extras::install_image_loaders(&cc.egui_ctx);

        AnalyzerApp::default()
    }

    /// Display in case of any errors or first time
    pub fn display_without(ctx: &Context, option: &Option<Error>, needs_recovery_mode: &mut bool) {
        WelcomeView::show(ctx, option, needs_recovery_mode)
    }

    /// Display with core
    pub fn display_with_core(context: &Context, core: &mut CoreApp) {
        let objects = &core.objects.objects.clone();
        let pages = &core.pages.pages.clone();
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .show(context, |ui| {
                ui.heading("IPA");
                ui.separator();
                let (mut current_id, mut is_page, mut is_object) = (None, false, false);
                let mut table = TableBuilder::new(ui).column(Column::remainder());

                table = table.sense(Sense::click());
                let labels_to_create = ["Summary", "Heuristics", "Trailer"];

                table.body(|mut body| {
                    for label in labels_to_create {
                        body.row(16.0, |mut row| {
                            row.col(|ui| {
                                ui.label(label);
                            });
                            core.change_view(row.index(), row.response());
                        });
                    }

                    body.row(16.0, |mut row| {
                        row.col(|ui| {
                            CollapsingHeader::new("Objects")
                                .default_open(false)
                                .show(ui, |ui| {
                                    let mut table2 =
                                        TableBuilder::new(ui).column(Column::remainder());
                                    table2 = table2.sense(Sense::click());

                                    table2.body(|body_inner_table| {
                                        body_inner_table.rows(16.0, objects.len(), |mut row| {
                                            let object = objects.get(row.index()).unwrap();
                                            row.col(|ui| {
                                                ui.label(format!(
                                                    "[id: {}] {}",
                                                    object.id, object.name
                                                ));
                                            });
                                            if row.response().clicked() {
                                                current_id = Some(row.index());
                                                is_object = true;
                                            }
                                        });
                                    })
                                });
                        });
                    });

                    body.row(16.0, |mut row| {
                        row.col(|ui| {
                            CollapsingHeader::new("Pages")
                                .default_open(false)
                                .show(ui, |ui| {
                                    let mut table3 =
                                        TableBuilder::new(ui).column(Column::remainder());
                                    table3 = table3.sense(Sense::click());

                                    table3.body(|body_inner_table| {
                                        body_inner_table.rows(16.0, pages.len(), |mut row| {
                                            let object = pages.get(row.index()).unwrap();
                                            row.col(|ui| {
                                                ui.label(object.name.to_string());
                                            });
                                            if row.response().clicked() {
                                                current_id = Some(row.index());
                                                is_page = true;
                                            }
                                        });
                                    })
                                });
                        });
                    });

                    if let Some(curr) = current_id {
                        if is_page {
                            core.switch_page(curr as u64);
                            is_page = false;
                        }

                        if is_object {
                            core.switch_object(curr as u64);
                            is_object = false;
                        }
                    }
                });
            });

        egui::CentralPanel::default().show(context, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                match core.gui.current_view {
                    AppView::Welcome => {}
                    AppView::Heuristics => {
                        HeuristicView::show(ui, &core.heuristics);
                    }
                    AppView::Summary => SummaryView::show(
                        ui,
                        core.input_file.buffer.len() as u128,
                        &core.input_file.path,
                        &core.pdf_file.trailer.info_dict,
                    ),
                    AppView::ObjectView => {
                        let mut nr_object = None;

                        let o = core
                            .objects
                            .objects_extended
                            .get(core.gui.current_id as usize);

                        let mut subview = None;

                        if let Some(object) = o {
                            let s = object.object_content.get_debug_name();
                            match s {
                                // semantics for any kind of object
                                "Stream" => ObjectViewer::display_stream(
                                    object,
                                    ui,
                                    &mut nr_object,
                                    &mut subview,
                                    &core.gui.subcurrent_obj_view,
                                ),
                                "Dictionary" => {
                                    ObjectViewer::display_dictionary(object, ui, &mut nr_object)
                                }
                                "Array" => {
                                    ui.heading(format!("Array (ID: {})", &object.id));
                                    ObjectViewer::display_object(
                                        &object.object_content,
                                        ui,
                                        &mut nr_object,
                                        0,
                                    )
                                }
                                "String" => {
                                    ui.heading(format!("String (ID: {})", &object.id));
                                    ObjectViewer::display_object(
                                        &object.object_content,
                                        ui,
                                        &mut nr_object,
                                        0,
                                    )
                                }
                                "Integer" => {
                                    ui.heading(format!("Integer (ID: {})", &object.id));
                                    ObjectViewer::display_integer(object, ui)
                                }
                                _ => {
                                    unimplemented!()
                                }
                            }

                            ui.separator();

                            let to_references = core.object_linker.to_objects.get(&object.id);
                            let from_references = core.object_linker.from_objects.get(&object.id);

                            ui.horizontal(|ui| {
                                if let Some(to_references) = to_references {
                                    ui.vertical(|ui| {
                                        if !to_references.is_empty() {
                                            ui.label(format!("To: {:?}", to_references));
                                        }
                                    });
                                }
                                if let Some(from_references) = from_references {
                                    ui.separator();
                                    ui.vertical(|ui| {
                                        if !from_references.is_empty() {
                                            ui.label(format!("From: {:?}", from_references));
                                        }
                                    });
                                }
                            });

                            // change the subview for an object
                            if let Some(sub) = subview {
                                core.gui.subcurrent_obj_view = sub;
                            }

                            // if anyone clicked on 'Go to object', then change the object
                            if let Some(nr_object2) = nr_object {
                                let index = core.objects.map.get(&nr_object2);
                                // safe
                                core.switch_object(*index.unwrap());
                            }
                        } else {
                            ui.label(format!(
                                "Impossible to find object at index {}",
                                core.gui.current_id
                            ));
                        }
                    }
                    AppView::Trailer => {
                        ui.heading("Trailer");
                        let trailer = &core.pdf_file.trailer;
                        ui.label(format!("{}", trailer.size));
                        ui.label(format!("{:?}", trailer.info_dict));
                        ui.label(format!("{:?}", trailer.prev_trailer_pos));
                        ui.label(format!("{:?}", trailer.root));
                    }
                    AppView::PageView => {
                        let p = core.pages.extended_pages.get(core.gui.current_id as usize);
                        let mut subview = None;

                        let mut nr_object = None;

                        if let Some(page) = p {
                            PageView::show(
                                page,
                                ui,
                                &mut subview,
                                &core.gui.subcurrent_page_view,
                                &mut nr_object,
                            );
                        }

                        if let Some(some) = nr_object {
                            let index = core.objects.map.get(&some);
                            if let Some(obj_to_change) = index {
                                core.gui.current_id = *obj_to_change;
                                core.gui.current_view = ObjectView;
                            }
                        }

                        if let Some(sub) = subview {
                            core.gui.subcurrent_page_view = sub;
                        }
                    }
                }

                //ui.label(format!("Current view (debug): {:?}", core.gui.current_view));
            });
        });
    }
}

impl App for AnalyzerApp {
    /// The main window
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        AboutWindow::show(&mut self.about_window, ctx);
        HowItWorksWindow::show(&mut self.help_window, ctx);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // add menu
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open PDF file...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("filter", ["pdf"].as_ref())
                            .pick_file()
                        {
                            self.picked_path = Some(path.display().to_string());
                            self.core = None;
                        }
                    }

                    if ui.button("Close").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("How IPA works").clicked() {
                        self.help_window = true;
                    }
                    if ui.button("About IPA").clicked() {
                        self.about_window = true;
                    }
                });
            });
        });

        if !self.dropped_files.is_empty() {
            let file = &self.dropped_files[0];
            if let Some(path) = &file.path {
                self.picked_path = Some(path.clone().into_os_string().into_string().unwrap());
            }
        }

        let mut try_recover = false;

        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        });

        // If the file has already been parsed, we display it. Otherwise, we show a welcome page
        match (&mut self.core, &self.recovery) {
            (None, None) => Self::display_without(ctx, &self.error, &mut try_recover),
            (Some(core), None) => Self::display_with_core(ctx, core),
            _ => {}
        }

        // launch recovery mode
        if try_recover {
            self.lighter_analysis = true
        }

        match &mut self.recovery {
            None => {}
            Some(recov) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Recovery mode");
                    ui.label("While parsing the PDF, we might have encountered an issue, and you've requested a lighter analysis. This is the same pass that Didier Steven implements in his tools for the lighter analysis. All credits goes to him.");
                    ui.separator();
                    ui.label(format!("File path: {}", recov.input_file.path));
                    ui.label(format!("File size: {} (bytes)", recov.input_file.buffer.len()));
                    for (key,value) in &recov.frequencies {
                        ui.label(format!("{}: {}", key, value));
                    }
                });
            }
        }

        // launch the core context creation
        if let Some(file) = &self.picked_path {
            if self.core.is_none() && self.error.is_none() {
                let core_app = CoreApp::new(file.clone());
                if let Ok(mut core) = core_app {
                    core.gui.current_view = AppView::Summary;
                    self.core = Some(core);
                    self.error = None;
                } else {
                    self.error = Some(core_app.err().unwrap());
                    self.core = None;
                }
            }
        }

        if self.lighter_analysis {
            if let Some(file) = &self.picked_path {
                let light = RecoveryMode::parse(file.clone());
                if let Ok(res) = light {
                    self.recovery = Some(res);
                    self.lighter_analysis = false;
                    self.error = None;
                    self.core = None;
                }
            }
        }
    }
}
