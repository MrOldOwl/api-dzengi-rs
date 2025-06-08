use super::DzengiRestClient;
use crate::{
    enums::{OrderType, Side},
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::NewOrderResponseResult,
    switch_url,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, RequestMethods)]
pub struct CreateOrderRequest {
    pub symbol: String,
    pub side: Side,
    pub quantity: f64,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub account_id: Option<String>,
    pub expire_timestamp: Option<i64>,
    pub guaranteed_stop_loss: Option<bool>,
    pub leverage: Option<i32>,
    pub price: Option<f64>,
    pub profit_distance: Option<f64>,
    pub recv_window: Option<u64>,
    pub new_order_resp_type: Option<String>,
    pub stop_distance: Option<f64>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub trailing_stop_loss: Option<bool>,
}

impl DzengiRestClient {
    pub async fn order_create(
        &self,
        request: CreateOrderRequest,
    ) -> DzengiRestClientResult<NewOrderResponseResult> {
        let settings = self.settings()?;

        let mut query = Query::<17>::new();
        query.add_item(DefaultKeys::timestamp(&self)?);
        request.fill_query(&mut query);

        let signature = query.gen_signature(settings)?;

        self.client
            .post(switch_url!("/api/v1/order", self.demo))
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
        enums::{OrderType, Side},
        rest_api::{CreateOrderRequest, DzengiRestClient},
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

        let resp = rest
            .order_create(CreateOrderRequest::new(
                "XRP/USD".into(),
                Side::Buy,
                1.0,
                OrderType::Market,
            ))
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
