#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggTradesRequest {
    pub end_time: Option<i64>,
    pub limit: Option<i32>,
    pub start_time: Option<i64>,
    pub symbol: String,
}
