use anyhow::Result;
use pdf::content::{Op, TextDrawAdjusted};
use pdf::object::{PageRc, Resolve};

/// Used in order to keep a list of pages names and one list of the concrete objects
/// retrieved with pdf-rs
#[derive(Clone)]
pub struct PagesList {
    /// the sum up for the pages of a PDF
    pub pages: Vec<SinglePages>,
    /// the pages of a PDF
    pub extended_pages: Vec<SinglePagesExtended>,
}

impl PagesList {
    /// Creates a new instance
    pub fn new(pages_scan: Vec<PageRc>, resolve: &impl Resolve) -> Result<Self> {
        let mut extended_pages = vec![];
        let mut pages = vec![];
        for (i, p) in pages_scan.iter().enumerate() {
            let b = SinglePages::new(i as u64, format!("Page {}", i));
            let a = SinglePagesExtended::new(i as u64, p.clone(), resolve);
            pages.push(b);
            extended_pages.push(a);
        }
        Ok(Self {
            pages,
            extended_pages,
        })
    }
}

/// Keeps in memory as little as possible details about a single page
#[derive(Clone)]
pub struct SinglePages {
    /// number of the page
    pub id: u64,
    /// name of the page
    pub name: String,
}

impl SinglePages {
    /// Creates a new instance
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

/// Keeps in memory all the details about a single page
#[derive(Debug, Clone)]
pub struct SinglePagesExtended {
    /// The page number
    pub id: u64,
    /// The page retrieved from the pdf
    pub page_rc: PageRc,
    /// object graph
    pub objects: Vec<u64>,
    /// operations
    /// TODO: wrap the op in a new
    pub operations: Vec<SingleOp>,
    /// text extracted
    pub text: String,
}

/// A single operation, made for making life easier to egui
#[derive(Clone, Debug)]
pub struct SingleOp {
    /// name of the type
    pub name: String,
    /// the operation itself
    pub operation: Op,
}

impl SingleOp {
    /// From operation get the name of the enum
    pub fn get_name(operation: &Op) -> String {
        let typed = match operation {
            Op::BeginMarkedContent { .. } => "BeginMarkedContent",
            Op::EndMarkedContent => "EndMarkedContent",
            Op::MarkedContentPoint { .. } => "MarkedContentPoint",
            Op::Close => "Close",
            Op::MoveTo { .. } => "MoveTo",
            Op::LineTo { .. } => "LineTo",
            Op::CurveTo { .. } => "CurveTo",
            Op::Rect { .. } => "Rect",
            Op::EndPath => "EndPath",
            Op::Stroke => "Stroke",
            Op::FillAndStroke { .. } => "FillAndStroke",
            Op::Fill { .. } => "Fill",
            Op::Shade { .. } => "Shade",
            Op::Clip { .. } => "Clip",
            Op::Save => "Save",
            Op::Restore => "Restore",
            Op::Transform { .. } => "Transform",
            Op::LineWidth { .. } => "LineWidth",
            Op::Dash { .. } => "Dash",
            Op::LineJoin { .. } => "LineJoin",
            Op::LineCap { .. } => "LineCap",
            Op::MiterLimit { .. } => "MiterLimit",
            Op::Flatness { .. } => "Flatness",
            Op::GraphicsState { .. } => "GraphicsState",
            Op::StrokeColor { .. } => "StrokeColor",
            Op::FillColor { .. } => "FillColor",
            Op::FillColorSpace { .. } => "FillColorSpace",
            Op::StrokeColorSpace { .. } => "StrokeColorSpace",
            Op::RenderingIntent { .. } => "RenderingIntent",
            Op::BeginText => "BeginText",
            Op::EndText => "EndText",
            Op::CharSpacing { .. } => "CharSpacing",
            Op::WordSpacing { .. } => "WordSpacing",
            Op::TextScaling { .. } => "TextScaling",
            Op::Leading { .. } => "Leading",
            Op::TextFont { .. } => "TextFont",
            Op::TextRenderMode { .. } => "TextRenderMode",
            Op::TextRise { .. } => "TextRise",
            Op::MoveTextPosition { .. } => "MoveTextPosition",
            Op::SetTextMatrix { .. } => "SetTextMatrix",
            Op::TextNewline => "TextNewline",
            Op::TextDraw { .. } => "TextDraw",
            Op::TextDrawAdjusted { .. } => "TextDrawAdjusted",
            Op::XObject { .. } => "XObject",
            Op::InlineImage { .. } => "InlineImage",
        };

        typed.to_string()
    }
}

impl SinglePagesExtended {
    /// Creates a new instance
    pub fn new(id: u64, page_rc: PageRc, resolver: &impl Resolve) -> Self {
        if let Some(content) = &page_rc.contents {
            if let Ok(op) = content.operations(resolver) {
                let text = Self::extract_text(&op);

                let operations = op
                    .into_iter()
                    .map(|o| SingleOp {
                        name: SingleOp::get_name(&o).to_string(),
                        operation: o,
                    })
                    .collect();

                return Self {
                    id,
                    page_rc,
                    objects: vec![],
                    operations,
                    text,
                };
            }
        }

        Self {
            id,
            page_rc,
            objects: vec![],
            operations: vec![],
            text: "".to_string(),
        }
    }

    /// Extract text
    fn extract_text(op: &Vec<Op>) -> String {
        let mut result = "".to_string();
        for o in op {
            match o {
                Op::TextRenderMode { .. } => {}
                Op::TextRise { .. } => {}
                Op::MoveTextPosition { .. } => {}
                Op::SetTextMatrix { .. } => {}
                Op::TextNewline => result = format!("{}\n", result).to_string(),
                Op::TextDraw { text } => {
                    if let Ok(text_decoded) = text.to_string() {
                        result = format!("{}{}", result, text_decoded).to_string()
                    } else {
                        result = format!("{}{:?}", result, text).to_string()
                    }
                }
                Op::TextDrawAdjusted { array } => {
                    for text_draw in array {
                        match text_draw {
                            TextDrawAdjusted::Text(text) => {
                                if let Ok(text_decoded) = text.to_string() {
                                    result = format!("{} {}", result, text_decoded).to_string()
                                } else {
                                    result = format!("{} {:?}", result, text).to_string()
                                }
                            }
                            TextDrawAdjusted::Spacing(space) => {
                                if *space > 0f32 {
                                    result = format!("{}  ", result).to_string()
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        result
    }
}

/// The key used to make an index for the node (aka their id)
pub type ObjectVertex = u64;
