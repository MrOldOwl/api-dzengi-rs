#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hr {
    pub ask_price: String,
    pub bid_price: String,
    pub close_time: i64,
    pub high_price: String,
    pub last_price: String,
    pub last_qty: String,
    pub low_price: String,
    pub open_price: String,
    pub open_time: i64,
    pub prev_close_price: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub quote_volume: String,
    pub symbol: String,
    pub volume: String,
    pub weighted_avg_price: String,
}
