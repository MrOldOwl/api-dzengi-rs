use std::time::SystemTimeError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CorrectionTimeError {
    #[error("The minimum lifetime is 10 seconds, your time is {0} seconds")]
    LifeTime(u64),
    #[error("{0}")]
    System(#[from] SystemTimeError),
}
