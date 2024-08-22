use crate::core::objects::collector::ObjectsList;
use crate::core::pages::PagesList;
use anyhow::Result;

/// Reports about anything "suspicious" in the PDF or anything weird
pub struct Analyzer {
    /// Result of the heuristics
    pub heuristics: Vec<ElementFound>,
}

impl Analyzer {
    /// Start the analyzer
    pub fn start(object_list: &ObjectsList, page_list: &PagesList) -> Result<Self> {
        let heuristics_object = Self::apply_for_objects(object_list)?;
        let heuristics_pages = Self::apply_for_pages(page_list)?;
        let heuristics = [heuristics_object, heuristics_pages].concat();
        Ok(Self { heuristics })
    }

    /// Apply the heuristics for a list of pages
    fn apply_for_pages(_pages_list: &PagesList) -> Result<Vec<ElementFound>> {
        Ok(vec![])
    }

    /// Apply the heuristics for a list of object
    fn apply_for_objects(objects_list: &ObjectsList) -> Result<Vec<ElementFound>> {
        let mut result = vec![];

        let maximum_len = objects_list.objects_extended.len();

        let obj_max_id = objects_list.objects_extended.get(maximum_len);

        if let Some(final_obj) = obj_max_id {
            if final_obj.plain_ref.id != maximum_len as u64 {
                result.push(ElementFound::new(
                    "Last object is not the one we're expecting. PDF file may be compromised"
                        .to_string(),
                ));
            }
        }

        Ok(result)
    }
}

/// An element found by the analysis
#[derive(Clone)]
pub struct ElementFound {
    /// Explanation given to users
    pub explain: String,
}

impl ElementFound {
    /// Creates a new instance
    pub fn new(explain: String) -> Self {
        Self { explain }
    }
}
