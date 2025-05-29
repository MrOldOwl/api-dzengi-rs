use super::SymbolFilter;
use crate::enums::{AssetType, MarketModes, MarketType, OrderType, SymbolStatus};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeSymbolInfo {
    pub asset_type: AssetType,
    pub base_asset: String,
    pub base_asset_precision: i32,
    pub country: Option<String>,
    pub exchange_fee: f64,
    pub filters: Vec<SymbolFilter>,
    pub industry: Option<String>,
    pub long_rate: f64,
    pub make_fee: f64,
    pub market_modes: Vec<MarketModes>,
    pub market_type: MarketType,
    pub max_sl_gap: f64,
    pub max_tp_gap: f64,
    pub min_sl_gap: f64,
    pub min_tp_gap: f64,
    pub name: String,
    pub order_types: Vec<OrderType>,
    pub quote_asset: String,
    pub quote_asset_id: String,
    pub quote_precision: i32,
    pub sector: Option<String>,
    pub short_rate: f64,
    pub status: SymbolStatus,
    pub swap_charge_interval: i64,
    pub symbol: String,
    pub take_fee: f64,
    pub tick_size: f64,
    pub tick_value: f64,
    pub trading_fee: f64,
    pub trading_hours: String,
}
