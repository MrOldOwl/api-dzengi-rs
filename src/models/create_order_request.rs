#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
    pub account_id: Option<i64>,
    #[validate(length(min = 1))]
    pub api_key: String,
    pub expire_timestamp: Option<i64>,
    pub guaranteed_stop_loss: Option<bool>,
    pub leverage: Option<i32>,
    pub new_order_resp_type: Option<String>,
    pub price: Option<f64>,
    pub profit_distance: Option<f64>,
    #[validate(range(min = 0.0))]
    pub quantity: f64,
    #[validate(range(max = 60000))]
    pub recv_window: Option<i64>,
    #[validate(length(min = 1))]
    pub side: String,
    #[validate(length(min = 1))]
    pub signature: String,
    pub stop_distance: Option<f64>,
    pub stop_loss: Option<f64>,
    #[validate(length(min = 1))]
    pub symbol: String,
    pub take_profit: Option<f64>,
    pub timestamp: i64,
    pub trailing_stop_loss: Option<bool>,
    #[serde(rename = "type")]
    #[validate(length(min = 1))]
    pub order_type: String,
}
