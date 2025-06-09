use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, RequestMethods)]
pub struct ChangeOrderRequest {
    pub order_id: String,
    pub recv_window: Option<u64>,
    pub expire_timestamp: Option<i64>,
    pub price: Option<f64>,
}
