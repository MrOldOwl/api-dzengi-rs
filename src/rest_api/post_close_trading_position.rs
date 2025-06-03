use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, timestamp_now},
    models::TradingPositionCloseAllResponse,
    switch_url,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloseTradingPositionRequest {
    pub position_id: String,
    pub recv_window: u64,
}
impl CloseTradingPositionRequest {
    pub fn new(position_id: String) -> Self {
        Self {
            position_id,
            recv_window: 5000,
        }
    }

    pub fn with_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }
}

impl DzengiRestClient {
    pub async fn close_trading_position(
        &self,
        request: CloseTradingPositionRequest,
    ) -> DzengiRestClientResult<TradingPositionCloseAllResponse> {
        let settings = self.settings()?;

        let url = switch_url!("/api/v1/closeTradingPosition", self.demo);
        let timestamp = timestamp_now(self.correction_time)?.to_string();
        let recv_window = request.recv_window.to_string();

        let mut params = [
            (DefaultKeys::timestamp(), timestamp),
            (DefaultKeys::recv_window(), recv_window),
            ("positionId", request.position_id),
        ];

        let signature = settings.generate_signature(params.as_mut_slice())?;

        self.client
            .post(url)
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
        crypto::UserSettings,
        rest_api::{CloseTradingPositionRequest, DzengiRestClient},
    };

    #[tokio::test]
    async fn test() {
        let ent_file = read_file(".env").unwrap();
        let api_key = ent_file["API_KEY"].clone();
        let secret = ent_file["SECRET"].clone();

        let mut rest =
            DzengiRestClient::new().with_user_settings(Some(UserSettings::new(api_key, secret)));

        rest.with_correction_time_req().await.unwrap();

        let response = rest
            .close_trading_position(CloseTradingPositionRequest::new("".into()))
            .await;

        match response {
            Err(x) => {
                println!("{x:?}");
                assert!(true)
            }
            _ => assert!(false),
        }
    }
}
