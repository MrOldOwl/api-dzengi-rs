use crate::{auto_import_models, crypto::UserSettings};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::Duration;
use tokio::{
    spawn,
    sync::mpsc::{self, Receiver},
    time::sleep,
};

auto_import_models! {
    v1,
    v2
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DzengiWsRequest {
    pub destination: &'static str,
    pub payload: serde_json::Value,
    #[serde(serialize_with = "serialize_usize_as_str")]
    pub correlationId: usize,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DzengiWsResponse {
    pub status: String,
    pub destination: String,
    pub payload: serde_json::Value,
    #[serde(deserialize_with = "deserialize_usize_from_str")]
    pub correlationId: usize,
}

fn serialize_usize_as_str<S>(value: &usize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

fn deserialize_usize_from_str<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

#[derive(Debug, Default, Clone)]
pub struct DzengiWsClient {
    demo: bool,
    settings: Option<UserSettings>,
    pub(crate) req: DzengiWsRequest,
}

impl DzengiWsClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base_url(mut self) -> Self {
        self.demo = false;
        self
    }

    pub fn demo_url(mut self) -> Self {
        self.demo = true;
        self
    }

    pub fn with_user_settings(mut self, settings: Option<UserSettings>) -> Self {
        self.settings = settings;
        self
    }

    pub fn v1(&mut self) -> Version1 {
        Version1::new(self)
    }

    pub fn v2(&mut self) -> Version2 {
        Version2::new(self)
    }

    async fn ping() -> Receiver<&'static str> {
        let (tx, rx) = mpsc::channel(1);
        spawn(async move {
            loop {
                if let Err(_) = tx.send("{\"op\":\"ping\"}").await {
                    break;
                };
                sleep(Duration::from_secs(20)).await;
            }
        });
        rx
    }
}
