use crate::help::Query;
use api_dzengi_rs_macro::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct SymbolWithRecvRequest {
    pub symbol: String,
    pub recv_window: Option<u64>,
}
