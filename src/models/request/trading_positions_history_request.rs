use crate::help::Query;
use api_dzengi_rs_macro::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct TradingPositionsHistoryRequest {
    pub symbol: String,
    pub limit: Option<usize>,
    pub from: Option<u128>,
    pub to: Option<u128>,
    pub recv_window: Option<u64>,
}
