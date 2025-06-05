use crate::enums::RequestState;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDtoResponse {
    pub id: i64,
    pub balance: f64,
    pub amount: f64,
    pub currency: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub timestamp: u128,
    pub commission: f64,
    pub status: RequestState,
    pub blockchain_transaction_hash: Option<String>,
    pub payment_method: Option<String>,
}
