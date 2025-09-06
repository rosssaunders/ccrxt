use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub timeout: Option<Duration>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Patch,
}

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Bytes,
}

impl Response {
    pub fn text(&self) -> Result<String, HttpError> {
        std::str::from_utf8(self.body.as_ref())
            .map(|s| s.to_owned())
            .map_err(|e| HttpError::Decode(format!("Failed to decode as UTF-8: {}", e)))
    }

    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Result<T, HttpError> {
        serde_json::from_slice(self.body.as_ref())
            .map_err(|e| HttpError::Decode(format!("Failed to decode JSON: {}", e)))
    }

    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum HttpError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Timeout error")]
    Timeout,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Decode error: {0}")]
    Decode(String),

    #[error("HTTP error: status {status}, body: {body}")]
    Http { status: u16, body: String },

    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn execute(&self, request: Request) -> Result<Response, HttpError>;
}

pub struct RequestBuilder {
    method: Method,
    url: String,
    headers: HashMap<String, String>,
    query: Vec<(String, String)>,
    body: Option<Vec<u8>>,
    timeout: Option<Duration>,
}

impl RequestBuilder {
    pub fn new(method: Method, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: HashMap::new(),
            query: Vec::new(),
            body: None,
            timeout: None,
        }
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    pub fn query<T: Serialize>(mut self, params: &T) -> Result<Self, HttpError> {
        let query_string = serde_urlencoded::to_string(params)
            .map_err(|e| HttpError::Unknown(format!("Failed to serialize query params: {}", e)))?;

        for pair in query_string.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                self.query.push((key.to_string(), value.to_string()));
            }
        }

        Ok(self)
    }

    pub fn json<T: Serialize>(mut self, json: &T) -> Result<Self, HttpError> {
        let body = serde_json::to_vec(json)
            .map_err(|e| HttpError::Unknown(format!("Failed to serialize JSON: {}", e)))?;
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        self.body = Some(body);
        Ok(self)
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Request {
        let mut url = self.url;

        if !self.query.is_empty() {
            let query_string = self
                .query
                .into_iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");

            url = if url.contains('?') {
                format!("{}&{}", url, query_string)
            } else {
                format!("{}?{}", url, query_string)
            };
        }

        Request {
            method: self.method,
            url,
            headers: self.headers,
            body: self.body,
            timeout: self.timeout,
        }
    }
}
