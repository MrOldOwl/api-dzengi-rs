use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct DepthRequest {
    pub symbol: String,
    pub limit: Option<usize>,
}
