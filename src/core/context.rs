use crate::core::heuristics::analyzer::Analyzer;
use crate::core::input_file::InputFile;
use crate::core::objects::collector::ObjectsList;
use crate::core::objects::graph::{ObjectGraph, ObjectGraphAnalysis};
use crate::core::objects::object_linker::ObjectLinker;
use crate::core::objects::stream_parser::StreamParser;
use crate::core::pages::PagesList;
use crate::core::recovery_mode::RecoveryMode;
use crate::gui::app::AppView;
use crate::gui::views::object_view::ObjectSubView;
use crate::gui::views::page_view::PageSubView;
use anyhow::Result;
use egui::Response;
use pdf::file::{File, FileOptions, NoLog, ObjectCache, ScanItem, StreamCache};
use pdf::object::ParseOptions;

/// Represents an instance of a file correctly parsed
pub struct CoreApp {
    /// information about the input file
    pub input_file: InputFile,
    /// the structure retrieved from the library once a pdf is correctly parsed
    pub pdf_file: File<Vec<u8>, ObjectCache, StreamCache, NoLog>,
    /// the list of the objects retrieved
    pub objects: ObjectsList,
    /// the list of the pages retrieved
    pub pages: PagesList,
    /// list of the links between objects
    pub object_linker: ObjectLinker,
    /// heuristics result
    pub heuristics: Analyzer,
    /// object graph
    pub object_graph: ObjectGraph,
    /// some details used by gui
    pub gui: CoreGui,
}

/// Contains some elements used by egui
#[derive(Default)]
pub struct CoreGui {
    /// current_id specifies which object or page we want to see
    pub current_id: u64,
    /// the current view of the UI
    pub current_view: AppView,
    /// the current subview for the Object
    pub subcurrent_obj_view: ObjectSubView,
    /// the current subview for the Page
    pub subcurrent_page_view: PageSubView,
}

impl CoreApp {
    /// Creates a new instance of CoreApp (implements the backend and the inner logic)
    pub fn new(path: String) -> Result<Self> {
        RecoveryMode::parse(path.clone())?;

        let input_file = InputFile::new(path)?;
        let pdf_file = FileOptions::cached()
            .parse_options(ParseOptions::tolerant())
            .open(&input_file.path)?;

        let pages_to_compose = pdf_file.pages().flatten().collect();

        let objects_to_compose = pdf_file
            .scan()
            .flatten()
            .filter(|o| matches!(o, ScanItem::Object(_, _)))
            .collect::<Vec<ScanItem>>();

        let mut objects = ObjectsList::new(objects_to_compose)?;

        let objects_extended =
            StreamParser::parse(objects.objects_extended.clone(), &pdf_file.resolver())?;
        objects.objects_extended = objects_extended;

        let pages = PagesList::new(pages_to_compose, &pdf_file.resolver())?;

        let object_linker = ObjectLinker::connect(&objects.objects_extended)?;

        let object_graph = ObjectGraphAnalysis::analyze(&pages)?;

        let heuristics = Analyzer::start(&objects, &pages)?;

        let gui = CoreGui::new(0, Default::default(), ObjectSubView::Decoded);

        Ok(Self {
            input_file,
            pdf_file,
            objects,
            pages,
            object_linker,
            heuristics,
            object_graph,
            gui,
        })
    }

    /// Switch the id and force the object view
    pub fn switch_object(&mut self, object_id: u64) {
        if self.objects.objects.get(object_id as usize).is_some() {
            self.gui.current_id = object_id;
            self.gui.current_view = AppView::ObjectView;
        }
    }

    /// Switch the id and force the object page
    pub fn switch_page(&mut self, page_id: u64) {
        self.gui.current_id = page_id;
        self.gui.current_view = AppView::PageView;
    }

    /// Changes the view
    pub fn change_view(&mut self, view_id: usize, response: Response) {
        if response.clicked() {
            let view = match view_id {
                0 => AppView::Summary,
                1 => AppView::Heuristics,
                2 => AppView::Trailer,
                3 => AppView::ObjectView,
                _ => AppView::Welcome,
            };
            self.gui.current_view = view;
        }
    }

    /// Switch the id for the view (see change_view to understand what kind of ID is mapped to a
    /// view)
    pub fn switch_id(&mut self, view_id: usize) {
        self.gui.current_id = view_id as u64;
    }
}

impl CoreGui {
    /// Creates a new instance for CoreGui
    pub fn new(current_id: u64, current_view: AppView, subcurrent_view: ObjectSubView) -> Self {
        Self {
            current_id,
            current_view,
            subcurrent_obj_view: subcurrent_view,
            subcurrent_page_view: Default::default(),
        }
    }
}
