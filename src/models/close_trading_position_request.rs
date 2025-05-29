#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(rename_all = "camelCase")]
pub struct CloseTradingPositionRequest {
    #[validate(length(min = 1))]
    pub api_key: String,
    #[validate(length(min = 1))]
    pub position_id: String,
    #[validate(range(max = 60000))]
    pub rec_window: Option<i64>,
    #[validate(length(min = 1))]
    pub signature: String,
    pub timestamp: i64,
}
