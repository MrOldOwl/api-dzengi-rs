use crate::{
    crypto::UserSettings,
    errors::{CryptoError, DzengiRestClientError, DzengiRestClientResult},
};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use zeroize::Zeroizing;

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

pub struct Query<const N: usize> {
    counter: usize,
    slice: [(&'static str, String); N],
}

impl<const N: usize> Query<N> {
    pub fn new() -> Self {
        Self {
            counter: 0,
            slice: std::array::from_fn(|_| ("", String::default())),
        }
    }

    pub fn add<T: ToString>(&mut self, key: &'static str, value: T) {
        self.slice[self.counter] = (key, value.to_string());
        self.counter += 1;
    }

    pub fn add_option<T: ToString>(&mut self, key: &'static str, value: Option<T>) {
        if let Some(value) = value {
            self.slice[self.counter] = (key, value.to_string());
            self.counter += 1;
        }
    }

    pub fn as_slice(&self) -> &[(&'static str, String)] {
        &self.slice[..self.counter]
    }

    pub fn gen_signature(
        &mut self,
        settings: &UserSettings,
    ) -> Result<Zeroizing<String>, CryptoError> {
        settings.generate_signature(&mut self.slice[..self.counter])
    }
}
