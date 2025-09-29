use std::fmt;

use crate::client::{IncomingMessage, WebSocketClient, WebSocketResult};

/// Connection lifecycle states. Host code decides reconnection policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Authenticating,
    Authenticated,
    Error(String),
}

impl fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionState::Disconnected => write!(f, "Disconnected"),
            ConnectionState::Connecting => write!(f, "Connecting"),
            ConnectionState::Connected => write!(f, "Connected"),
            ConnectionState::Authenticating => write!(f, "Authenticating"),
            ConnectionState::Authenticated => write!(f, "Authenticated"),
            ConnectionState::Error(e) => write!(f, "Error({e})"),
        }
    }
}

/// Events emitted by a connection wrapper to inform host code.
#[derive(Debug, Clone)]
pub enum WebSocketEvent {
    Connected,
    Disconnected,
    Binary(bytes::Bytes),
    Text(String),
    Error(String),
}

/// Thin wrapper over a `WebSocketClient` that tracks state and emits events.
pub struct ConnectionController<C: WebSocketClient> {
    client: C,
    state: ConnectionState,
}

impl<C: WebSocketClient> ConnectionController<C> {
    pub fn new(client: C) -> Self {
        Self {
            client,
            state: ConnectionState::Disconnected,
        }
    }

    pub fn state(&self) -> ConnectionState {
        self.state.clone()
    }

    pub async fn connect(&mut self, url: &str) -> WebSocketResult<()> {
        self.state = ConnectionState::Connecting;
        match self.client.connect(url).await {
            Ok(()) => {
                self.state = ConnectionState::Connected;
                Ok(())
            }
            Err(e) => {
                self.state = ConnectionState::Error(e.to_string());
                Err(e)
            }
        }
    }

    pub async fn disconnect(&mut self) -> WebSocketResult<()> {
        let res = self.client.disconnect().await;
        self.state = ConnectionState::Disconnected;
        res
    }

    pub async fn send(&mut self, msg: bytes::Bytes) -> WebSocketResult<()> {
        self.client.send(msg).await
    }

    /// Poll the underlying client for the next message and translate into events.
    pub async fn next_event(&mut self) -> WebSocketResult<Option<WebSocketEvent>> {
        if !self.client.is_connected() {
            if matches!(
                self.state,
                ConnectionState::Connected | ConnectionState::Connecting
            ) {
                self.state = ConnectionState::Disconnected;
                return Ok(Some(WebSocketEvent::Disconnected));
            }
            return Ok(None);
        }

        match self.client.receive().await {
            Ok(Some(IncomingMessage::Binary(b))) => Ok(Some(WebSocketEvent::Binary(b))),
            Ok(Some(IncomingMessage::Text(s))) => Ok(Some(WebSocketEvent::Text(s))),
            Ok(None) => Ok(None),
            Err(e) => {
                self.state = ConnectionState::Error(e.to_string());
                Ok(Some(WebSocketEvent::Error(e.to_string())))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use parking_lot::Mutex;

    use super::*;
    use crate::client::{IncomingMessage, WebSocketError};

    #[derive(Default)]
    struct MockClient {
        connected: Mutex<bool>,
        next_connect_result: Mutex<Option<WebSocketResult<()>>>,
        incoming: Mutex<Vec<WebSocketResult<Option<IncomingMessage>>>>,
        sent: Mutex<Vec<Bytes>>,
    }

    impl MockClient {
        fn new() -> Self {
            Self::default()
        }

        fn with_incoming(self, items: Vec<WebSocketResult<Option<IncomingMessage>>>) -> Self {
            *self.incoming.lock() = items;
            self
        }

        fn set_connected(&self, v: bool) {
            *self.connected.lock() = v;
        }
        fn set_connect_result(&self, r: WebSocketResult<()>) {
            *self.next_connect_result.lock() = Some(r);
        }
    }

    #[async_trait::async_trait]
    impl WebSocketClient for MockClient {
        async fn connect(&mut self, _url: &str) -> WebSocketResult<()> {
            if let Some(r) = self.next_connect_result.lock().take() {
                r
            } else {
                *self.connected.lock() = true;
                Ok(())
            }
        }

        async fn disconnect(&mut self) -> WebSocketResult<()> {
            *self.connected.lock() = false;
            Ok(())
        }

        async fn send(&mut self, message: Bytes) -> WebSocketResult<()> {
            if !*self.connected.lock() {
                return Err(WebSocketError::NotConnected);
            }
            self.sent.lock().push(message);
            Ok(())
        }

        async fn receive(&mut self) -> WebSocketResult<Option<IncomingMessage>> {
            if !*self.connected.lock() {
                return Err(WebSocketError::NotConnected);
            }
            self.incoming.lock().pop().unwrap_or(Ok(None))
        }

        fn is_connected(&self) -> bool {
            *self.connected.lock()
        }
    }

    #[test]
    fn connection_state_display() {
        assert_eq!(ConnectionState::Disconnected.to_string(), "Disconnected");
        assert_eq!(ConnectionState::Connecting.to_string(), "Connecting");
        assert_eq!(ConnectionState::Connected.to_string(), "Connected");
        assert_eq!(
            ConnectionState::Authenticating.to_string(),
            "Authenticating"
        );
        assert_eq!(ConnectionState::Authenticated.to_string(), "Authenticated");
        assert_eq!(ConnectionState::Error("x".into()).to_string(), "Error(x)");
    }

    #[tokio::test]
    async fn connect_success_and_receive_binary() {
        let mock = MockClient::new().with_incoming(vec![Ok(Some(IncomingMessage::Binary(
            Bytes::from_static(b"abc"),
        )))]);
        let mut ctl = ConnectionController::new(mock);
        ctl.connect("wss://unit").await.expect("connect ok");
        assert_eq!(ctl.state(), ConnectionState::Connected);

        match ctl.next_event().await.expect("event").expect("some") {
            WebSocketEvent::Binary(b) => assert_eq!(b, Bytes::from_static(b"abc")),
            other => panic!("unexpected event: {:?}", other),
        }
    }

    #[tokio::test]
    async fn receive_text_maps_to_event() {
        let mut ctl = ConnectionController::new(
            MockClient::default().with_incoming(vec![Ok(Some(IncomingMessage::Text("hi".into())))]),
        );
        ctl.connect("wss://unit").await.unwrap();
        match ctl.next_event().await.unwrap().unwrap() {
            WebSocketEvent::Text(s) => assert_eq!(s, "hi"),
            other => panic!("unexpected: {:?}", other),
        }
    }

    #[tokio::test]
    async fn disconnected_event_when_client_drops() {
        let mock = MockClient::default();
        let mut ctl = ConnectionController::new(mock);
        ctl.connect("wss://unit").await.unwrap();
        // Simulate drop
        ctl.client.set_connected(false);
        match ctl.next_event().await.unwrap() {
            Some(WebSocketEvent::Disconnected) => {}
            other => panic!("expected Disconnected, got {:?}", other),
        }
        assert_eq!(ctl.state(), ConnectionState::Disconnected);
    }

    #[tokio::test]
    async fn error_event_on_receive_error() {
        let mut ctl = ConnectionController::new(
            MockClient::default().with_incoming(vec![Err(WebSocketError::Receive("boom".into()))]),
        );
        ctl.connect("wss://unit").await.unwrap();
        match ctl.next_event().await.unwrap().unwrap() {
            WebSocketEvent::Error(e) => assert!(e.contains("boom")),
            other => panic!("unexpected: {:?}", other),
        }
        match ctl.state() {
            ConnectionState::Error(e) => assert!(e.contains("boom")),
            s => panic!("unexpected state: {:?}", s),
        }
    }

    #[tokio::test]
    async fn connect_failure_sets_error_state() {
        let mock = MockClient::default();
        mock.set_connect_result(Err(WebSocketError::Connection("fail".into())));
        let mut ctl = ConnectionController::new(mock);
        let err = ctl.connect("wss://unit").await.unwrap_err();
        match err {
            WebSocketError::Connection(e) => assert_eq!(e, "fail"),
            other => panic!("unexpected err: {:?}", other),
        }
        match ctl.state() {
            ConnectionState::Error(e) => assert!(e.contains("fail")),
            s => panic!("unexpected state: {:?}", s),
        }
    }
}
