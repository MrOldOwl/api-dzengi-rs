#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDtoResponseRes {
    pub account_id: i64,
    pub exec_price: f64,
    pub exec_quantity: f64,
    pub expire_time: i64,
    pub guaranteed_stop_loss: bool,
    pub margin: f64,
    pub order_id: String,
    pub price: f64,
    pub quantity: f64,
    pub reject_reason: String,
    pub side: String,
    pub status: String,
    pub stop_loss: f64,
    pub symbol: String,
    pub take_profit: f64,
    pub time_in_force_type: String,
    pub timestamp: i64,
    pub trailing_stop_loss: bool,
    #[serde(rename = "type")]
    pub order_type: String,
}
