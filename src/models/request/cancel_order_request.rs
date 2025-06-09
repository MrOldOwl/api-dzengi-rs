use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, RequestMethods)]
pub struct CancelOrderRequest {
    pub symbol: String,
    pub order_id: String,
    pub recv_window: Option<u64>,
}
