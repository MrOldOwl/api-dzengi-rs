use super::DzengiRestClient;
use crate::{errors::DzengiRestClientResult, models::ServerTime, switch_url};

impl DzengiRestClient {
    pub async fn server_time(&self) -> DzengiRestClientResult<ServerTime> {
        let url = switch_url!("/api/v2/time", self.demo);
        let time: ServerTime = self.get(url, &[]).await?;
        Ok(time)
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
