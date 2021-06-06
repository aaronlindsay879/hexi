use clap::{load_yaml, value_t_or_exit, App};
use hexi_lib::{error::Error, hexi::Hexi, options::Options};

/// Gets the options this program was ran with.
fn get_options() -> (Options, bool) {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    (
        Options {
            file: value_t_or_exit!(matches, "file", String),
            section_length: value_t_or_exit!(matches, "sections_length", usize),
            sections_per_line: value_t_or_exit!(matches, "sections_count", usize),
            chunk_size: value_t_or_exit!(matches, "chunk_size", usize),
        },
        matches.occurrences_of("i") > 0,
    )
}
/// Runs the main hex viewer, respecting whether to use interactive mode or not.
fn run(mut hexi: Hexi, interactive: bool) -> Result<(), Error> {
    if interactive {
        hexi.run()
    } else {
        for line in hexi.dump_file() {
            println!("{}", line);
        }

        Ok(())
    }
}

fn main() {
    // if on unix, ensure the SIGPIPE signal doesn't cause program to panic - this allows pipes to be used with this program
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let (options, interactive) = get_options();
    Hexi::with_options(options)
        .and_then(|hexi| run(hexi, interactive))
        .unwrap_or_else(|e| eprintln!("Something went wrong!\n{}", e));
}
