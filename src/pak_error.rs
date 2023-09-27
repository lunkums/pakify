use std::{fmt, io};

pub enum PakError {
    Io(io::Error),
    InvalidField(&'static str),
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
            PakError::InvalidField(field_name) => write!(f, "Invalid field ({})", field_name),
            PakError::UnexpectedEof => f.write_str("Unexpected EOF"),
        }
    }
}
