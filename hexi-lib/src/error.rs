use std::{
    error,
    fmt::{self, Display, Formatter},
    io,
};

use io::ErrorKind;

#[derive(Debug)]
pub enum Error {
    FileNotFound,
    InvalidFileName,
    Other,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::FileNotFound => write!(f, "File not found"),
            Self::InvalidFileName => write!(f, "File name is invalid"),
            Self::Other => write!(f, "Unknown error"),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            ErrorKind::NotFound | ErrorKind::PermissionDenied => Self::FileNotFound,
            _ => Self::Other,
        }
    }
}
