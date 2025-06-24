use crate::help::Query;
use api_dzengi_rs_macro::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, RequestMethods)]
pub struct UpdateTradingOrderRequest {
    pub recv_window: Option<u64>,
    pub expire_timestamp: Option<i64>,
    pub guaranteed_stop_loss: Option<bool>,
    pub new_price: Option<f64>,
    pub order_id: String,
    pub profit_distance: Option<f64>,
    pub stop_distance: Option<f64>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub trailing_stop_loss: Option<bool>,
}
