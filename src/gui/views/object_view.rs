use crate::core::objects::collector::{FileContent, SingleObjectExtended};
use egui::Ui;
use egui_extras::{Column, TableBuilder};
use pdf::primitive::{PdfString, Primitive};
use std::fs;

/// Helper to visualize an object
pub struct ObjectViewer {}

/// What is the subview we should see
#[derive(Debug, PartialEq, Default)]
pub enum ObjectSubView {
    /// The structure itself
    #[default]
    Decoded,
    /// The hex content for the structure compressed
    HexCompressed,
    /// The hex content for the structure uncompressed
    HexUncompressed,
    /// The content
    Content,
}

impl ObjectViewer {
    /// Display the stream
    pub fn display_stream(
        object: &SingleObjectExtended,
        ui: &mut Ui,
        nr_object: &mut Option<u64>,
        selected: &mut Option<ObjectSubView>,
        actual_sub_view: &ObjectSubView,
    ) {
        let primitive = &object.object_content;
        if let Primitive::Stream(pdfstream) = primitive {
            let dictionary = &pdfstream.info;

            ui.horizontal(|ui| {
                ui.heading(format!("Stream (ID: {})", object.id));
                ui.button("❓").on_hover_text("A stream is a particular object that helps embedding any file into a PDF file.");
                egui::ComboBox::new("display_stream", "")
                    .selected_text(format!("{:?}", actual_sub_view))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(selected, Some(ObjectSubView::Decoded), "Decoded");
                        ui.selectable_value(selected, Some(ObjectSubView::Content), "Content");
                        ui.selectable_value(selected, Some(ObjectSubView::HexCompressed), "Hex (compressed)");
                        //ui.selectable_value(selected, Some(ObjectSubView::HexUncompressed), "Hex (uncompressed)"); (not implemented)
                    });
                ui.button("❓").on_hover_text("Decoded: the object parsed by IPA.\n Content is the content of the stream parsed.\n Hex presents the raw data for the stream.");
            });

            match actual_sub_view {
                ObjectSubView::Decoded => {
                    TableBuilder::new(ui)
                        .striped(true)
                        .column(Column::auto().at_least(100f32))
                        .column(Column::remainder())
                        .header(25.0, |mut row| {
                            row.col(|ui| {
                                ui.label("Key");
                            });
                            row.col(|ui| {
                                ui.label("Value");
                            });
                        })
                        .body(|mut body| {
                            for k in dictionary {
                                body.row(15.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(format!("{}", k.0));
                                    });
                                    row.col(|ui| {
                                        ObjectViewer::display_object(k.1, ui, nr_object, 0);
                                    });
                                });
                            }
                        });
                }
                ObjectSubView::HexCompressed => {
                    if let Some(buff) = &object.raw_buffer {
                        ui.label("Raw content of stream.".to_string());
                        if ui.button("Save the raw content of stream").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_file_name(format!("raw_stream_{}", object.id))
                                .save_file()
                            {
                                fs::write(path, buff).unwrap();
                            }
                        };

                        ui.label(format!("{:?}", buff));
                    } else {
                        ui.label("No content for this stream");
                    }
                }
                ObjectSubView::Content => match &object.file_content {
                    None => {
                        ui.label("No content for this stream.");
                    }
                    Some(file) => match file {
                        FileContent::Unknown(_) => {
                            ui.label(format!("{:?}", object.file_content));
                        }
                        FileContent::GraphicState(_) => {}
                        FileContent::TextASCII(_, stra) => {
                            ui.label(format!("String {}", stra));
                        }
                    },
                },
                _ => {}
            }
        }
    }

    /// Display a generic primitive
    pub fn display_object(
        primitive: &Primitive,
        ui: &mut Ui,
        nr_object: &mut Option<u64>,
        level: u64,
    ) {
        #[allow(unused_assignments)]
        match primitive {
            Primitive::Null => {
                ui.label("NULL");
            }
            Primitive::Integer(integer) => {
                ui.label(format!("{:x}", integer));
            }
            Primitive::Number(num) => {
                ui.label(format!("{}", *num));
            }
            Primitive::Boolean(boo) => {
                ui.label(format!("{}", *boo));
            }
            Primitive::String(pdf_string) => {
                Self::display_string("", pdf_string, ui);
            }
            Primitive::Stream(stream) => {
                ui.label(format!("{:x?}", stream));
            }
            Primitive::Dictionary(dict) => {
                if !dict.is_empty() {
                    ui.label("Inner Dictionary preview");

                    ui.push_id(level, |ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .column(Column::auto().at_least(100f32))
                            .column(Column::remainder())
                            .header(25.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Key");
                                });
                                row.col(|ui| {
                                    ui.label("Value");
                                });
                            })
                            .body(|mut body| {
                                for k in dict {
                                    body.row(15.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(format!("{}", k.0));
                                        });
                                        row.col(|ui| {
                                            ObjectViewer::display_object(
                                                k.1,
                                                ui,
                                                nr_object,
                                                level + 1,
                                            );
                                        });
                                    });
                                }
                            });
                    });
                } else {
                    ui.label("[blank]");
                }
            }
            Primitive::Array(arr) => {
                let mut level_mod = level;
                for p in arr {
                    level_mod += 1;
                    Self::display_object(p, ui, nr_object, level_mod)
                }
            }
            Primitive::Reference(reference) => {
                ui.horizontal(|ui| {
                    ui.label(format!("@{}", reference.id));
                    if ui.button("Go to object").clicked() {
                        nr_object.take();
                        *nr_object = Some(reference.id);
                    }
                });
            }
            Primitive::Name(name) => {
                ui.label(format!("{}", name));
            }
        }
    }

    /// Display a single string
    pub fn display_string(field: &str, pdf_string: &PdfString, ui: &mut Ui) {
        let buffer = pdf_string.data.as_slice();

        let field_to_insert = match field {
            "" => "".to_string(),
            _ => field.to_string() + ": ",
        };

        if buffer.len() > 2 {
            // see if it's UTF-16
            if buffer[0] == 0xfe && buffer[1] == 0xff {
                // skip the first two bytes because they are marker of an utf-16 string
                let iter = (1..(buffer.len() / 2))
                    .map(|i| u16::from_be_bytes([buffer[2 * i], buffer[2 * i + 1]]))
                    .collect::<Vec<u16>>();

                let t = String::from_utf16(&iter).unwrap();
                ui.label(format!("{}{:?}", field_to_insert, t));
            } else {
                ui.label(format!("{}{:?}", field_to_insert, pdf_string));
            }
        } else {
            ui.label(format!("{}{:?}", field_to_insert, pdf_string));
        }
    }

    /// Display dictionary
    pub fn display_dictionary(
        object: &SingleObjectExtended,
        ui: &mut Ui,
        nr_object: &mut Option<u64>,
    ) {
        let primitive = &object.object_content;

        if let Primitive::Dictionary(dictionary) = primitive {
            ui.heading(format!("Dictionary (ID: {})", object.id));
            TableBuilder::new(ui)
                .striped(true)
                .column(Column::auto().at_least(100f32))
                .column(Column::remainder())
                .header(25.0, |mut row| {
                    row.col(|ui| {
                        ui.label("Key");
                    });
                    row.col(|ui| {
                        ui.label("Value");
                    });
                })
                .body(|mut body| {
                    for (name, primitive) in dictionary {
                        body.row(15.0, |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}", name));
                            });
                            row.col(|ui| {
                                ObjectViewer::display_object(primitive, ui, nr_object, 0);
                            });
                        });
                    }
                });
        }
    }

    /// Display an integer value
    pub fn display_integer(object: &SingleObjectExtended, ui: &mut Ui) {
        let primitive = &object.object_content;

        if let Primitive::Integer(int) = primitive {
            ui.label(format!("Value: {}", int));
        }
    }
}
