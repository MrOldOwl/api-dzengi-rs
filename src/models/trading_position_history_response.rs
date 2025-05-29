use super::PositionExecutionReportDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPositionHistoryResponse {
    pub history: Vec<PositionExecutionReportDto>,
}
