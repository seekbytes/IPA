use anyhow::Result;
use egui::ahash::HashMap;
use pdf::file::ScanItem;
use pdf::object::PlainRef;
use pdf::primitive::Primitive;

/// An object id
pub type ObjectID = u64;

/// Used in order to keep a list of object names and one list of the concrete objects
/// retrieved with pdf-rs
#[derive(Clone, Debug)]
pub struct ObjectsList {
    /// list of single objects to display in the lateral view
    pub objects: Vec<SingleObject>,
    /// list of single objects that hold the content
    pub objects_extended: Vec<SingleObjectExtended>,
    /// a map that lists key (object_id) -> value (the concrete index for the object)
    /// TODO: change the value to usize
    pub map: HashMap<ObjectID, u64>,
}

/// A small representation of the object to ensure I'm not consuming bytes of data while iterating
/// on it for various goals (e.g. display them in the menu)
#[derive(Clone, Debug)]
pub struct SingleObject {
    /// object name
    pub name: String,
    /// identifier for the object
    pub id: u64,
}

/// The object itself with the details
#[derive(Clone, Debug)]
pub struct SingleObjectExtended {
    /// internal key to access the object (fetched from PDF)
    pub id: ObjectID,
    /// internal name of the object
    pub name: String,
    /// internal ref for the object
    pub plain_ref: PlainRef,
    /// holds a serie of details and internals of the object given by pdf-rs
    pub object_content: Primitive,
    /// the raw buffer
    pub raw_buffer: Option<Vec<u8>>,
    /// the file content based on the buffer, None for the object that are not stream
    pub file_content: Option<FileContent>,
    /// references of the object
    pub references: Vec<u64>,
}

/// The type of the content associated to a stream
#[derive(Clone, Debug)]
pub enum FileContent {
    /// we can't infer the file embedded into the object based on the raw_buffer
    Unknown(Vec<u8>),
    /// The q operator shall push a copy of the entire grapihcs state onto the stack
    /// This operator can be used to encapsulate a graphical element so that it can modify parameters
    /// of the graphics state (page 132 of PDF32000)
    GraphicState(Vec<u8>),
    /// Textual
    TextASCII(Vec<u8>, String),
}

impl ObjectsList {
    /// Creates a new instance
    pub fn new(objects: Vec<ScanItem>) -> Result<Self> {
        let mut result = vec![];
        let mut objects_extended = vec![];
        for o in objects {
            match o {
                ScanItem::Trailer(dict) => {
                    println!("{}", dict);
                }
                ScanItem::Object(plain_ref, object_content) => {
                    let a = SingleObject::new(
                        object_content.get_debug_name().to_string(),
                        plain_ref.id,
                    )?;
                    result.push(a.clone());
                    let complete_name = format!("{} – {}", a.id, a.name);
                    let n =
                        SingleObjectExtended::new(a.id, complete_name, plain_ref, object_content)?;
                    objects_extended.push(n);
                }
            }
        }

        objects_extended.sort_by(|a, b| a.id.cmp(&b.id));
        result.sort_by(|a, b| a.id.cmp(&b.id));
        let map = objects_extended
            .iter()
            .enumerate()
            .map(|(i, o)| (o.id as ObjectID, i as u64))
            .collect();

        Ok(Self {
            objects: result,
            objects_extended,
            map,
        })
    }
}

impl SingleObject {
    /// Creates a new instance
    pub fn new(name: String, id: u64) -> Result<Self> {
        Ok(Self { name, id })
    }
}

impl SingleObjectExtended {
    /// Creates a new instance
    pub fn new(
        id: u64,
        name: String,
        plain_ref: PlainRef,
        object_content: Primitive,
    ) -> Result<Self> {
        Ok(Self {
            id,
            name,
            plain_ref,
            object_content,
            raw_buffer: None,
            file_content: None,
            references: vec![],
        })
    }

    /// Insert a buffer
    pub fn insert_raw_buffer(&mut self, buffer: &[u8]) {
        self.raw_buffer = Some(buffer.to_vec())
    }

    /// Insert file content
    pub fn insert_file_content(&mut self, file_content: FileContent) {
        self.file_content = Some(file_content)
    }
}
