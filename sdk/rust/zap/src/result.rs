use crate::ZapError;

/// The standard result type used throughout the Zap SDK.
pub type Result<T> = std::result::Result<T, ZapError>;
