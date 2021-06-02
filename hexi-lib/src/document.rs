use crate::{error::Error, options::Options};
use std::ops::Index;

/// Stores the currently opened document.
#[derive(Debug)]
pub(crate) struct Document {
    name: String,
    data: Vec<u8>,
    section_length: usize,
    sections_per_line: usize,
    chunk_size: usize,
}

impl Index<usize> for Document {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.get_line_length();
        let end = std::cmp::min(start + self.get_line_length(), self.data.len());

        &self.data[start..end]
    }
}

impl Document {
    /// Constructs a document by reading data from a given path.
    pub(crate) fn from_options(options: Options) -> Result<Self, Error> {
        let name = options.file;
        let data = std::fs::read(&name)?;

        Ok(Self {
            name,
            data,
            section_length: options.section_length,
            sections_per_line: options.sections_per_line,
            chunk_size: options.chunk_size,
        })
    }

    /// Gets the number of bytes drawn on a single line.
    pub(crate) fn get_line_length(&self) -> usize {
        self.section_length * self.sections_per_line
    }

    /// Formats a single byte chunk. This respects `self.chunk_size`, and appends that many bytes into a single string
    fn format_chunk(&self, bytes: &[u8]) -> String {
        let mut string = String::with_capacity(self.chunk_size * 2);
        string.extend(bytes.iter().map(|byte| format!("{:02X}", byte)));

        string
    }

    /// Formats a single byte section. This respects both `self.section_length` and `self.chunk_size`.
    /// This returns a string of chunks, delimited by spaces.
    fn format_section(&self, bytes: &[u8]) -> String {
        let mut string = String::with_capacity(self.section_length * 3);
        string.extend(
            bytes
                .chunks(self.chunk_size)
                .map(|chunk| self.format_chunk(chunk) + " "),
        );

        string
    }

    /// Formats a given line of the document.
    ///
    /// # Panics
    /// Panics if the line is out of range - should not be a concern since this is only used internally.
    pub(crate) fn format_line(&self, line: usize) -> String {
        let data = &self[line];

        let mut line = String::with_capacity(self.get_line_length() * 3);
        line.extend(
            data.chunks(self.section_length)
                .map(|section| self.format_section(section) + "   "),
        );

        line
    }

    /// Returns the number of lines of data stored in this document.
    pub(crate) fn len(&self) -> usize {
        // performs integer division of self.data.len() / self.get_line_length(), rounding up instead of down
        (self.data.len() - 1) / self.get_line_length() + 1
    }
}

#[cfg(test)]
mod test {
    use super::{Options, *};

    #[test]
    fn from_options() {
        let document = Document::from_options(Options {
            file: String::from("test.data"),
            section_length: 8,
            sections_per_line: 2,
            chunk_size: 1,
        });

        assert!(document.is_ok());

        let document = document.unwrap();
        assert_eq!(
            document.data,
            &[
                0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
                0x67, 0x68
            ]
        )
    }

    #[test]
    fn get_line_length() {
        let document = Document::from_options(Options {
            file: String::from("test.data"),
            section_length: 8,
            sections_per_line: 2,
            chunk_size: 1,
        })
        .unwrap();

        assert_eq!(document.get_line_length(), 16);

        let document = Document::from_options(Options {
            file: String::from("test.data"),
            section_length: 2,
            sections_per_line: 2,
            chunk_size: 2,
        })
        .unwrap();

        assert_eq!(document.get_line_length(), 4);
    }

    #[test]
    fn format_chunk() {
        let document = Document::from_options(Options {
            file: String::from("test.data"),
            section_length: 8,
            sections_per_line: 2,
            chunk_size: 1,
        })
        .unwrap();

        assert_eq!(document.format_chunk(&[0x31]), "31");

        let document = Document::from_options(Options {
            file: String::from("test.data"),
            section_length: 2,
            sections_per_line: 2,
            chunk_size: 2,
        })
        .unwrap();

        assert_eq!(document.format_chunk(&[0x31, 0x32]), "3132");
    }

    #[test]
    fn format_section() {
        let document = Document::from_options(Options {
            file: String::from("test.data"),
            section_length: 8,
            sections_per_line: 2,
            chunk_size: 1,
        })
        .unwrap();

        assert_eq!(
            document
                .format_section(&[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38])
                .trim(),
            "31 32 33 34 35 36 37 38"
        );

        let document = Document::from_options(Options {
            file: String::from("test.data"),
            section_length: 2,
            sections_per_line: 2,
            chunk_size: 2,
        })
        .unwrap();

        assert_eq!(
            document.format_section(&[0x31, 0x32, 0x33, 0x34]).trim(),
            "3132 3334"
        );
    }

    #[test]
    fn format_line() {
        let document = Document::from_options(Options {
            file: String::from("test.data"),
            section_length: 8,
            sections_per_line: 2,
            chunk_size: 1,
        })
        .unwrap();

        assert_eq!(
            document.format_line(0).trim(),
            "31 32 33 34 35 36 37 38    61 62 63 64 65 66 67 68"
        );

        let document = Document::from_options(Options {
            file: String::from("test.data"),
            section_length: 2,
            sections_per_line: 2,
            chunk_size: 2,
        })
        .unwrap();

        assert_eq!(document.format_line(0).trim(), "3132    3334");
    }
}
