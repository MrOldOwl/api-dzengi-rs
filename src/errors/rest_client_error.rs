use super::CryptoError;
use crate::errors::CorrectionTimeError;
use std::fmt::Display;
use thiserror::Error;
use tokio_tungstenite::tungstenite;

pub type DzengiRestClientResult<T> = Result<T, DzengiRestClientError>;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DzengiCorrectError {
    pub code: i64,
    pub msg: String,
}
impl Display for DzengiCorrectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {}, msg: {}", self.code, self.msg)
    }
}

#[derive(Debug, Error)]
pub enum DzengiRestClientError {
    #[error("User settings is None")]
    NoneUserSettings,
    #[error("{0}")]
    Time(#[from] std::time::SystemTimeError),
    #[error("{0}")]
    Crypto(#[from] CryptoError),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Reqwest string: {0}")]
    DzengiUncorrected(String),
    #[error("{0}")]
    DzengiCorrect(DzengiCorrectError),
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0}")]
    CorrectionTime(#[from] CorrectionTimeError),
    #[error("{0}")]
    Tungstenite(#[from] tungstenite::Error),
}
