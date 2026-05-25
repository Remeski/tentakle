use futures::StreamExt;
use serde::Deserialize;
use tokio::{select, sync::mpsc::UnboundedSender};
use tokio_tungstenite::{WebSocketStream, connect_async};

use crate::{
    event::{AppEvent, Event, EventHandler},
    trace_dbg,
};

pub struct NtfyHandler {}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct NtfyMessage {
    pub id: String,
    pub sequence_id: Option<String>,
    pub time: usize,
    pub expires: Option<usize>,
    pub event: String,
    pub topic: String,
    pub message: Option<String>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl NtfyHandler {
    pub async fn new(event_sender: UnboundedSender<Event>) -> Self {
        let (ws_stream, _) = connect_async("wss://ntfy.etremes.net/tentakle/ws")
            .await
            .expect("failed to connect ntfy");
        let (_, mut read) = ws_stream.split();

        tokio::spawn(async move {
            loop {
                select! {
                    _ = event_sender.closed() => {
                        break
                    }
                    socket = read.next() => {
                        let data = socket.unwrap().unwrap();
                        let msg = String::from(data.to_text().expect("not text"));
                        trace_dbg!(&msg);
                        let ntfy = serde_json::from_str::<NtfyMessage>(&msg);
                        if let Ok(ntfy) = ntfy {
                            event_sender.send(Event::App(AppEvent::NtfyMsg(ntfy))).unwrap();
                        }
                    }
                }
            }
        });
        Self {}
    }
}
