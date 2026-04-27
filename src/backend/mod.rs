#[cfg(feature = "async")]
pub mod async_backend {
    #[async_trait::async_trait]
    pub trait AsyncLcdBackend {}
}

pub mod sync_backend {
    pub trait LcdBackend {}
}
