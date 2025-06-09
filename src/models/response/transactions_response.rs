use super::TransactionDtoResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsResponse {
    pub transactions: Vec<TransactionDtoResponse>,
}
