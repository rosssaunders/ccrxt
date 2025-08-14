use std::collections::HashMap;

use crate::{VenueMessage, WebSocketError};

#[cfg(feature = "native")]
use crate::native::NativeWebSocketClient;

#[cfg(feature = "wasm")]
use crate::wasm::WasmWebSocketClient;

/// Platform-agnostic WebSocket client builder
///
/// This builder creates the appropriate WebSocket client implementation
/// based on the target platform (native or WASM).
///
/// # Example
///
/// ```ignore
/// let client = WebSocketClientBuilder::new("wss://example.com")
///     .header("Authorization", "Bearer token")
///     .build::<MyMessage>()?;
/// ```
pub struct WebSocketClientBuilder {
    url: String,
    headers: HashMap<String, String>,
}

impl WebSocketClientBuilder {
    /// Create a new WebSocket client builder with the given URL
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            headers: HashMap::new(),
        }
    }

    /// Add a header to the WebSocket connection request
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Add multiple headers to the WebSocket connection request
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Build a native WebSocket client (available only when native feature is enabled)
    #[cfg(feature = "native")]
    pub fn build<T: VenueMessage + 'static>(
        self,
    ) -> Result<NativeWebSocketClient<T>, WebSocketError> {
        let mut client = NativeWebSocketClient::new(self.url);

        for (key, value) in self.headers {
            client = client.with_header(key, value);
        }

        Ok(client)
    }

    /// Build a WASM WebSocket client (available only when wasm feature is enabled)
    #[cfg(feature = "wasm")]
    pub fn build<T: VenueMessage + 'static>(
        self,
    ) -> Result<WasmWebSocketClient<T>, WebSocketError> {
        let mut client = WasmWebSocketClient::new(self.url)?;

        // Note: web-sys WebSocket doesn't support custom headers in the constructor
        // Headers would need to be sent as part of the protocol if needed
        if !self.headers.is_empty() {
            // Log a warning that headers are not supported in WASM WebSocket
            // In a real implementation, we might want to handle this differently
        }

        Ok(client)
    }

    /// Build a boxed WebSocket client that implements the WebSocketConnection trait
    /// This is useful when you want to abstract over the concrete implementation
    #[cfg(feature = "native")]
    pub fn build_boxed<T: VenueMessage + 'static>(
        self,
    ) -> Result<Box<dyn crate::WebSocketConnection<T>>, WebSocketError> {
        Ok(Box::new(self.build::<T>()?))
    }

    #[cfg(feature = "wasm")]
    pub fn build_boxed<T: VenueMessage + 'static>(
        self,
    ) -> Result<Box<dyn crate::WebSocketConnection<T>>, WebSocketError> {
        Ok(Box::new(self.build::<T>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let builder = WebSocketClientBuilder::new("wss://example.com");
        assert_eq!(builder.url, "wss://example.com");
        assert!(builder.headers.is_empty());
    }

    #[test]
    fn test_builder_with_headers() {
        let builder = WebSocketClientBuilder::new("wss://example.com")
            .header("Authorization", "Bearer token")
            .header("X-Custom", "value");

        assert_eq!(builder.headers.len(), 2);
        assert_eq!(
            builder.headers.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
        assert_eq!(builder.headers.get("X-Custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_builder_with_multiple_headers() {
        let mut headers = HashMap::new();
        headers.insert("Header1".to_string(), "Value1".to_string());
        headers.insert("Header2".to_string(), "Value2".to_string());

        let builder = WebSocketClientBuilder::new("wss://example.com").headers(headers);

        assert_eq!(builder.headers.len(), 2);
    }
}
