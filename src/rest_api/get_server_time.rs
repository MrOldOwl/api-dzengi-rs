use super::DzengiRestClient;
use crate::{errors::DzengiRestClientResult, help::AutoToJson, models::ServerTime, switch_url};

impl DzengiRestClient {
    pub async fn server_time(&self) -> DzengiRestClientResult<ServerTime> {
        self.client
            .get(switch_url!("/api/v2/time", self.demo))
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
        let resp = rest.server_time().await.unwrap();
        println!("{:?}", resp);
    }
}
