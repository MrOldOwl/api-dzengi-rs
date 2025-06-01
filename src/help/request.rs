use crate::errors::DzengiRestClientResult;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

pub trait AutoToJson {
    async fn send_and_json<T: DeserializeOwned>(self) -> DzengiRestClientResult<T>;
}

impl AutoToJson for RequestBuilder {
    async fn send_and_json<T: DeserializeOwned>(self) -> DzengiRestClientResult<T> {
        Ok(self.send().await?.json::<T>().await?)
    }
}
