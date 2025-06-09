use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::TradingOrderUpdateResponse,
    switch_url,
};
use macr::RequestMethods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, RequestMethods)]
pub struct UpdateTradingOrderRequest {
    pub recv_window: Option<u64>,
    pub expire_timestamp: Option<i64>,
    pub guaranteed_stop_loss: Option<bool>,
    pub new_price: Option<f64>,
    pub order_id: String,
    pub profit_distance: Option<f64>,
    pub stop_distance: Option<f64>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub trailing_stop_loss: Option<bool>,
}

impl DzengiRestClient {
    pub async fn update_trading_order(
        &self,
        request: UpdateTradingOrderRequest,
    ) -> DzengiRestClientResult<TradingOrderUpdateResponse> {
        let settings = self.settings()?;

        let mut query = Query::<11>::new();
        query.add_item(DefaultKeys::timestamp(&self)?);
        request.fill_query(&mut query);

        let signature = query.gen_signature(settings)?;

        self.client
            .post(switch_url!("/api/v1/updateTradingOrder", self.demo))
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
        rest_api::{DzengiRestClient, UpdateTradingOrderRequest},
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

        //TODO: create order
        let resp = rest
            .update_trading_order(UpdateTradingOrderRequest::new("id".into()))
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
