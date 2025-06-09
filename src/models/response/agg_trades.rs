#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggTrades {
    #[serde(rename = "T")]
    pub trade_time: i64,
    #[serde(rename = "a")]
    pub agg_trade_id: i64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
}
