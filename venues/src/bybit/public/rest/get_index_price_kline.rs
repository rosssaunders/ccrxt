use serde::{Deserialize, Serialize};

use crate::bybit::{EndpointType, RestResult, enums::*};

use super::client::RestClient;
use super::get_kline::Kline;

const INDEX_PRICE_KLINE_ENDPOINT: &str = "/v5/market/index-price-kline";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPriceKlineRequest {
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
pub struct GetIndexPriceKlineData {
    pub category: Category,
    pub symbol: String,
    pub list: Vec<Kline>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetIndexPriceKlineResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetIndexPriceKlineData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get index price kline data
    ///
    /// Query for historical index price klines. USDT/USDC contract and Inverse contract only.
    ///
    /// # Arguments
    /// * `request` - The index price kline request parameters
    ///
    /// # Returns
    /// A result containing the index price kline response or an error
    pub async fn get_index_price_kline(
        &self,
        request: GetIndexPriceKlineRequest,
    ) -> RestResult<GetIndexPriceKlineResponse> {
        self.send_public_request(
            INDEX_PRICE_KLINE_ENDPOINT,
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
    fn test_get_index_price_kline_request_construction() {
        let request = GetIndexPriceKlineRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            interval: Interval::Min60,
            start: None,
            end: Some(1670688000000),
            limit: Some(100),
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.interval, Interval::Min60);
        assert_eq!(request.end, Some(1670688000000));
        assert_eq!(request.limit, Some(100));
        assert!(request.start.is_none());
    }
}
