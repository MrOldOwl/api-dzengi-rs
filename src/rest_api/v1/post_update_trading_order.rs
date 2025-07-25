use super::Version1;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::{TradingOrderUpdateResponse, UpdateTradingOrderRequest},
    switch_url,
};

impl Version1<'_> {
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
            .post(switch_url!("/v1/updateTradingOrder", self.demo))
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
        crypto::UserSettings, models::UpdateTradingOrderRequest, rest_api::DzengiRestClient,
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
            .v1()
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
