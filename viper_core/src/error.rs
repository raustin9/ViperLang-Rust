use strum_macros::Display;
use thiserror::Error;

/// An enumeration of errors types that can be encountered while compiling
#[derive(Debug, Error, Display)]
pub enum ViperError {
    IoError,
    ParserError,
}

impl ViperError {
    /// Return the error code for this error
    pub fn error_code(&self) -> i32 {
        0
    }
}


/// An enumeration of warnings that can be encountered while compiling
#[derive(Debug, Error)]
pub enum ViperWarning {

}



