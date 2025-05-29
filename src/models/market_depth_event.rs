use super::MarketDepthData;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDepthEvent {
    pub data: MarketDepthData,
    pub symbol: String,
}
