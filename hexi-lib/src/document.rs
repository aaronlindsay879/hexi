use crate::{error::Error, options::Options};
use std::ops::Index;

/// Stores the currently opened document.
#[derive(Debug)]
pub(crate) struct Document {
    name: String,
    data: Vec<u8>,
    section_length: usize,
    sections_per_line: usize,
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
        })
    }

    /// Gets the number of bytes drawn on a single line.
    pub(crate) fn get_line_length(&self) -> usize {
        self.section_length * self.sections_per_line
    }

    /// Formats a given line of the document.
    ///
    /// # Panics
    /// Panics if the line is out of range - should not be a concern since this is only used internally.
    pub(crate) fn format_line(&self, line: usize) -> String {
        let data = &self[line];

        // first of all split the data into chunks with correct section length
        // can make reasonable worst case guess of string length upper bound, so have that as the seed value
        data.chunks(self.section_length)
            .fold(
                String::with_capacity(self.get_line_length() * 3),
                |section_acc, section| {
                    // option one
                    let section_str = section.iter().fold(
                        String::with_capacity(self.section_length * 3),
                        |acc, val| acc + &format!("{:02X} ", val),
                    );

                    // then join each section up, delimited by 3 spaces - will be 4 spaces when extra space from section_str is included
                    // this leads to a string of the form "[section]    [section] ...    " where each section is "[byte] [byte] ... "
                    section_acc + &section_str + "   "
                },
            )
            .trim_end()
            .to_string()
    }

    /// Returns the number of lines of data stored in this document.
    pub(crate) fn len(&self) -> usize {
        // performs integer division of self.data.len() / self.get_line_length(), rounding up instead of down
        (self.data.len() - 1) / self.get_line_length() + 1
    }
}
