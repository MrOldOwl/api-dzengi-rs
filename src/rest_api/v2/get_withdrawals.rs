use super::Version2;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::{RangeRequest, TransactionDtoResponse},
    switch_url,
};

impl Version2<'_> {
    pub async fn withdrawals(
        &self,
        request: RangeRequest,
    ) -> DzengiRestClientResult<Vec<TransactionDtoResponse>> {
        let settings = self.settings()?;

        let mut query = Query::<5>::new();
        query.add_item(DefaultKeys::timestamp(&self)?);
        request.fill_query(&mut query);
        let signature = query.gen_signature(&settings)?;

        self.client
            .get(switch_url!("/v2/withdrawals", self.demo))
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

    use crate::{crypto::UserSettings, models::RangeRequest, rest_api::DzengiRestClient};

    #[tokio::test]
    async fn test() {
        let ent_file = read_file(".env").unwrap();
        let api_key = ent_file["API_KEY"].clone();
        let secret = ent_file["SECRET"].clone();

        let mut rest =
            DzengiRestClient::new().with_user_settings(Some(UserSettings::new(api_key, secret)));

        rest.calc_correction_with_server().await.unwrap();

        let resp = rest
            .v2()
            .withdrawals(RangeRequest::new().with_limit(Some(10)))
            .await
            .unwrap();

        println!("{:?}", resp)
    }
}
