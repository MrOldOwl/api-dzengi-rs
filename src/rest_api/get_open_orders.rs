use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::OpenOrdersResponse,
    switch_url,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, RequestMethods)]
pub struct OpenOrdersRequest {
    pub symbol: Option<String>,
    pub recv_window: Option<u64>,
}

impl DzengiRestClient {
    pub async fn open_orders(
        &self,
        request: OpenOrdersRequest,
    ) -> DzengiRestClientResult<Vec<OpenOrdersResponse>> {
        let settings = self.settings()?;

        let mut query = Query::<3>::new();
        query.add_item(DefaultKeys::timestamp(&self)?);
        request.fill_query(&mut query);
        let signature = query.gen_signature(&settings)?;

        self.client
            .get(switch_url!("/api/v1/openOrders", self.demo))
            .header(DefaultKeys::api_key(), settings.api_key.as_str())
            .query(query.as_slice())
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
        rest_api::{DzengiRestClient, OpenOrdersRequest},
    };

    #[tokio::test]
    async fn test() {
        let ent_file = read_file(".env").unwrap();
        let api_key = ent_file["API_KEY"].clone();
        let secret = ent_file["SECRET"].clone();

        let mut rest =
            DzengiRestClient::new().with_user_settings(Some(UserSettings::new(api_key, secret)));

        rest.calc_correction_with_server().await.unwrap();

        let resp = rest.open_orders(OpenOrdersRequest::new()).await.unwrap();

        println!("{:?}", resp)
    }
}
