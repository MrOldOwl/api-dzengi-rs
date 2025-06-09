use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct DepositsRequest {
    pub limit: Option<usize>,
    pub start_time: Option<u128>,
    pub end_time: Option<u128>,
    pub recv_window: Option<u64>,
}
