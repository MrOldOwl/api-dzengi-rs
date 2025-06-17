use super::Version2;
use crate::{errors::DzengiRestClientResult, help::AutoToJson, models::ServerTime, switch_url};

impl Version2<'_> {
    pub async fn server_time(&self) -> DzengiRestClientResult<ServerTime> {
        self.client
            .get(switch_url!("/v2/time", self.demo))
            .send_and_json()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::rest_api::DzengiRestClient;

    #[tokio::test]
    async fn test() {
        let rest = DzengiRestClient::new();
        let resp = rest.v2().server_time().await.unwrap();
        println!("{:?}", resp);
    }
}
