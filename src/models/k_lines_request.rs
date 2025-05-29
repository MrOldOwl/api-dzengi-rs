#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KLinesRequest {
    pub end_time: Option<i64>,
    pub interval: String,
    pub limit: Option<i32>,
    pub price_type: Option<String>,
    pub start_time: Option<i64>,
    pub symbol: String,
    #[serde(rename = "type")]
    pub kind: String,
}
