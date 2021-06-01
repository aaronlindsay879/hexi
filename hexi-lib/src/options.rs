#[derive(Default)]
pub struct Options {
    /// Name of the file
    pub file: String,
    /// Length of each individual sequence of bytes
    pub section_length: usize,
    /// Number of byte sections on each line
    pub sections_per_line: usize,
}
