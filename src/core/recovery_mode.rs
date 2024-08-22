use crate::core::input_file::InputFile;
use anyhow::Result;
use std::collections::HashMap;

/// To prevent users opening tons of issues because their malformed PDF is not supported by IPA,
/// I decided to implement a "Recovery mode" that is basically the clone of PDFid by Didier Stevens.
///
/// We iterate on the strings fetched from the PDF file and see if there's something "malicious".
pub struct RecoveryMode {
    /// input file
    pub input_file: InputFile,
    /// how many times I encountered that item
    pub frequencies: HashMap<String, usize>,
}

impl RecoveryMode {
    /// Try to parse it
    pub fn parse(path: String) -> Result<Self> {
        let input_file = InputFile::new(path)?;

        let mut strings = vec![];
        let mut stringa = "".to_string();

        // extract strings from the pdf file
        for byt in &input_file.buffer {
            // for the sake of readability: if a byte is an ascii character, excluding '\n' and
            // the others, we append it to a workspace string.
            // As soon as we have a character that is not an ascii character or a control character,
            // we push the workspace string to the strings extracted
            if byt.is_ascii() && !byt.is_ascii_control() {
                stringa.push(char::from(*byt))
            } else if !stringa.is_empty() {
                strings.push(stringa);
                stringa = "".to_string();
            }
        }

        // if the last character is an ASCII, re-run the check
        if !stringa.is_empty() {
            strings.push(stringa);
        }

        let keywords = [
            "obj",
            "endobj",
            "stream",
            "endstream",
            "xref",
            "trailer",
            "startxref",
            "/Page",
            "/Encrypt",
            "/ObjStm",
            "/JS",
            "/JavaScript",
            "/AA",
            "/OpenAction",
            "/AcroForm",
            "/JBIG2Decode",
            "/RichMedia",
            "/Launch",
            "/EmbeddedFile",
            "/XFA",
        ];

        let mut frequencies: HashMap<String, usize> = HashMap::new();

        for keyword in keywords {
            let mut frequencies_count = 0;
            for string in &strings {
                if string.contains(keyword) {
                    frequencies_count += 1;
                }
            }
            frequencies.insert(keyword.to_string(), frequencies_count);
        }

        Ok(Self {
            input_file,
            frequencies,
        })
    }
}
