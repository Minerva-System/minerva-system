//! This module describes a structure for whenever an error when parsing an
//! enumeration happens. This is mostly related to enforcing certain numbers
//! as database enumerations.

/// Represents an enumeration conversion error.
#[derive(Debug)]
pub struct EnumError {
    pub message: String,
    pub status: u16,
}

impl EnumError {
    /// Creates a conversion error for whenever a precondition fails.
    /// In that case, the system needs to be explicitly recovered.
    pub fn precondition_failed(message: String) -> Self {
        Self {
            message,
            status: 412,
        }
    }
}
