use crate::enums::CurrencyType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyDtoResponse {
    pub name: String,
    pub display_symbol: String,
    pub precision: i32,
    #[serde(rename = "type")]
    pub currency_type: CurrencyType,

    pub commission_fixed: Option<f64>,
    pub commission_min: Option<f64>,
    pub commission_percent: Option<f64>,
    pub max_withdrawal: Option<f64>,
    pub min_deposit: Option<f64>,
    pub min_withdrawal: Option<f64>,
}
