use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct AggTradesRequest {
    pub symbol: String,
    pub limit: Option<usize>,
    pub start_time: Option<u128>,
    pub end_time: Option<u128>,
}
