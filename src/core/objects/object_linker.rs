use crate::core::objects::collector::{ObjectID, SingleObjectExtended};
use anyhow::Result;
use pdf::primitive::Primitive;
use std::collections::HashMap;

/// From a list of objects, this struct remembers which objects are references to/from. Helpful to
/// identify any orphan or indirect objects, or to identify the roots of the PDF file.
pub struct ObjectLinker {
    /// references to an object
    pub to_objects: HashMap<ObjectID, Vec<ObjectID>>,
    /// references from an object
    pub from_objects: HashMap<ObjectID, Vec<ObjectID>>,
}

impl ObjectLinker {
    /// Traverse the tree of primitive objects
    pub fn scan(primitive: &Primitive, references_to: &mut Vec<ObjectID>) {
        match primitive {
            Primitive::Stream(ess) => {
                for (_, element) in &ess.info {
                    Self::scan(element, references_to)
                }
            }
            Primitive::Dictionary(elements) => {
                for (_, element) in elements {
                    Self::scan(element, references_to)
                }
            }
            Primitive::Array(elements) => {
                for element in elements {
                    Self::scan(element, references_to)
                }
            }
            Primitive::Reference(reference) => {
                let object_id_to_push = ObjectID::from(reference.id);
                if !references_to.contains(&object_id_to_push){
                    references_to.push(object_id_to_push)
                }
            },
            _ => {}
        }
    }

    /// Connect objects
    pub fn connect(objects: &Vec<SingleObjectExtended>) -> Result<Self> {
        let mut to_hashmap: HashMap<ObjectID, Vec<ObjectID>> = HashMap::new();
        let mut from_hashmap: HashMap<ObjectID, Vec<ObjectID>> = HashMap::new();

        // a) build the to hashmap: given an object A, which objects does A mention?
        for obj in objects {
            let primitive = &obj.object_content;
            let mut from = vec![];
            Self::scan(primitive, &mut from);
            to_hashmap.insert(obj.id, from);
        }

        // b) build the from hashmap: given an object A, which objects mentions A?
        for keys in to_hashmap.keys() {
            let mut parents_for_key = vec![];
            for (actual_key, values) in &to_hashmap {
                if values.contains(keys) {
                    if !parents_for_key.contains(actual_key){
                        parents_for_key.push(*actual_key);
                    }
                }
            }

            from_hashmap.insert(*keys, parents_for_key);
        }

        Ok(Self {
            to_objects: to_hashmap,
            from_objects: from_hashmap,
        })
    }
}
