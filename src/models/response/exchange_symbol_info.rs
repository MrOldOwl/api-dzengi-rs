use super::SymbolFilter;
use crate::enums::{AssetType, MarketModes, MarketType, OrderType, SymbolStatus};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeSymbolInfo {
    pub symbol: String,
    pub name: String,
    pub status: SymbolStatus,
    pub base_asset: String,
    pub base_asset_precision: i32,
    pub quote_asset: String,
    pub quote_asset_id: String,
    pub quote_precision: i32,
    pub order_types: Vec<OrderType>,
    pub filters: Vec<SymbolFilter>,
    pub market_modes: Vec<MarketModes>,
    pub market_type: MarketType,
    pub country: Option<String>,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub trading_hours: String,
    pub tick_size: f64,
    pub tick_value: Option<f64>,
    pub exchange_fee: Option<f64>,

    pub asset_type: Option<AssetType>,
    pub long_rate: Option<f64>,
    pub make_fee: Option<f64>,
    pub max_sl_gap: Option<f64>,
    pub max_tp_gap: Option<f64>,
    pub min_sl_gap: Option<f64>,
    pub min_tp_gap: Option<f64>,
    pub short_rate: Option<f64>,
    pub swap_charge_interval: Option<i64>,
    pub take_fee: Option<f64>,
    pub trading_fee: Option<f64>,
}
