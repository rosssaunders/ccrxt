use std::{collections::HashMap, pin::Pin};

use async_trait::async_trait;
use futures::{StreamExt, channel::mpsc, stream::Stream};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{CloseEvent, ErrorEvent, MessageEvent, WebSocket};

use crate::{
    VenueMessage, WebSocketConnection,
    events::{ConnectionState, DisconnectReason, WebSocketError, WebSocketEvent},
};

/// WASM WebSocket client implementation using web-sys
pub struct WasmWebSocketClient<T: VenueMessage> {
    /// WebSocket URL
    url: String,

    /// WebSocket instance
    ws: Option<WebSocket>,

    /// Current connection state
    state: ConnectionState,

    /// Event channel sender
    event_tx: mpsc::UnboundedSender<WebSocketEvent<T>>,

    /// Event channel receiver
    event_rx: Option<mpsc::UnboundedReceiver<WebSocketEvent<T>>>,

    /// Closures for event handlers (kept alive)
    _on_open: Option<Closure<dyn FnMut()>>,
    _on_message: Option<Closure<dyn FnMut(MessageEvent)>>,
    _on_error: Option<Closure<dyn FnMut(ErrorEvent)>>,
    _on_close: Option<Closure<dyn FnMut(CloseEvent)>>,
}

impl<T: VenueMessage + 'static> WasmWebSocketClient<T> {
    /// Create a new WASM WebSocket client
    pub fn new(url: impl Into<String>) -> Result<Self, WebSocketError> {
        let (event_tx, event_rx) = mpsc::unbounded();

        Ok(Self {
            url: url.into(),
            ws: None,
            state: ConnectionState::Disconnected,
            event_tx,
            event_rx: Some(event_rx),
            _on_open: None,
            _on_message: None,
            _on_error: None,
            _on_close: None,
        })
    }
}

#[async_trait(?Send)]
impl<T: VenueMessage + 'static> WebSocketConnection<T> for WasmWebSocketClient<T> {
    async fn connect(&mut self) -> Result<(), WebSocketError> {
        if self.state == ConnectionState::Connected {
            return Err(WebSocketError::AlreadyConnected);
        }

        self.state = ConnectionState::Connecting;

        // Create WebSocket
        let ws = WebSocket::new(&self.url)
            .map_err(|e| WebSocketError::ConnectionFailed(format!("{:?}", e)))?;

        // Set binary type to arraybuffer
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let event_tx = self.event_tx.clone();

        // Setup onopen handler
        let on_open_tx = event_tx.clone();
        let on_open = Closure::wrap(Box::new(move || {
            let _ = on_open_tx.unbounded_send(WebSocketEvent::Connected);
        }) as Box<dyn FnMut()>);
        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        self._on_open = Some(on_open);

        // Setup onmessage handler
        let on_message_tx = event_tx.clone();
        let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
            // Try to get the message data
            if let Ok(text) = event.data().dyn_into::<js_sys::JsString>() {
                let text: String = text.into();
                // Try to deserialize the message
                match serde_json::from_str::<T>(&text) {
                    Ok(venue_msg) => {
                        let _ = on_message_tx
                            .unbounded_send(WebSocketEvent::Message { message: venue_msg });
                    }
                    Err(e) => {
                        let _ = on_message_tx.unbounded_send(WebSocketEvent::Error {
                            error: WebSocketError::DeserializationError(e.to_string()),
                        });
                    }
                }
            } else if let Ok(array_buffer) = event.data().dyn_into::<js_sys::ArrayBuffer>() {
                // Handle binary message
                let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                let mut data = vec![0; uint8_array.length() as usize];
                uint8_array.copy_to(&mut data);

                // Try to deserialize binary message
                match serde_json::from_slice::<T>(&data) {
                    Ok(venue_msg) => {
                        let _ = on_message_tx
                            .unbounded_send(WebSocketEvent::Message { message: venue_msg });
                    }
                    Err(e) => {
                        let _ = on_message_tx.unbounded_send(WebSocketEvent::Error {
                            error: WebSocketError::DeserializationError(e.to_string()),
                        });
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        self._on_message = Some(on_message);

        // Setup onerror handler
        let on_error_tx = event_tx.clone();
        let on_error = Closure::wrap(Box::new(move |_event: ErrorEvent| {
            let _ = on_error_tx.unbounded_send(WebSocketEvent::Error {
                error: WebSocketError::PlatformError("WebSocket error occurred".to_string()),
            });
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        self._on_error = Some(on_error);

        // Setup onclose handler
        let on_close_tx = event_tx.clone();
        let on_close = Closure::wrap(Box::new(move |event: CloseEvent| {
            let reason = if event.was_clean() {
                DisconnectReason::RemoteClosed {
                    code: event.code(),
                    reason: event.reason(),
                }
            } else {
                DisconnectReason::NetworkError {
                    details: format!("Connection closed abnormally with code {}", event.code()),
                }
            };
            let _ = on_close_tx.unbounded_send(WebSocketEvent::Disconnected { reason });
        }) as Box<dyn FnMut(CloseEvent)>);
        ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
        self._on_close = Some(on_close);

        // Wait for connection to be established
        // In WASM, we can't really block here, so we'll consider it connected
        // The actual connection status will be communicated via events
        self.ws = Some(ws);
        self.state = ConnectionState::Connected;

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), WebSocketError> {
        if self.state != ConnectionState::Connected {
            return Ok(());
        }

        self.state = ConnectionState::Disconnecting;

        // Close the WebSocket connection
        if let Some(ws) = self.ws.take() {
            let _ = ws.close();
        }

        self.state = ConnectionState::Disconnected;

        // Send disconnected event
        let _ = self.event_tx.unbounded_send(WebSocketEvent::Disconnected {
            reason: DisconnectReason::UserInitiated,
        });

        // Clear event handlers
        self._on_open = None;
        self._on_message = None;
        self._on_error = None;
        self._on_close = None;

        Ok(())
    }

    fn is_connected(&self) -> bool {
        if let Some(ws) = &self.ws {
            ws.ready_state() == WebSocket::OPEN
        } else {
            false
        }
    }

    fn connection_state(&self) -> ConnectionState {
        if let Some(ws) = &self.ws {
            match ws.ready_state() {
                WebSocket::CONNECTING => ConnectionState::Connecting,
                WebSocket::OPEN => ConnectionState::Connected,
                WebSocket::CLOSING => ConnectionState::Disconnecting,
                WebSocket::CLOSED => ConnectionState::Disconnected,
                _ => ConnectionState::Disconnected,
            }
        } else {
            self.state
        }
    }

    fn event_stream(&mut self) -> Pin<Box<dyn Stream<Item = WebSocketEvent<T>>>> {
        // Take the receiver and create a stream from it
        if let Some(rx) = self.event_rx.take() {
            Box::pin(rx)
        } else {
            // If receiver was already taken, create an empty stream
            Box::pin(futures::stream::empty())
        }
    }

    async fn send(&mut self, message: T) -> Result<(), WebSocketError> {
        if !self.is_connected() {
            return Err(WebSocketError::NotConnected);
        }

        // Serialize the message
        let json = serde_json::to_string(&message)
            .map_err(|e| WebSocketError::SerializationError(e.to_string()))?;

        // Send the message
        if let Some(ws) = &self.ws {
            ws.send_with_str(&json)
                .map_err(|e| WebSocketError::SendFailed(format!("{:?}", e)))?;
        } else {
            return Err(WebSocketError::NotConnected);
        }

        Ok(())
    }
}
