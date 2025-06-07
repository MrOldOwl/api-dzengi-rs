use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::EditExchangeOrderResponse,
    switch_url,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, RequestMethods)]
pub struct ChangeOrderRequest {
    pub order_id: String,
    pub recv_window: Option<u64>,
    pub expire_timestamp: Option<i64>,
    pub price: Option<f64>,
}

impl DzengiRestClient {
    pub async fn order_change(
        &self,
        request: ChangeOrderRequest,
    ) -> DzengiRestClientResult<EditExchangeOrderResponse> {
        let settings = self.settings()?;

        let mut query = Query::<5>::new();
        query.add(
            DefaultKeys::timestamp(),
            &self.correction_time.timestamp_now()?,
        );
        request.fill_query(&mut query);

        let signature = query.gen_signature(settings)?;

        self.client
            .put(switch_url!("/api/v1/order", self.demo))
            .header(DefaultKeys::api_key(), settings.api_key.as_str())
            .query(query.as_slice())
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
        rest_api::{ChangeOrderRequest, DzengiRestClient},
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

        //TODO: create order in demo
        let resp = rest
            .order_change(ChangeOrderRequest::new("id".into()))
            .await;

        match resp {
            Err(x) => {
                println!("{x:?}");
                assert!(true)
            }
            _ => assert!(false),
        }
    }
}
