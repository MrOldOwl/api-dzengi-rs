#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeEventReg {
    pub id: i32,
    pub order_id: String,
    pub price: f64,
    pub size: f64,
    pub symbol: String,
    pub ts: i64,
}
