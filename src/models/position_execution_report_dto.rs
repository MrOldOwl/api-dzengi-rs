use crate::enums::{ExecutionSource, ExecutionStatus, ExecutionType, PositionState, RejectReason};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionExecutionReportDto {
    pub account_currency: String,
    pub account_id: i64,
    pub created_timestamp: i64,
    pub currency: String,
    pub exec_id: String,
    pub exec_timestamp: i64,
    pub instrument_id: i64,
    pub position_id: String,
    pub source: ExecutionSource,
    pub status: ExecutionStatus,
    pub execution_type: Option<ExecutionType>,
    pub fee_details: Option<f64>,
    pub fx_rate: Option<f64>,
    pub gsl: Option<bool>,
    pub price: Option<f64>,
    pub quantity: Option<f64>,
    pub reject_reason: Option<RejectReason>,
    pub rpl: Option<f64>,
    pub rpl_converted: Option<f64>,
    pub state: Option<PositionState>,
    pub stop_loss: Option<f64>,
    pub swap: Option<f64>,
    pub swap_converted: Option<f64>,
    pub symbol: Option<String>,
    pub take_profit: Option<f64>,
    pub trailing_stop_loss: Option<bool>,
}
