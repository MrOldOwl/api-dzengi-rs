use reqwest::Error;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{auto_import_models, crypto::UserSettings};

auto_import_models! {
    get_account_info
}

const BASE_URL: &'static str = "https://api-adapter.dzengi.com/";
const DEMO_URL: &'static str = "https://demo-api-adapter.dzengi.com/";

#[derive(Debug, Default, Clone)]
pub struct DzengiRestClient {
    url: &'static str,
    settings: Option<UserSettings>,
    client: reqwest::Client,
}

impl DzengiRestClient {
    pub fn new() -> Self {
        Self {
            url: BASE_URL,
            ..Default::default()
        }
    }

    pub fn base_url(mut self) -> Self {
        self.url = BASE_URL;
        self
    }

    pub fn demo_url(mut self) -> Self {
        self.url = DEMO_URL;
        self
    }

    pub fn with_user_settings(mut self, settings: Option<UserSettings>) -> Self {
        self.settings = settings;
        self
    }

    async fn request_get<T: DeserializeOwned>(
        &self,
        params: &[(&str, String)],
    ) -> Result<T, Error> {
        self.client
            .get(self.url)
            .query(params)
            .send()
            .await?
            .json::<T>()
            .await
    }
}
