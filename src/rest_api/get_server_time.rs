use super::DzengiRestClient;
use crate::{errors::DzengiRestClientResult, help::AutoToJson, models::ServerTime, switch_url};

impl DzengiRestClient {
    pub async fn server_time(&self) -> DzengiRestClientResult<ServerTime> {
        let url = switch_url!("/api/v2/time", self.demo);
        self.client.get(url).send_and_json().await
    }
}

#[cfg(test)]
mod test {
    use crate::rest_api::DzengiRestClient;

    #[tokio::test]
    async fn test() {
        let rest = DzengiRestClient::new();
        let time = rest.server_time().await.unwrap();
        println!("{:?}", time);
    }
}
