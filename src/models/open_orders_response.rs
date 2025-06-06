use crate::enums::{OrderStatus, OrderType, Side, TimeInForce};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersResponse {
    pub account_id: String,
    pub executed_qty: String,
    pub expire_timestamp: i64,
    pub guaranteed_stop_loss: bool,
    pub iceberg_qty: String,
    pub leverage: bool,
    pub margin: f64,
    pub order_id: String,
    pub orig_qty: String,
    pub price: String,
    pub side: Side,
    pub status: OrderStatus,
    pub stop_loss: f64,
    pub symbol: String,
    pub take_profit: f64,
    pub time: f64,
    pub time_in_force: TimeInForce,
    pub trailing_stop_loss: bool,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub update_time: i64,
    pub working: bool,
}
