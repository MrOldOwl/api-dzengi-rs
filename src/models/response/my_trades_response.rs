#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyTradesResponse {
    pub symbol: String,
    pub order_id: String,
    pub price: String,
    pub qty: String,
    pub commission: String,
    pub commission_asset: String,
    pub time: i64,
    pub maker: bool,
    pub buyer: bool,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub id: Option<String>,
    pub quote_qty: Option<String>,
}
