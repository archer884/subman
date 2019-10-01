use serde::de;
use std::error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum IniError {
    Serde(String),
    ExpectedString,
}

impl Display for IniError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IniError::Serde(e) => f.write_str(e),
            IniError::ExpectedString => f.write_str("A value was expected"),
        }
    }
}

impl error::Error for IniError {}

impl de::Error for IniError {
    fn custom<T: Display>(msg: T) -> Self {
        IniError::Serde(msg.to_string())
    }
}
