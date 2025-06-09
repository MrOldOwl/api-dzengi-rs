#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalQuote {
    pub bid: f64,
    pub bid_qty: f64,
    pub ofr: f64,
    pub ofr_qty: f64,
    pub symbol_name: String,
    pub timestamp: i64,
}
