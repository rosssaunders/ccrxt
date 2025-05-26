use futures::SinkExt;
use serde_json::Value;
use std::error::Error;
use tokio_tungstenite::tungstenite::Message;

pub struct BaseWebSocket {
    pub(crate) url: String,
    pub(crate) ws_stream: Option<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
}

impl BaseWebSocket {
    pub(crate) async fn send_message(
        &mut self,
        message: Value,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ws) = self.ws_stream.as_mut() {
            ws.send(Message::Text(message.to_string().into())).await?;
        }
        Ok(())
    }
}
