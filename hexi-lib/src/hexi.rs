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
        for line in self.dump_file() {
            println!("{}", line);
        }

        Ok(())
    }

    /// Creates a simple dump of the loaded data, this is useful for debugging.
    ///
    /// This returns an iterator for performance reasons - with an iterator, I can begin printing straight away.
    /// This removes a large delay if a large file is dumped.
    fn dump_file(&self) -> impl Iterator<Item = String> + '_ {
        // for every line of the document
        (0..self.document.len()).map(move |line| {
            // get that line formatted as a correct string, and print with correct line number
            format!(
                "{:04X}| {}",
                line * self.document.get_line_length(),
                self.document.format_line(line)
            )
        })
    }
}
