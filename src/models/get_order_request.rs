#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    #[validate(length(min = 1))]
    pub api_key: String,
    #[validate(length(min = 1))]
    pub order_id: String,
    #[validate(range(max = 60000))]
    pub recv_window: Option<i64>,
    #[validate(length(min = 1))]
    pub signature: String,
    #[validate(length(min = 1))]
    pub symbol: String,
    pub timestamp: i64,
}
