use super::OvernightRate;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingFeesResponse {
    pub fee: f64,
    pub name: String,
    pub symbol: String,
    pub overnight_fee_timestamp: Option<i64>,
    pub overnight_rates: Option<Vec<OvernightRate>>,
}
