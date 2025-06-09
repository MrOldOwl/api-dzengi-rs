#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderDtoResponseRes {
    pub account_id: i64,
    pub order_id: String,
    pub quantity: f64,
    pub price: f64,
    pub timestamp: i64,
    pub status: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub expire_time: i64,
    pub time_in_force_type: String,
    pub side: String,
    pub guaranteed_stop_loss: bool,
    pub margin: f64,
    pub take_profit: f64,

    pub exec_price: f64,
    pub exec_quantity: f64,
    pub reject_reason: String,
    pub stop_loss: f64,
    pub symbol: String,
    pub trailing_stop_loss: bool,
}
