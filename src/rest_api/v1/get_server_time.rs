use super::Version1;
use crate::{errors::DzengiRestClientResult, help::AutoToJson, models::ServerTime, switch_url};

impl Version1<'_> {
    pub async fn server_time(&self) -> DzengiRestClientResult<ServerTime> {
        self.client
            .get(switch_url!("/v1/time", self.demo))
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
        let resp = rest.v1().server_time().await.unwrap();
        println!("{:?}", resp);
    }
}
