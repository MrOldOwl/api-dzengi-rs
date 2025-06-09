use super::MyTradesResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllMyTradesResponse {
    pub my_trades: Vec<MyTradesResponse>,
}
