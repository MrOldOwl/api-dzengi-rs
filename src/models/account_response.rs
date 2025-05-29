use super::AccountBalance;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResponse {
    pub affiliated: String,
    pub balances: Vec<AccountBalance>,
    pub buyer_commission: f64,
    pub can_deposit: bool,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub maker_commission: f64,
    pub seller_commission: f64,
    pub taker_commission: f64,
    pub update_time: i64,
    pub user_id: i64,
}
