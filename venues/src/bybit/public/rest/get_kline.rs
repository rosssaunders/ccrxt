use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKlineRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    pub symbol: String,
    pub interval: Interval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct Kline {
    pub start_time: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub volume: String,
    pub turnover: String,
}

impl<'de> Deserialize<'de> for Kline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr: Vec<String> = Vec::deserialize(deserializer)?;
        if arr.len() != 7 {
            return Err(serde::de::Error::custom("Expected 7 elements in kline array"));
        }
        Ok(Kline {
            start_time: arr[0].clone(),
            open_price: arr[1].clone(),
            high_price: arr[2].clone(),
            low_price: arr[3].clone(),
            close_price: arr[4].clone(),
            volume: arr[5].clone(),
            turnover: arr[6].clone(),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetKlineData {
    pub category: Category,
    pub symbol: String,
    pub list: Vec<Kline>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetKlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetKlineData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get kline (candlestick) data
    ///
    /// Query for historical klines. Charts are returned in groups based on the requested interval.
    ///
    /// # Arguments
    /// * `request` - The kline request parameters
    ///
    /// # Returns
    /// A result containing the kline response or an error
    pub async fn get_kline(&self, request: GetKlineRequest) -> RestResult<GetKlineResponse> {
        self.send_public_request(
            "/v5/market/kline",
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}

impl GetKlineRequest {
    /// Create a new kline request
    pub fn new(symbol: String, interval: Interval) -> Self {
        Self {
            category: None,
            symbol,
            interval,
            start: None,
            end: None,
            limit: None,
        }
    }

    /// Set the category (default: linear)
    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    /// Set the start timestamp (ms)
    pub fn start(mut self, start: u64) -> Self {
        self.start = Some(start);
        self
    }

    /// Set the end timestamp (ms)
    pub fn end(mut self, end: u64) -> Self {
        self.end = Some(end);
        self
    }

    /// Set the limit (1-1000, default: 200)
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kline_deserialization() {
        let json = r#"[
            "1670601600000",
            "17202.00",
            "17202.50",
            "17199.00",
            "17200.50",
            "268611",
            "15.60925564"
        ]"#;

        let kline: Kline = serde_json::from_str(json).unwrap();
        assert_eq!(kline.start_time, "1670601600000");
        assert_eq!(kline.open_price, "17202.00");
        assert_eq!(kline.high_price, "17202.50");
        assert_eq!(kline.low_price, "17199.00");
        assert_eq!(kline.close_price, "17200.50");
        assert_eq!(kline.volume, "268611");
        assert_eq!(kline.turnover, "15.60925564");
    }

    #[test]
    fn test_get_kline_request_builder() {
        let request = GetKlineRequest::new("BTCUSDT".to_string(), Interval::Min5)
            .category(Category::Linear)
            .start(1670601600000)
            .limit(100);

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.interval, Interval::Min5);
        assert_eq!(request.category, Some(Category::Linear));
        assert_eq!(request.start, Some(1670601600000));
        assert_eq!(request.limit, Some(100));
        assert!(request.end.is_none());
    }
}