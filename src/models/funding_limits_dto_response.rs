#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingLimitsDtoResponse {
    pub account_currency: String,
    pub min_withdrawal: String,
    pub payment_option: String,
}
