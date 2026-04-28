//! Errors that can happen in the UI layer.

#[derive(thiserror::Error, Debug)]
pub enum UiError {
    #[error("Storage backend: {0}")]
    Storage(#[from] StorageError),
}

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Not enough space in the container.")]
    NotEnoughStorage,
}
