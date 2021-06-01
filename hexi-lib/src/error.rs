use std::{
    error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum Error {
    FileNotFound(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::FileNotFound(file) => write!(f, "File not found at `{}`", file),
        }
    }
}

impl error::Error for Error {}
