use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct OpenOrdersRequest {
    pub symbol: Option<String>,
    pub recv_window: Option<u64>,
}
