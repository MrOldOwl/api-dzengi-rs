use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys},
    models::TransactionDtoResponse,
    switch_url,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositsRequest {
    pub limit: Option<usize>,
    pub start_time: Option<u128>,
    pub end_time: Option<u128>,
    pub recv_window: u64,
}
impl DepositsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_limit(mut self, limit: Option<usize>) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_start_time(mut self, start_time: Option<u128>) -> Self {
        self.start_time = start_time;
        self
    }

    pub fn with_end_time(mut self, end_time: Option<u128>) -> Self {
        self.end_time = end_time;
        self
    }

    pub fn with_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }
}

impl DzengiRestClient {
    pub async fn deposits(
        &self,
        request: DepositsRequest,
    ) -> DzengiRestClientResult<Vec<TransactionDtoResponse>> {
        let settings = self.settings()?;

        let url = switch_url!("/api/v1/deposits", self.demo);

        let timestamp = self.correction_time.timestamp_now()?.to_string();
        let recv_window = request.recv_window.to_string();

        let mut count = 2_usize;
        let mut params = [
            (DefaultKeys::timestamp(), timestamp),
            (DefaultKeys::recv_window(), recv_window),
            DefaultKeys::empty_item(),
            DefaultKeys::empty_item(),
            DefaultKeys::empty_item(),
        ];

        if let Some(limit) = request.limit {
            params[count] = ("limit", limit.to_string());
            count += 1;
        }

        if let Some(start_time) = request.start_time {
            params[count] = ("startTime", start_time.to_string());
            count += 1;
        }

        if let Some(end_time) = request.end_time {
            params[count] = ("endTime", end_time.to_string());
            count += 1;
        }

        let signature = settings.generate_signature(&mut params.as_mut_slice()[..count])?;

        self.client
            .get(url)
            .header(DefaultKeys::api_key(), settings.api_key.as_str())
            .query(&params[..count])
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
        rest_api::{DepositsRequest, DzengiRestClient},
    };

    #[tokio::test]
    async fn test() {
        let ent_file = read_file(".env").unwrap();
        let api_key = ent_file["API_KEY"].clone();
        let secret = ent_file["SECRET"].clone();

        let mut rest =
            DzengiRestClient::new().with_user_settings(Some(UserSettings::new(api_key, secret)));

        rest.calc_correction_with_server().await.unwrap();

        let resp = rest
            .deposits(DepositsRequest::new().with_limit(Some(10)))
            .await
            .unwrap();

        println!("{:?}", resp)
    }
}
