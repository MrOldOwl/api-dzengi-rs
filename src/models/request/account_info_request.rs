use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct AccountInfoRequest {
    pub show_zero_balance: Option<bool>,
    pub recv_window: Option<u64>,
}
