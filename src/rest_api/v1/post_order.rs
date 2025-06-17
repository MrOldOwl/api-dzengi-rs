use super::Version1;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::{CreateOrderRequest, NewOrderResponseResult},
    switch_url,
};

impl Version1<'_> {
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
            .post(switch_url!("/v1/order", self.demo))
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
        models::CreateOrderRequest,
        rest_api::DzengiRestClient,
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
            .v1()
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
