use super::PositionDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPositionListResponse {
    pub positions: Vec<PositionDto>,
}
