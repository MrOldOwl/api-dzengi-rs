#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingLimit {
    pub last_price: f64,
    pub max_volume: f64,
    pub min_step: f64,
    pub min_volume: f64,
    pub name: String,
    pub symbol: String,
    pub tick_size: f64,
}
