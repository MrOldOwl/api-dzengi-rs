use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::CurrencyDtoResponse,
    response_models::RecvWindowRequest,
    switch_url,
};

impl DzengiRestClient {
    pub async fn currencies(
        &self,
        request: RecvWindowRequest,
    ) -> DzengiRestClientResult<Vec<CurrencyDtoResponse>> {
        let settings = self.settings()?;

        let mut query = Query::<2>::new();
        query.add(
            DefaultKeys::timestamp(),
            self.correction_time.timestamp_now()?,
        );
        request.fill_query(&mut query);
        let signature = query.gen_signature(&settings)?;

        self.client
            .get(switch_url!("/api/v1/currencies", self.demo))
            .header(DefaultKeys::api_key(), settings.api_key.as_str())
            .query(&query.as_slice())
            .query(&[(DefaultKeys::signature(), signature.as_str())])
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use env_file_reader::read_file;

    use crate::{
        crypto::UserSettings, response_models::RecvWindowRequest, rest_api::DzengiRestClient,
    };

    #[tokio::test]
    async fn test() {
        let ent_file = read_file(".env").unwrap();
        let api_key = ent_file["API_KEY"].clone();
        let secret = ent_file["SECRET"].clone();

        let mut rest =
            DzengiRestClient::new().with_user_settings(Some(UserSettings::new(api_key, secret)));

        rest.calc_correction_with_server().await.unwrap();

        let resp = rest.currencies(RecvWindowRequest::new()).await.unwrap();
        println!("Currencies: {:?}", &resp[0..10]);
    }
}
