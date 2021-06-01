use crate::error::Error;

/// Highest level representation of hex viewer, abstracts away all login and allows it to be ran.
pub struct Hexi;

impl Default for Hexi {
    fn default() -> Self {
        Self
    }
}

impl Hexi {
    /// Starts the hex viewer - this runs for an indefinite amount of time, handling user input.
    pub fn run(&mut self) -> Result<(), Error> {
        println!("started running");

        Ok(())
    }
}
