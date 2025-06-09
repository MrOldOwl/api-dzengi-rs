use uuid::Uuid;

use crate::enums::{PositionState, PositionType};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDto {
    pub account_id: String,
    pub close_price: f64,
    pub close_quantity: f64,
    pub created_timestamp: i64,
    pub currency: String,
    pub id: Uuid,
    pub instrument_id: i64,
    pub margin: f64,
    pub open_price: f64,
    pub open_quantity: f64,
    pub open_timestamp: i64,
    pub order_id: Uuid,
    pub state: PositionState,
    pub close_timestamp: Option<i64>,
    pub cost: Option<f64>,
    pub current_trailing_price: Option<f64>,
    pub current_trailing_price_updated_timestamp: Option<i64>,
    pub dividend: Option<f64>,
    pub fee: Option<f64>,
    pub guaranteed_stop_loss: Option<bool>,
    pub rpl: Option<f64>,
    pub rpl_converted: Option<f64>,
    pub stop_loss: Option<f64>,
    pub swap: Option<f64>,
    pub swap_converted: Option<f64>,
    pub symbol: Option<String>,
    pub take_profit: Option<f64>,
    pub trailing_quoted_price: Option<f64>,
    pub trailing_stop_loss: Option<bool>,
    pub position_type: Option<PositionType>,
    pub url: Option<String>,
    pub upl_converted: Option<f64>,
}
