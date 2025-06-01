use hmac::digest::InvalidLength;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
    #[error("Params is empty")]
    ParamsEmpty,
    #[error("{0}")]
    InvalidLength(#[from] InvalidLength),
}
