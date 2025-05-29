#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(rename_all = "camelCase")]
pub struct PositionHistoryRequest {
    pub api_key: String,
    #[validate(range(max = 60000))]
    pub recv_window: Option<i64>,
    pub from: Option<i64>,
    pub limit: Option<i32>,
    pub signature: String,
    pub symbol: Option<String>,
    pub timestamp: i64,
    pub to: Option<i64>,
}
