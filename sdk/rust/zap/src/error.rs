use std::fmt;

/// Errors returned by the Zap SDK.
#[derive(Debug)]
pub enum ZapError {
    /// A value had the wrong Zap type.
    InvalidType { expected: &'static str, found: u8 },

    /// A function received the wrong number of arguments.
    InvalidArgumentCount { expected: usize, found: usize },

    /// A string contained invalid UTF-8.
    InvalidUtf8,

    /// An object handle was invalid.
    InvalidHandle,

    /// Native code attempted an unsupported operation.
    Unsupported(&'static str),

    /// Generic runtime error.
    Runtime(String),
}

impl fmt::Display for ZapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidType { expected, found } => {
                write!(f, "expected {}, found Zap type {}", expected, found)
            }

            Self::InvalidArgumentCount { expected, found } => {
                write!(f, "expected {} arguments, got {}", expected, found)
            }

            Self::InvalidUtf8 => {
                write!(f, "invalid UTF-8 string")
            }

            Self::InvalidHandle => {
                write!(f, "invalid native object handle")
            }

            Self::Unsupported(feature) => {
                write!(f, "{} is not supported", feature)
            }

            Self::Runtime(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}

impl std::error::Error for ZapError {}
