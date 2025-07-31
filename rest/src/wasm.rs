use async_trait::async_trait;
use crate::http_client::{HttpClient, HttpError, Method, Request, Response};
use wasm_http_client::HttpClient as WasmHttpClientTrait;

/// Wrapper around wasm_http_client::WasmHttpClient that implements our HttpClient trait
#[derive(Clone)]
pub struct WasmHttpClient {
    inner: wasm_http_client::WasmHttpClient,
}

impl WasmHttpClient {
    pub fn new() -> Self {
        Self {
            inner: wasm_http_client::WasmHttpClient::new(),
        }
    }
}

impl Default for WasmHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl HttpClient for WasmHttpClient {
    async fn execute(&self, request: Request) -> Result<Response, HttpError> {
        // Convert our Method to wasm_http_client::Method
        let method = match request.method {
            Method::Get => wasm_http_client::Method::Get,
            Method::Post => wasm_http_client::Method::Post,
            Method::Put => wasm_http_client::Method::Put,
            Method::Delete => wasm_http_client::Method::Delete,
            Method::Head => wasm_http_client::Method::Head,
            Method::Options => wasm_http_client::Method::Options,
            Method::Patch => wasm_http_client::Method::Patch,
        };

        // Create wasm_http_client::Request
        let wasm_request = wasm_http_client::Request {
            method,
            url: request.url,
            headers: request.headers,
            body: request.body,
            timeout: request.timeout,
        };

        // Execute using the inner client
        let wasm_response = self.inner.execute(wasm_request).await
            .map_err(|e| match e {
                wasm_http_client::HttpError::Network(msg) => HttpError::Network(msg),
                wasm_http_client::HttpError::Timeout => HttpError::Timeout,
                wasm_http_client::HttpError::InvalidUrl(msg) => HttpError::InvalidUrl(msg),
                wasm_http_client::HttpError::Decode(msg) => HttpError::Decode(msg),
                wasm_http_client::HttpError::Http { status, body } => HttpError::Http { status, body },
                wasm_http_client::HttpError::Unknown(msg) => HttpError::Unknown(msg),
            })?;

        // Convert response back
        Ok(Response {
            status: wasm_response.status,
            headers: wasm_response.headers,
            body: wasm_response.body,
        })
    }
}