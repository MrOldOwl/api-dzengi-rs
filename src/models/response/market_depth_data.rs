use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MarketDepthData {
    pub bid: HashMap<String, f64>,
    pub ofr: HashMap<String, f64>,
    pub ts: i64,
}
