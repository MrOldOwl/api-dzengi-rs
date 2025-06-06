use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys, Query},
    models::TradingPositionCloseAllResponse,
    switch_url,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloseTradingPositionRequest {
    pub position_id: String,
    pub recv_window: Option<u64>,
}
impl CloseTradingPositionRequest {
    pub fn new(position_id: String) -> Self {
        Self {
            position_id,
            recv_window: None,
        }
    }

    pub fn with_recv_window(mut self, recv_window: Option<u64>) -> Self {
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

        let mut query = Query::<3>::new();
        query.add(
            DefaultKeys::timestamp(),
            self.correction_time.timestamp_now()?,
        );
        query.add_option(DefaultKeys::recv_window(), request.recv_window);
        query.add("positionId", request.position_id);
        let signature = query.gen_signature(settings)?;

        self.client
            .post(switch_url!("/api/v1/closeTradingPosition", self.demo))
            .header(DefaultKeys::api_key(), settings.api_key.as_str())
            .query(query.as_slice())
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

        let mut rest = DzengiRestClient::new()
            .with_user_settings(Some(UserSettings::new(api_key, secret)))
            .demo_url();

        rest.calc_correction_with_server().await.unwrap();

        //TODO: CREATE POSITION IN DEMO
        let resp = rest
            .close_trading_position(CloseTradingPositionRequest::new("".into()))
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
