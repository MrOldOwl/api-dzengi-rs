use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct MyTradesRequest {
    pub symbol: String,
    pub recv_window: Option<u64>,
    pub limit: Option<usize>,
    pub start_time: Option<u128>,
    pub end_time: Option<u128>,
}
