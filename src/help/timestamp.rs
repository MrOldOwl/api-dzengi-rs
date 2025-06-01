use crate::enums::CorrectionLocalTime;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub fn timestamp_now(correction: CorrectionLocalTime) -> Result<u128, SystemTimeError> {
    let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    Ok(match correction {
        CorrectionLocalTime::None => time,
        CorrectionLocalTime::Add(delta) => time + delta,
        CorrectionLocalTime::Sub(delta) => time - delta,
    })
}
