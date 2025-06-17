use crate::{
    errors::DzengiRestClientResult,
    models::{KlinesRequest, KlinesResponse},
    switch_wss,
    websocket_api::{DzengiWsClient, DzengiWsResponse, Version1},
};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::Message};

impl Version1<'_> {
    pub async fn klines(
        &mut self,
        request: KlinesRequest,
        func: impl Fn(Vec<KlinesResponse>),
    ) -> DzengiRestClientResult<()> {
        self.req.destination = "/api/v1/klines";
        self.req.payload = serde_json::to_value(&request)?;
        let id = self.req.correlationId + 1;
        self.req.correlationId = id;

        let (ws_stream, _) = connect_async(switch_wss!(self.demo)).await?;
        let (mut write, mut read) = ws_stream.split();

        let json = serde_json::to_string(&self.req)?;
        write.send(Message::Text(json.into())).await?;

        let mut rx = DzengiWsClient::ping().await;

        loop {
            if let Ok(ping) = rx.try_recv() {
                write.send(Message::Ping(ping.into())).await?;
            }

            if let Some(result) = read.next().await {
                match result {
                    Ok(msg) => match msg {
                        Message::Text(text) => {
                            println!("{:?}", text);
                            println!("");
                            let resp: DzengiWsResponse = serde_json::from_str(text.as_str())?;
                            println!("{:?}", resp);
                            println!("");
                            if resp.correlationId == id {
                                #[derive(Deserialize)]
                                pub struct Lines {
                                    pub lines: Vec<KlinesResponse>,
                                }

                                let array: Lines = serde_json::from_value(resp.payload)?;
                                func(array.lines);
                            }
                        }
                        Message::Ping(_) => {
                            write
                                .send(Message::Pong("{\"op\":\"pong\"}".into()))
                                .await?
                        }
                        Message::Close(_) => return Ok(()),
                        _ => {}
                    },
                    Err(x) => return Err(x.into()),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{enums::Interval, websocket_api::DzengiWsClient};
    use std::time::Duration;
    use tokio::spawn;

    #[tokio::test]
    async fn test() {
        let mut ws = DzengiWsClient::new();

        let task1 = spawn(async move {
            let res = ws
                .v1()
                .klines(
                    KlinesRequest::new("BTC/USD".into(), Interval::OneMinute).with_limit(Some(4)),
                    |x| println!("{x:?}"),
                )
                .await;

            println!("Span result: {:?}", res);
        });

        let task2 = tokio::time::sleep(Duration::from_secs(60));

        tokio::select! {
            res1 = task1 => match res1 {
                Ok(_) => println!("Первая задача завершена"),
                Err(e) => println!("Ошибка в первой задаче: {}", e),
            },
            _ = task2 => println!("Таймаут"),
        }
    }
}
