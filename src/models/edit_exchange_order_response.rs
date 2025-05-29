use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EditExchangeOrderResponse {
    #[serde(rename = "orderId")]
    pub order_id: Uuid,
}
