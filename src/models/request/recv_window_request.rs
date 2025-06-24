use crate::help::Query;
use api_dzengi_rs_macro::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct RecvWindowRequest {
    pub recv_window: Option<u64>,
}
