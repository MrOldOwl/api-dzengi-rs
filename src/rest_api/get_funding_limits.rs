use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys},
    models::FundingLimitsDtoResponse,
    response_models::RecvWindowRequest,
    switch_url,
};

impl DzengiRestClient {
    pub async fn funding_limits(
        &self,
        request: RecvWindowRequest,
    ) -> DzengiRestClientResult<Vec<FundingLimitsDtoResponse>> {
        let settings = self.settings()?;

        let url = switch_url!("/api/v1/fundingLimits", self.demo);
        let timestamp = self.correction_time.timestamp_now()?.to_string();
        let recv_window = request.recv_window.to_string();

        let mut params = [
            (DefaultKeys::timestamp(), timestamp),
            (DefaultKeys::recv_window(), recv_window),
        ];

        let signature = settings.generate_signature(params.as_mut_slice())?;

        self.client
            .get(url)
            .header(DefaultKeys::api_key(), settings.api_key.as_str())
            .query(&params)
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

        let resp = rest.funding_limits(RecvWindowRequest::new()).await.unwrap();
        println!("{:?}", &resp[0..10]);
    }
}
