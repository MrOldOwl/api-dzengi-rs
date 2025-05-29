use crate::enums::RequestState;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingOrderUpdateResponse {
    pub request_id: i64,
    pub state: RequestState,
}
