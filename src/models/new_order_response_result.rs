use crate::enums::{OrderStatus, OrderType, Side, TimeInForce};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResponseResult {
    pub executed_qty: String,
    pub expire_timestamp: i64,
    pub guaranteed_stop_loss: bool,
    pub margin: f64,
    pub order_id: String,
    pub orig_qty: String,
    pub price: String,
    pub profit_distance: f64,
    pub reject_message: String,
    pub side: Side,
    pub status: OrderStatus,
    pub stop_distance: f64,
    pub stop_loss: f64,
    pub symbol: String,
    pub take_profit: f64,
    pub time_in_force: TimeInForce,
    pub trailing_stop_loss: bool,
    pub transact_time: i64,
    #[serde(rename = "type")]
    pub order_type: OrderType,
}
