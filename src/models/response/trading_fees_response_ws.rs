use super::TradingFee;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingFeesResponseWS {
    pub fees: Vec<TradingFee>,
}
