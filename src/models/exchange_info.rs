use super::{ExchangeFilter, ExchangeSymbolInfo, RateLimits};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    pub exchange_filters: Vec<ExchangeFilter>,
    pub rate_limits: Vec<RateLimits>,
    pub server_time: i64,
    pub symbols: Vec<ExchangeSymbolInfo>,
    pub timezone: String,
}
