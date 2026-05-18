use std::{error::Error, fmt};

/// Convenience result type for Scripting Bridge operations.
pub type Result<T> = std::result::Result<T, ScriptingBridgeError>;

/// Error returned when a Scripting Bridge bridge call fails.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScriptingBridgeError {
    /// Names the bridge function that reported the failure.
    pub function: &'static str,
    /// Holds the failure message returned by the bridge.
    pub message: String,
}

impl ScriptingBridgeError {
    pub(crate) fn new(function: &'static str, message: impl Into<String>) -> Self {
        Self {
            function,
            message: message.into(),
        }
    }
}

impl fmt::Display for ScriptingBridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} failed: {}", self.function, self.message)
    }
}

impl Error for ScriptingBridgeError {}
