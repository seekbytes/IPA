use crate::core::pages::{ObjectVertex, PagesList};
use anyhow::Result;

/// Implements the analysis of the references
pub struct ObjectGraphAnalysis {}

impl ObjectGraphAnalysis {
    /// Analyze a number of pages
    pub fn analyze(pages: &PagesList) -> Result<ObjectGraph> {
        for p in &pages.extended_pages {
            let _primitive = &p.page_rc.parent;
        }
        Ok(ObjectGraph::new(vec![], vec![]))
    }
}

/// The object graph
pub struct ObjectGraph {
    /// the various indexes of the object vertex
    pub vertices: Vec<ObjectVertex>,
    /// the various edges
    pub edges: Vec<ObjectEdge>,
}

impl ObjectGraph {
    /// Creates a new instance of object graph
    pub fn new(vertices: Vec<ObjectVertex>, edges: Vec<ObjectEdge>) -> Self {
        Self { vertices, edges }
    }
}

/// An edge that represents a connection across two objects
pub struct ObjectEdge {
    /// source object
    pub src: ObjectVertex,
    /// destination object
    pub dst: ObjectVertex,
}
