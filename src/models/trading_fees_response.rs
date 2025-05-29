use super::OvernightRate;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingFeesResponse {
    pub fee: f64,
    pub name: String,
    pub overnight_fee_timestamp: i64,
    pub overnight_rates: Vec<OvernightRate>,
    pub symbol: String,
}
