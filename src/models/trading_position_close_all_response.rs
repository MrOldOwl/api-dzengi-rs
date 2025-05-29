use super::RequestDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPositionCloseAllResponse {
    pub request: Vec<RequestDto>,
}
