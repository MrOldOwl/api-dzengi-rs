use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct CloseTradingPositionRequest {
    pub position_id: String,
    pub recv_window: Option<u64>,
}
