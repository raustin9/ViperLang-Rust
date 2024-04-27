use core::fmt;
use std::error::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VError {
    IoError(IoError),
    LexerError,
    ParserError
}

#[derive(Clone, Debug, PartialEq, Eq,)]
pub struct IoError {
    msg: String,
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl fmt::Display for VError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Default VError msg")
    }
}

impl Error for IoError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl Error for VError {
    fn description(&self) -> &str {
        "Default VError message"
    }
}

impl IoError {
    pub fn new(msg: &str) -> VError {
        VError::IoError(IoError {
            msg: msg.to_owned(),
        })
    }
}
