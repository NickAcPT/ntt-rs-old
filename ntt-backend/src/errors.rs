use actix_web::ResponseError;
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum NttBackendError {
    #[error("NTT Core Error: {0}")]
    NttCoreError(#[from] ntt_core::errors::NttCoreError),
}

pub(crate) type NttBackendResult<T> = Result<T, NttBackendError>;

impl ResponseError for NttBackendError {}
