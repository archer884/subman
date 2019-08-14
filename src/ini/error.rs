use serde::de;
use std::error;
use std::fmt::{self, Display};

type ParserError = pest::error::Error<crate::ini::parser::Rule>;

#[derive(Debug)]
pub enum IniError {
    Parser(ParserError),
    Serde(String),
    ExpectedString,
}

impl From<ParserError> for IniError {
    fn from(e: ParserError) -> Self {
        IniError::Parser(e)
    }
}

impl Display for IniError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IniError::Parser(e) => write!(f, "{}", e),
            IniError::Serde(e) => f.write_str(e),
            IniError::ExpectedString => f.write_str("A value was expected"),
        }
    }
}

impl error::Error for IniError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            IniError::Parser(e) => Some(e),

            _ => None,
        }
    }
}

impl de::Error for IniError {
    fn custom<T: Display>(msg: T) -> Self {
        IniError::Serde(msg.to_string())
    }
}
