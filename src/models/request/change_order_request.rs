use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, RequestMethods)]
pub struct ChangeOrderRequest {
    pub order_id: String,
    pub price: Option<f64>,
    pub expire_timestamp: Option<i64>,
    pub recv_window: Option<u64>,
}
