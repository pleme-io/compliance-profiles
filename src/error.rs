use std::fmt;
use std::io;

/// Errors that can occur during profile operations.
#[derive(Debug)]
pub enum Error {
    /// I/O error (file not found, permission denied, etc.).
    Io(io::Error),
    /// JSON serialization/deserialization error.
    Json(serde_json::Error),
    /// Directory walk error.
    Walk(walkdir::Error),
    /// Profile not found by ID.
    ProfileNotFound(String),
    /// Duplicate profile ID.
    DuplicateProfile(String),
    /// Invalid profile directory (missing inspec.yml, etc.).
    InvalidProfile(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Json(e) => write!(f, "JSON error: {e}"),
            Self::Walk(e) => write!(f, "directory walk error: {e}"),
            Self::ProfileNotFound(id) => write!(f, "profile not found: {id}"),
            Self::DuplicateProfile(id) => write!(f, "duplicate profile ID: {id}"),
            Self::InvalidProfile(msg) => write!(f, "invalid profile: {msg}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Json(e) => Some(e),
            Self::Walk(e) => Some(e),
            Self::ProfileNotFound(_) | Self::DuplicateProfile(_) | Self::InvalidProfile(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<walkdir::Error> for Error {
    fn from(e: walkdir::Error) -> Self {
        Self::Walk(e)
    }
}

/// Convenience alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;
