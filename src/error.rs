//! Errors that can happen in the UI layer.

/// Errors that can happen while drawing the UI.
#[derive(thiserror::Error, Debug)]
pub enum UiError {
    /// Storage constraints reached.
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    // #[error("Screen backend error: {0}")]
    // Backend(#[from] StorageError),
}

/// Storage constraints reached.
#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    /// Not enough space in the container..
    #[error("Not enough space in the container.")]
    NotEnoughStorage,
}
