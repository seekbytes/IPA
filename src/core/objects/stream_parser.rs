use crate::core::objects::collector::{FileContent, SingleObjectExtended};
use anyhow::Result;
use flate2::read::ZlibDecoder;
use log::warn;
use pdf::object::Resolve;
use pdf::primitive::Primitive;
use std::io::{Cursor, Read};

/// Helper that parse the stream and identifies the file content
pub struct StreamParser {}

impl StreamParser {
    /// Returns true if all the characters in the vector are ascii
    fn is_ascii(buffer: &[u8]) -> bool {
        buffer.iter().all(|u| u.is_ascii())
    }

    /// Identify the data based on the buffer
    fn identify_data_on_buffer(buffer: &[u8]) -> Result<FileContent> {
        let mut result = FileContent::Unknown(buffer.to_vec());
        if buffer.len() > 1 {
            result = match buffer[0] {
                // 'q'
                113 => FileContent::GraphicState(buffer.to_vec()),
                0_u8..=u8::MAX if Self::is_ascii(buffer) => {
                    let str = String::from_utf8(buffer.to_vec())?;
                    FileContent::TextASCII(buffer.to_vec(), str)
                }
                _ => FileContent::Unknown(buffer.to_vec()),
            };
        }

        Ok(result)
    }

    /// Decompress buffer
    fn decompress_buffer(name: &Primitive, buffer: &[u8]) -> Result<Vec<u8>> {
        let mut decompressed_data: Vec<u8> = Vec::new();
        if let Primitive::Name(filter_name) = name {
            match filter_name.as_str() {
                "FlateDecode" => {
                    if buffer.len() >= 2 {
                        match (buffer[0], buffer[1]) {
                            (0x78, 0xDA) | (0x78, 0x9c) => {
                                let mut decoder = ZlibDecoder::new(Cursor::new(buffer));
                                decoder.read_to_end(&mut decompressed_data)?;
                                return Ok(decompressed_data);
                            }
                            _ => {}
                        }
                    }
                    /*let mut deflater = DeflateDecoder::new(decompressed_data);
                    deflater.write_all(buffer)?;
                    decompressed_data = deflater.finish()?;
                    return Ok(decompressed_data)*/
                }
                _ => {
                    warn!("Not managed: {}", filter_name)
                }
            }
        }

        Ok(buffer.to_owned())

        // TODO: refactor me
        /*let mut decompressed_data: Vec<u8> = Vec::new();
        if let Some(Primitive::Name(filter_name)) = name {
            match filter_name.as_str() {
                "FlateDecode" if buffer.len() >= 2 => match (buffer[0], buffer[1]) {
                    (0x78, 0xDA) => {
                        let mut decoder = ZlibDecoder::new(Cursor::new(decompressed_data.clone()));
                        decoder.read_to_end(&mut decompressed_data)?;
                        if decompressed_data.is_empty() {
                            return Self::identify_data_on_buffer(&buffer);
                        } else {
                            return Self::identify_data_on_buffer(&decompressed_data);
                        }
                    }
                    _ => {
                        let mut decoder = DeflateDecoder::new(Cursor::new(buffer.clone()));
                        if let Ok(len) = decoder.read_to_end(&mut decompressed_data) {
                            return Self::identify_data_on_buffer(&decompressed_data);
                        }
                    }
                },

                "Dctdecode" => {
                    // not managed :(
                }
                _ => println!("Not managed: {}", filter_name),
            }

            if !decompressed_data.is_empty() {
                match (decompressed_data[0], decompressed_data[1]) {
                    (0x78, 0x9c) | (0x78, 0xDA) => {
                        let mut buff_2 = Vec::new();
                        let mut decoder = ZlibDecoder::new(Cursor::new(decompressed_data.clone()));
                        decoder.read_to_end(&mut buff_2)?;
                        Self::identify_data_on_buffer(&buff_2)
                    }

                    _ => Self::identify_data_on_buffer(&decompressed_data),
                }
            } else {
                Self::identify_data_on_buffer(&buffer)
            }
        } else {
            Self::identify_data_on_buffer(&buffer)
        }*/
    }

    /// Parse some bytes
    pub fn parse(
        objects: Vec<SingleObjectExtended>,
        resolver: &impl Resolve,
    ) -> Result<Vec<SingleObjectExtended>> {
        let mut new_objects = vec![];

        // 1) capture raw buffer and filter

        for object_ext in &objects {
            let mut object = object_ext.clone();

            let (buffer, filter, len) = match &object_ext.object_content {
                Primitive::Stream(str) => (
                    Some(str.raw_data(resolver)?.to_vec()),
                    str.info.get("Filter"),
                    str.info.get("Length"),
                ),
                _ => (None, None, None),
            };

            let mut result = None;

            match (&buffer, filter, len) {
                (Some(buffer), Some(filter_name), _) => {
                    object.insert_raw_buffer(buffer.as_slice());
                    let buffer_decompressed = Self::decompress_buffer(filter_name, buffer)?;
                    result = Some(Self::identify_data_on_buffer(&buffer_decompressed)?);
                }
                (Some(buffer), None, _) => {
                    object.insert_raw_buffer(buffer.as_slice());
                    result = Some(Self::identify_data_on_buffer(buffer)?)
                }
                (_, _, _) => {}
            }

            if let Some(file_content) = result {
                object.file_content = Some(file_content);
            }

            new_objects.push(object);
        }

        Ok(new_objects)
    }
}
