#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(rename_all = "camelCase")]
pub struct AllMyTradesRequest {
    #[validate(length(min = 1))]
    pub api_key: String,
    pub end_time: Option<i64>,
    pub limit: Option<i32>,
    #[validate(range(max = 60000))]
    pub recv_window: Option<i64>,
    #[validate(length(min = 1))]
    pub signature: String,
    pub start_time: Option<i64>,
    #[validate(length(min = 1))]
    pub symbol: String,
    pub timestamp: i64,
}
