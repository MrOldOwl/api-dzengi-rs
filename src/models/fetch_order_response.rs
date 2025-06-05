use crate::enums::{OrderStatus, OrderType, Side, TimeInForce};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchOrderResponse {
    pub account_id: u64,
    pub order_id: String,
    pub quantity: f64,
    pub price: f64,
    pub timestamp: u128,
    pub status: OrderStatus,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub expire_time: u128,
    pub time_in_force_type: TimeInForce,
    pub side: Side,
    pub guaranteed_stop_loss: bool,
    pub margin: f64,
    pub take_profit: f64,
    pub take_profit_type: String,
    pub stop_loss: f64,
    pub stop_loss_type: String,
}
