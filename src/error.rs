use std::{fmt::Debug, num::ParseIntError, ops::Range};

use home_config::JsonError;

#[derive(Debug)]
pub enum StylixError {
    ParseColorError(ParseColorError),
    ConfigLoadError(JsonError),
}
impl std::error::Error for StylixError {}
impl std::fmt::Display for StylixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

#[derive(Debug)]
pub enum ParseColorError {
    InvalidLength { value: String, length: usize },
    IndexOutOfRange { value: String, range: Range<usize> },
    ParseIntError { value: String, error: ParseIntError },
}

impl std::error::Error for ParseColorError {}
impl std::fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}
impl ParseColorError {
    pub fn to_stylix_error(self) -> StylixError {
        StylixError::ParseColorError(self)
    }
}
