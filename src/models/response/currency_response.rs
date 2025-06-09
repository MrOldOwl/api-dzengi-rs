use super::CurrencyDtoResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyResponse {
    pub currencies: Vec<CurrencyDtoResponse>,
}
