use clap::{load_yaml, value_t_or_exit, App};
use hexi_lib::{error::Error, hexi::Hexi, options::Options};

/// Gets the options this program was ran with.
fn get_options() -> Options {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    Options {
        file: value_t_or_exit!(matches, "file", String),
        section_length: value_t_or_exit!(matches, "sections_length", usize),
        sections_per_line: value_t_or_exit!(matches, "sections_count", usize),
        chunk_size: value_t_or_exit!(matches, "chunk_size", usize),
    }
}

/// Runs the main hex viewer, returning errors as soon as they happen
fn run(options: Options) -> Result<(), Error> {
    let mut hexi = Hexi::with_options(options)?;
    hexi.run()?;

    Ok(())
}

fn main() {
    let options = get_options();
    if let Err(e) = run(options) {
        eprintln!("Something went wrong!\n{}", e);
    }
}
