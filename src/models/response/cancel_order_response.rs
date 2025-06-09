use crate::enums::{OrderStatus, OrderType, Side, TimeInForce};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub executed_qty: String,
    pub order_id: String,
    pub orig_qty: String,
    pub price: String,
    pub side: Side,
    pub status: OrderStatus,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
}
