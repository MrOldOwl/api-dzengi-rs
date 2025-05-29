use crate::enums::{RejectReason, RequestState, RequestType};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestDto {
    pub account_id: String,
    pub created_timestamp: i64,
    pub id: i64,
    pub order_id: Option<String>,
    pub position_id: Option<String>,
    pub reject_reason: Option<RejectReason>,
    pub rq_type: RequestType,
    pub state: RequestState,
}
