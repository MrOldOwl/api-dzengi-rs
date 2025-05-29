use super::QueryOrderResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersResponse {
    pub open_orders: Vec<QueryOrderResponse>,
}
