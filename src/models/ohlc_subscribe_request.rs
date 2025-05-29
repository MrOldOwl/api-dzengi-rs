use crate::enums::{CandleType, Interval};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OHLCSubscribeRequest {
    pub intervals: Vec<Interval>,
    pub symbols: Vec<String>,
    #[serde(rename = "type")]
    pub candle_type: CandleType,
}
