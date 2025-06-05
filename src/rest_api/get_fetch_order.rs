use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys},
    models::FetchOrderResponse,
    switch_url,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FetchOrderRequest {
    pub recv_window: u64,
    pub symbol: String,
    pub order_id: String,
}
impl FetchOrderRequest {
    pub fn new(symbol: String, order_id: String) -> Self {
        Self {
            recv_window: 5000,
            symbol,
            order_id,
        }
    }

    pub fn with_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn with_order_id(mut self, order_id: String) -> Self {
        self.order_id = order_id;
        self
    }
}

impl DzengiRestClient {
    pub async fn fetch_order(
        &self,
        request: FetchOrderRequest,
    ) -> DzengiRestClientResult<FetchOrderResponse> {
        let settings = self.settings()?;

        let url = switch_url!("/api/v1/fetchOrder", self.demo);
        let timestamp = self.correction_time.timestamp_now()?.to_string();
        let recv_window = request.recv_window.to_string();

        let mut params = [
            (DefaultKeys::timestamp(), timestamp),
            (DefaultKeys::recv_window(), recv_window),
            (DefaultKeys::symbol(), request.symbol),
            ("orderId", request.order_id),
        ];

        let signature = settings.generate_signature(params.as_mut_slice())?;

        self.client
            .get(url)
            .header(DefaultKeys::api_key(), settings.api_key.as_str())
            .query(&params)
            .query(&[(DefaultKeys::signature(), signature.as_str())])
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use env_file_reader::read_file;

    use crate::{
        crypto::UserSettings,
        rest_api::{DzengiRestClient, FetchOrderRequest},
    };

    #[tokio::test]
    async fn test() {
        let ent_file = read_file(".env").unwrap();
        let api_key = ent_file["API_KEY"].clone();
        let secret = ent_file["SECRET"].clone();

        let mut rest = DzengiRestClient::new()
            .with_user_settings(Some(UserSettings::new(api_key, secret)))
            .demo_url();

        rest.calc_correction_with_server().await.unwrap();

        //TODO: CREATE ORDER IN DEMO
        let resp = rest
            .fetch_order(FetchOrderRequest::new("BTC/USD_LEVERAGE".into(), "".into()))
            .await
            .unwrap();

        println!("Info: {:?}", resp);
    }
}
