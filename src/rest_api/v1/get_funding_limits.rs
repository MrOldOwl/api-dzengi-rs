use super::Version1;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::{FundingLimitsDtoResponse, RecvWindowRequest},
    switch_url,
};

impl Version1<'_> {
    pub async fn funding_limits(
        &self,
        request: RecvWindowRequest,
    ) -> DzengiRestClientResult<Vec<FundingLimitsDtoResponse>> {
        let settings = self.settings()?;

        let mut query = Query::<2>::new();
        query.add_item(DefaultKeys::timestamp(&self)?);
        request.fill_query(&mut query);
        let signature = query.gen_signature(&settings)?;

        self.client
            .get(switch_url!("/v1/fundingLimits", self.demo))
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

    use crate::{crypto::UserSettings, models::RecvWindowRequest, rest_api::DzengiRestClient};

    #[tokio::test]
    async fn test() {
        let ent_file = read_file(".env").unwrap();
        let api_key = ent_file["API_KEY"].clone();
        let secret = ent_file["SECRET"].clone();

        let mut rest =
            DzengiRestClient::new().with_user_settings(Some(UserSettings::new(api_key, secret)));

        rest.calc_correction_with_server().await.unwrap();

        let resp = rest
            .v1()
            .funding_limits(RecvWindowRequest::new())
            .await
            .unwrap();
        println!("{:?}", &resp[..10]);
    }
}
