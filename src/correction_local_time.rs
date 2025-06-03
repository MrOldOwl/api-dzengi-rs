use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::errors::CorrectionTimeError;

#[derive(
    Debug, Default, Hash, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub enum CorrectionTime {
    #[default]
    None,
    Add(u128),
    Sub(u128),
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct CorrectionLocalTime {
    correction: CorrectionTime,
    instant: Option<Instant>,
    life_time: Duration,
}
impl Default for CorrectionLocalTime {
    fn default() -> Self {
        Self {
            correction: Default::default(),
            instant: Default::default(),
            life_time: Duration::from_secs(60 * 60),
        }
    }
}
impl CorrectionLocalTime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_correction(mut self, correction: CorrectionTime) -> Self {
        self.instant = match correction {
            CorrectionTime::None => None,
            _ => Some(Instant::now()),
        };
        self.correction = correction;
        self
    }

    pub fn with_max_life_time(&mut self, life_time: Duration) -> Result<(), CorrectionTimeError> {
        if life_time.as_secs() < 10 {
            return Err(CorrectionTimeError::LifeTime(life_time.as_secs()));
        }
        self.life_time = life_time;
        return Ok(());
    }

    pub fn is_outdated(&self) -> bool {
        match self.instant {
            Some(x) => x.elapsed() > self.life_time,
            None => false,
        }
    }

    pub fn timestamp_now(&self) -> Result<u128, CorrectionTimeError> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        Ok(match self.correction {
            CorrectionTime::None => time,
            CorrectionTime::Add(delta) => time + delta,
            CorrectionTime::Sub(delta) => time - delta,
        })
    }
}
