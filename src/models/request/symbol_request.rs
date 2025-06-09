use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct SymbolRequest {
    pub symbol: String,
}
