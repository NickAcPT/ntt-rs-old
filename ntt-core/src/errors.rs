use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum NttCoreError {
    #[error("Unable to parse OAuth2 URL: {0}")]
    OAuth2UrlParseError(#[from] oauth2::url::ParseError),
}

pub(crate) type NttCoreResult<T> = Result<T, NttCoreError>;
