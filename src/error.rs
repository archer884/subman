use crate::ini::error::IniError;
use std::error;
use std::fmt::{self, Display};
use std::io;

#[derive(Debug)]
pub enum Error {
    Ini(IniError),
    IO(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Ini(e) => write!(f, "{}", e),
            Error::IO(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Ini(e) => Some(e),
            Error::IO(e) => Some(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<IniError> for Error {
    fn from(e: IniError) -> Self {
        Error::Ini(e)
    }
}
