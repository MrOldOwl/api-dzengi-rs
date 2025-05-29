#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDtoResponse {
    pub amount: f64,
    pub balance: f64,
    pub blockchain_transaction_hash: Option<String>,
    pub commission: f64,
    pub currency: String,
    pub id: i64,
    pub payment_method: String,
    pub status: String,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub transaction_type: String,
}
