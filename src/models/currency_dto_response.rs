use crate::enums::CurrencyType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyDtoResponse {
    pub commission_fixed: f64,
    pub commission_min: f64,
    pub commission_percent: f64,
    pub display_symbol: String,
    pub max_withdrawal: f64,
    pub min_deposit: f64,
    pub min_withdrawal: f64,
    pub name: String,
    pub precision: i32,
    #[serde(rename = "type")]
    pub currency_type: CurrencyType,
}
