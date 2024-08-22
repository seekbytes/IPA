use anyhow::{bail, Result};
use log::info;
use std::fs::File;
use std::io::Read;

/// Holds very easy details for the input file (provided by the open file window or the dropped file)
pub struct InputFile {
    /// the path of the input file
    pub path: String,
    /// the content of the file
    pub buffer: Vec<u8>,
}

/// The start of a common PDF file
const PDF_FILE_SIGNATURE: (u8, u8, u8, u8) = (0x25, 0x50, 0x44, 0x46);

impl InputFile {
    /// Creates a new instance of input file that checks if the input file is a correct PDF
    pub fn new(path: String) -> Result<Self> {
        let mut file = File::open(path.clone())?;
        // read the first 4 bytes of a pdf file
        // TODO: pdf spec mentions that the signature should be found in the first 1024 bytes
        let mut buffer = [0u8; 4];
        file.read_exact(&mut buffer)?;

        match (buffer[0], buffer[1], buffer[2], buffer[3]) {
            PDF_FILE_SIGNATURE => {
                info!("this is a pdf file")
            }
            _ => bail!(
                "Not a PDF file, obtained {:x} {:x} {:x} {:x}",
                buffer[0],
                buffer[1],
                buffer[2],
                buffer[3]
            ),
        }

        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;

        Ok(Self { path, buffer })
    }
}
