//! Errors that can happen in the UI layer.

use crate::backend::BackendError;

/// Errors that can happen while drawing the UI.
#[derive(thiserror::Error, Debug)]
pub enum UiError<BE: BackendError> {
    /// Storage constraints reached.
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    /// Error emitted from the backend.
    #[error("Screen backend error: {0}")]
    Backend(#[from] BE),
}

/// Storage constraints reached.
#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    /// Not enough space in the container..
    #[error("Not enough space in the container.")]
    NotEnoughStorage,
}
