#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTradesResponse {
    pub buyer: bool,
    pub commission: String,
    pub commission_asset: String,
    pub id: String,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub maker: bool,
    pub order_id: String,
    pub price: String,
    pub qty: String,
    pub quote_qty: String,
    pub symbol: String,
    pub time: i64,
}
