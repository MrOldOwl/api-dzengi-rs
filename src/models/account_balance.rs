#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub account_id: String,
    pub asset: String,
    pub collateral_currency: bool,
    pub default: bool,
    pub free: f64,
    pub locked: f64,
}
