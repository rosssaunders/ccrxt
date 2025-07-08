use crate::bybit::{enums::*, EndpointType, RestResult};
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use super::get_kline::Kline;

const PREMIUM_INDEX_PRICE_KLINE_ENDPOINT: &str = "/v5/market/premium-index-price-kline";

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
            PREMIUM_INDEX_PRICE_KLINE_ENDPOINT,
            Some(&request),
            EndpointType::Market,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_premium_index_price_kline_request_direct_construction() {
        let request = GetPremiumIndexPriceKlineRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            interval: Interval::Day,
            start: Some(1670601600000),
            end: Some(1670688000000),
            limit: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.interval, Interval::Day);
        assert_eq!(request.start, Some(1670601600000));
        assert_eq!(request.end, Some(1670688000000));
        assert!(request.limit.is_none());
    }
}