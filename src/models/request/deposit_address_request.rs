use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct DepositAddressRequest {
    pub coin: String,
    pub recv_window: Option<u64>,
}
