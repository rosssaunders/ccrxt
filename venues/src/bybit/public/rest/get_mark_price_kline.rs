use serde::{Deserialize, Serialize};

use super::{client::RestClient, get_kline::Kline};
use crate::bybit::{EndpointType, RestResult, enums::*};

const MARK_PRICE_KLINE_ENDPOINT: &str = "/v5/market/mark-price-kline";

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
            MARK_PRICE_KLINE_ENDPOINT,
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
    fn test_get_mark_price_kline_request_construction() {
        let request = GetMarkPriceKlineRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            interval: Interval::Min15,
            start: Some(1670601600000),
            end: None,
            limit: Some(50),
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.interval, Interval::Min15);
        assert_eq!(request.start, Some(1670601600000));
        assert_eq!(request.limit, Some(50));
        assert!(request.end.is_none());
    }
}
