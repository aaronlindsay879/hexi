use crate::{document::Document, error::Error, options::Options};

/// Highest level representation of hex viewer, abstracts away all login and allows it to be ran.
pub struct Hexi {
    document: Document,
}

impl Hexi {
    /// Creates a hex viewer implementation, getting the file name from the first argument passed to it.
    pub fn with_options(options: Options) -> Result<Self, Error> {
        let document = Document::from_options(options)?;

        Ok(Self { document })
    }

    /// Starts the hex viewer - this runs for an indefinite amount of time, handling user input.
    pub fn run(&mut self) -> Result<(), Error> {
        println!("{}", self.dump_file());

        Ok(())
    }

    /// Creates a simple dump of the loaded data, this is useful for debugging.
    fn dump_file(&self) -> String {
        // for every line of the document
        (0..self.document.len())
            .map(|line| {
                // get that line formatted as a correct string, and print with correct line number
                format!(
                    "{:04X}| {}",
                    line * self.document.get_line_length(),
                    self.document.format_line(line)
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
