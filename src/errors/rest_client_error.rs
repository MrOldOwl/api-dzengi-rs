use thiserror::Error;

pub type DzengiRestClientResult<T> = Result<T, DzengiRestClientError>;

#[derive(Debug, Error)]
pub enum DzengiRestClientError {
    #[error("User settings is None")]
    NoneUserSettings,
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
}
