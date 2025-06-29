use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;
use super::get_kline::Kline;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPremiumIndexPriceKlineRequest {
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
pub struct GetPremiumIndexPriceKlineData {
    pub category: Category,
    pub symbol: String,
    pub list: Vec<Kline>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetPremiumIndexPriceKlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetPremiumIndexPriceKlineData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get premium index price kline data
    ///
    /// Query for historical premium index klines. USDT and USDC perpetual only.
    ///
    /// # Arguments
    /// * `request` - The premium index price kline request parameters
    ///
    /// # Returns
    /// A result containing the premium index price kline response or an error
    pub async fn get_premium_index_price_kline(
        &self,
        request: GetPremiumIndexPriceKlineRequest,
    ) -> RestResult<GetPremiumIndexPriceKlineResponse> {
        self.send_public_request(
            "/v5/market/premium-index-price-kline",
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}

impl GetPremiumIndexPriceKlineRequest {
    /// Create a new premium index price kline request
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
    fn test_get_premium_index_price_kline_request_builder() {
        let request = GetPremiumIndexPriceKlineRequest::new(
            Category::Linear,
            "BTCUSDT".to_string(),
            Interval::Day
        )
        .start(1670601600000)
        .end(1670688000000);

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.interval, Interval::Day);
        assert_eq!(request.start, Some(1670601600000));
        assert_eq!(request.end, Some(1670688000000));
        assert!(request.limit.is_none());
    }
}