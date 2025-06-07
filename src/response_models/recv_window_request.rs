use crate::help::Query;
use macr::RequestMethods;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, RequestMethods,
)]
pub struct RecvWindowRequest {
    pub recv_window: Option<u64>,
}
