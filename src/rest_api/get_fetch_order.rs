use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::FetchOrderResponse,
    switch_url,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct FetchOrderRequest {
    pub symbol: String,
    pub order_id: String,
    pub recv_window: Option<u64>,
}

impl DzengiRestClient {
    pub async fn fetch_order(
        &self,
        request: FetchOrderRequest,
    ) -> DzengiRestClientResult<FetchOrderResponse> {
        let settings = self.settings()?;

        let mut query = Query::<4>::new();
        query.add_item(DefaultKeys::timestamp(&self)?);
        request.fill_query(&mut query);
        let signature = query.gen_signature(&settings)?;

        self.client
            .get(switch_url!("/api/v1/fetchOrder", self.demo))
            .header(DefaultKeys::api_key(), settings.api_key.as_str())
            .query(&query.as_slice())
            .query(&DefaultKeys::signature(&signature))
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

        println!("{:?}", resp);
    }
}
