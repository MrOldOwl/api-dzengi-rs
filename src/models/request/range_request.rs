use crate::help::Query;
use api_dzengi_rs_macro::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct RangeRequest {
    pub limit: Option<usize>,
    pub start_time: Option<u128>,
    pub end_time: Option<u128>,
    pub recv_window: Option<u64>,
}
