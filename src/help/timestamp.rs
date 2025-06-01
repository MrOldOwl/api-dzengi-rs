use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub fn timestamp_now() -> Result<u64, SystemTimeError> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64)
}
