use super::DzengiRestClient;
use crate::{
    errors::DzengiRestClientResult,
    help::{AutoToJson, DefaultKeys},
    models::AccountResponse,
    switch_url,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountInfoRequest {
    pub show_zero_balance: bool,
    pub recv_window: u64,
}
impl Default for AccountInfoRequest {
    fn default() -> Self {
        Self {
            show_zero_balance: false,
            recv_window: 5000,
        }
    }
}
impl AccountInfoRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_show_zero(mut self, zero_balance: bool) -> Self {
        self.show_zero_balance = zero_balance;
        self
    }

    pub fn with_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }
}

impl DzengiRestClient {
    pub async fn account_info(
        &self,
        request: AccountInfoRequest,
    ) -> DzengiRestClientResult<AccountResponse> {
        let settings = self.settings()?;

        let url = switch_url!("/api/v1/account", self.demo);
        let timestamp = self.correction_time.timestamp_now()?.to_string();
        let show_zero_balance = request.show_zero_balance.to_string();
        let recv_window = request.recv_window.to_string();

        let mut params = [
            (DefaultKeys::timestamp(), timestamp),
            (DefaultKeys::recv_window(), recv_window),
            ("showZeroBalance", show_zero_balance),
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
        crypto::UserSettings,
        rest_api::{AccountInfoRequest, DzengiRestClient},
    };

    #[tokio::test]
    async fn test() {
        let ent_file = read_file(".env").unwrap();
        let api_key = ent_file["API_KEY"].clone();
        let secret = ent_file["SECRET"].clone();

        let mut rest =
            DzengiRestClient::new().with_user_settings(Some(UserSettings::new(api_key, secret)));

        rest.calc_correction_with_server().await.unwrap();

        let resp = rest.account_info(AccountInfoRequest::new()).await.unwrap();
        println!("Info: {:?}", resp);
    }
}
