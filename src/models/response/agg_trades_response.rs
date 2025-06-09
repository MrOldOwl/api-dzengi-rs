use super::AggTrades;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggTradesResponse {
    pub agg_trades: Vec<AggTrades>,
}
