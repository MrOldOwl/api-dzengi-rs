use crate::{auto_import_models, websocket_api::DzengiWsClient};
use std::ops::{Deref, DerefMut};

auto_import_models! {}

pub struct Version2<'a>(&'a mut DzengiWsClient);

impl<'a> Version2<'a> {
    pub fn new(client: &'a mut DzengiWsClient) -> Self {
        Self(client)
    }
}

impl<'a> Deref for Version2<'a> {
    type Target = DzengiWsClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Version2<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
