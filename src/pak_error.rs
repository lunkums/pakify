use std::{fmt, io};

#[derive(Debug)]
pub enum PakError {
    Io(io::Error),
    InvalidField(&'static str),
    MissingEntry(String),
    UnexpectedEof,
}

impl From<io::Error> for PakError {
    fn from(error: io::Error) -> Self {
        PakError::Io(error)
    }
}

impl fmt::Display for PakError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PakError::Io(io_error) => io_error.fmt(f),
            PakError::InvalidField(field) => write!(f, "Invalid field ({})", field),
            PakError::MissingEntry(entry) => write!(f, "Missing entry ({})", entry),
            PakError::UnexpectedEof => f.write_str("Unexpected EOF"),
        }
    }
}
