use core::{error, num};
use std::io;

use crate::tokens::Token;

#[derive(Debug)]
pub enum RscError {
    UnexpectedToken(Box<String>, Token),
    Io(io::Error),
    ParseNum(Box<dyn error::Error>),
}

impl From<io::Error> for RscError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<num::ParseFloatError> for RscError {
    fn from(value: num::ParseFloatError) -> Self {
        Self::ParseNum(Box::new(value))
    }
}

impl From<num::ParseIntError> for RscError {
    fn from(value: num::ParseIntError) -> Self {
        Self::ParseNum(Box::new(value))
    }
}
