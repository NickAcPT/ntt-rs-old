use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum NttCoreError {}

pub(crate) type NttCoreResult<T> = Result<T, NttCoreError>;
