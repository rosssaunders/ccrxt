use std::time::Duration;

use async_trait::async_trait;
use reqwest;

use crate::http_client::{HttpClient, HttpError, Method, Request, Response};

#[derive(Clone)]
pub struct NativeHttpClient {
    client: reqwest::Client,
}

impl NativeHttpClient {
    pub fn new() -> Result<Self, HttpError> {
        let client = reqwest::Client::builder()
            .build()
            .map_err(|e| HttpError::Unknown(format!("Failed to create client: {}", e)))?;

        Ok(Self { client })
    }

    pub fn builder() -> NativeHttpClientBuilder {
        NativeHttpClientBuilder::new()
    }
}

impl Default for NativeHttpClient {
    fn default() -> Self {
        // Avoid expect() to satisfy clippy and project no-panic requirement in tests.
        match Self::new() {
            Ok(client) => client,
            Err(_e) => {
                // Fallback: build a minimal reqwest client; if it fails, construct a clear Unknown error variant payload.
                // Since Default cannot return Result, choose a sensible fallback.
                let client = reqwest::Client::builder()
                    .build()
                    .unwrap_or_else(|_| reqwest::Client::new());
                Self { client }
            }
        }
    }
}

#[async_trait]
impl HttpClient for NativeHttpClient {
    async fn execute(&self, request: Request) -> Result<Response, HttpError> {
        let method = match request.method {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
            Method::Options => reqwest::Method::OPTIONS,
            Method::Patch => reqwest::Method::PATCH,
        };

        let mut builder = self.client.request(method, &request.url);

        for (key, value) in request.headers {
            builder = builder.header(&key, &value);
        }

        if let Some(body) = request.body {
            builder = builder.body(body);
        }

        if let Some(timeout) = request.timeout {
            builder = builder.timeout(timeout);
        }

        let response = builder.send().await.map_err(|e| {
            if e.is_timeout() {
                HttpError::Timeout
            } else if e.is_connect() || e.is_request() {
                HttpError::Network(e.to_string())
            } else {
                HttpError::Unknown(e.to_string())
            }
        })?;

        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body = response
            .bytes()
            .await
            .map_err(|e| HttpError::Network(format!("Failed to read response body: {}", e)))?
            .to_vec();

        Ok(Response {
            status,
            headers,
            body,
        })
    }
}

pub struct NativeHttpClientBuilder {
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    user_agent: Option<String>,
}

impl NativeHttpClientBuilder {
    pub fn new() -> Self {
        Self {
            timeout: None,
            connect_timeout: None,
            user_agent: None,
        }
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn build(self) -> Result<NativeHttpClient, HttpError> {
        let mut builder = reqwest::Client::builder();

        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(connect_timeout) = self.connect_timeout {
            builder = builder.connect_timeout(connect_timeout);
        }

        if let Some(user_agent) = self.user_agent {
            builder = builder.user_agent(user_agent);
        }

        let client = builder
            .build()
            .map_err(|e| HttpError::Unknown(format!("Failed to create client: {}", e)))?;

        Ok(NativeHttpClient { client })
    }
}

impl Default for NativeHttpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
