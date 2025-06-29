use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;
use super::get_kline::Kline;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarkPriceKlineRequest {
    pub category: Category,
    pub symbol: String,
    pub interval: Interval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetMarkPriceKlineData {
    pub category: Category,
    pub symbol: String,
    pub list: Vec<Kline>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetMarkPriceKlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetMarkPriceKlineData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get mark price kline data
    ///
    /// Query for historical mark price klines. USDT/USDC contract and Inverse contract only.
    ///
    /// # Arguments
    /// * `request` - The mark price kline request parameters
    ///
    /// # Returns
    /// A result containing the mark price kline response or an error
    pub async fn get_mark_price_kline(
        &self,
        request: GetMarkPriceKlineRequest,
    ) -> RestResult<GetMarkPriceKlineResponse> {
        self.send_public_request(
            "/v5/market/mark-price-kline",
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}

impl GetMarkPriceKlineRequest {
    /// Create a new mark price kline request
    pub fn new(category: Category, symbol: String, interval: Interval) -> Self {
        Self {
            category,
            symbol,
            interval,
            start: None,
            end: None,
            limit: None,
        }
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
    fn test_get_mark_price_kline_request_builder() {
        let request = GetMarkPriceKlineRequest::new(
            Category::Linear,
            "BTCUSDT".to_string(),
            Interval::Min15
        )
        .start(1670601600000)
        .limit(50);

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.interval, Interval::Min15);
        assert_eq!(request.start, Some(1670601600000));
        assert_eq!(request.limit, Some(50));
        assert!(request.end.is_none());
    }
}