use crate::core::pages::SinglePagesExtended;
use crate::gui::views::object_view::ObjectViewer;
use egui::Ui;
use egui_extras::{Column, TableBuilder};
use pdf::object::PlainRef;
use pdf::primitive::Primitive;
use std::fmt::{Display, Formatter};

/// View a single page
pub struct PageView {}

/// The subview for any page
#[derive(Debug, PartialOrd, PartialEq, Default)]
pub enum PageSubView {
    /// Decoded
    #[default]
    Decoded,
    /// Text extracted
    TextExtracted,
    /// Operations
    Operations,
}

impl Display for PageSubView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PageSubView::Decoded => write!(f, "Decoded"),
            PageSubView::TextExtracted => write!(f, "Text extracted"),
            PageSubView::Operations => write!(f, "Operations"),
        }
    }
}

impl PageView {
    /// show the page view
    pub fn show(
        page: &SinglePagesExtended,
        ui: &mut Ui,
        selected: &mut Option<PageSubView>,
        actual_sub_view: &PageSubView,
        nr_object: &mut Option<u64>,
    ) {
        ui.horizontal(|ui| {
            ui.heading(format!("Page (ID: {})", page.id));
            egui::ComboBox::new("display_stream", "")
                .selected_text(format!("{}", actual_sub_view))
                .show_ui(ui, |ui| {
                    ui.selectable_value(selected, Some(PageSubView::Decoded), "Decoded");
                    ui.selectable_value(
                        selected,
                        Some(PageSubView::TextExtracted),
                        "Text extracted",
                    );
                    ui.selectable_value(selected, Some(PageSubView::Operations), "Operations");
                });
        });

        match actual_sub_view {
            PageSubView::Decoded => Self::show_decoded(ui, page, nr_object),
            PageSubView::TextExtracted => Self::show_text_extracted(ui, page),
            PageSubView::Operations => Self::show_operations(ui, page),
        }
    }

    /// Show the text extracted from the pdf page
    fn show_text_extracted(ui: &mut Ui, page: &SinglePagesExtended) {
        ui.label(page.text.to_string());
    }

    /// Show operations
    fn show_operations(ui: &mut Ui, page: &SinglePagesExtended) {
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto().at_least(250.0))
            .column(Column::auto())
            .column(Column::remainder())
            .header(15.0, |mut header| {
                header.col(|ui| {
                    ui.label("Operation");
                });
                header.col(|ui| {
                    ui.label("Operation type");
                });
                header.col(|ui| {
                    ui.label("Preview");
                });
            })
            .body(|body| {
                body.rows(16.0, page.operations.len(), |mut row| {
                    let single_op = page.operations.get(row.index()).unwrap();
                    row.col(|ui| {
                        // print debug information
                        ui.label(format!("{:?}", single_op.operation));
                    });
                    row.col(|ui| {
                        // print the name
                        ui.label(&single_op.name);
                    });
                    row.col(|_ui| {
                        // preview of feature
                    });
                });
            });
    }

    /// Show the decoded content of a pdf page
    fn show_decoded(ui: &mut Ui, page: &SinglePagesExtended, nr_object: &mut Option<u64>) {
        let page_rc = &page.page_rc;
        if let Some(obj_primitive) = &page_rc.metadata {
            let mut nr_object = Some(0);
            ObjectViewer::display_object(obj_primitive, ui, &mut nr_object, 0);
        }

        if let Some(vp) = &page_rc.vp {
            let mut nr_object = Some(0);
            ObjectViewer::display_object(vp, ui, &mut nr_object, 0);
        }

        if let Some(lgi_primitive) = &page_rc.lgi {
            let mut nr_object = Some(0);
            ObjectViewer::display_object(lgi_primitive, ui, &mut nr_object, 0);
        }

        if let Some(contnent) = &page_rc.contents {
            ui.label(format!("{:?}", contnent));
        }

        let mut nr_objects = None;
        ObjectViewer::display_object(
            &Primitive::Dictionary(page_rc.other.clone()),
            ui,
            &mut nr_objects,
            0,
        );

        ui.label("X objects");
        if let Ok(res) = &page_rc.resources() {
            for (name_, x_object) in &res.xobjects {
                let PlainRef { id, .. } = x_object.get_inner();
                {
                    ui.horizontal(|ui| {
                        ui.label(format!("XObject: {}: Obj number @{}", name_, id));
                        if ui.button("Go to object").clicked() {
                            nr_object.take();
                            *nr_object = Some(id);
                        }
                    });
                }
            }
        }
    }
}
