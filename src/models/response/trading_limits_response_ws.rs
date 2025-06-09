use super::TradingLimit;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingLimitsResponseWS {
    pub limits: Vec<TradingLimit>,
}
