use crate::help::Query;
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct RecvWindowRequest {
    pub recv_window: Option<u64>,
}
