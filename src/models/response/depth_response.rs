#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthResponse {
    pub asks: Vec<Vec<f64>>,
    pub bids: Vec<Vec<f64>>,
    pub last_update_id: i64,
}
