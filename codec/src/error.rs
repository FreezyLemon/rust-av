use core::fmt;

use alloc::string::String;

/// General coding errors.
#[derive(Debug)]
pub enum Error {
    /// Invalid input data.
    InvalidData,
    /// A coding operation needs more data to be completed.
    MoreDataNeeded,
    /// Incomplete input configuration.
    ConfigurationIncomplete,
    /// Invalid input configuration.
    ConfigurationInvalid,
    /// Unsupported requested feature.
    Unsupported(String),
    // TODO add support for dependency-specific errors here
    // Inner(failure::Context)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidData => write!(f, "Invalid Data"),
            Error::MoreDataNeeded => write!(f, "Additional data needed"),
            Error::ConfigurationIncomplete => write!(f, "Configuration Incomplete"),
            Error::ConfigurationInvalid => write!(f, "Configuration Invalid"),
            Error::Unsupported(uf) => write!(f, "Unsupported feature {uf}"),
        }
    }
}

impl core::error::Error for Error {}

/// A specialized `Result` type for coding operations.
pub type Result<T> = ::core::result::Result<T, Error>;
