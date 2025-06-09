use crate::{
    enums::{OrderType, Side},
    help::Query,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, RequestMethods)]
pub struct CreateOrderRequest {
    pub symbol: String,
    pub side: Side,
    pub quantity: f64,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub account_id: Option<String>,
    pub expire_timestamp: Option<i64>,
    pub guaranteed_stop_loss: Option<bool>,
    pub leverage: Option<i32>,
    pub price: Option<f64>,
    pub profit_distance: Option<f64>,
    pub recv_window: Option<u64>,
    pub new_order_resp_type: Option<String>,
    pub stop_distance: Option<f64>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub trailing_stop_loss: Option<bool>,
}
