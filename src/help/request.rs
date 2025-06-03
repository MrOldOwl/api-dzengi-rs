use crate::errors::{DzengiRestClientError, DzengiRestClientResult};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

pub trait AutoToJson {
    async fn send_and_json<T: DeserializeOwned>(self) -> DzengiRestClientResult<T>;
}

impl AutoToJson for RequestBuilder {
    async fn send_and_json<T: DeserializeOwned>(self) -> DzengiRestClientResult<T> {
        let response = self.send().await?;

        if !response.status().is_success() {
            let text = response.text().await?;
            return match serde_json::de::from_str(&text) {
                Ok(x) => Err(DzengiRestClientError::DzengiCorrect(x)),
                _ => Err(DzengiRestClientError::DzengiUncorrected(text)),
            };
        }

        Ok(response.json::<T>().await?)
    }
}
