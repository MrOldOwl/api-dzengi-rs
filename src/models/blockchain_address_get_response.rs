#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainAddressGetResponse {
    pub address: String,
    pub address_legacy: Option<String>,
    pub destination_tag: Option<String>,
}
