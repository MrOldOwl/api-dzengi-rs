#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(rename_all = "camelCase")]
pub struct EditExchangeOrderRequest {
    #[validate(length(min = 1))]
    pub api_key: String,
    pub expire_timestamp: Option<i64>,
    #[validate(length(min = 1))]
    pub order_id: String,
    pub price: Option<f64>,
    #[validate(range(max = 60000))]
    pub recv_window: Option<i64>,
    #[validate(length(min = 1))]
    pub signature: String,
    pub timestamp: i64,
}
