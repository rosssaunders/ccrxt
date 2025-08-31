use serde::{Deserialize, Serialize};

use crate::bybit::{EndpointType, PublicRestClient, RestResult, enums::*};

const MARK_PRICE_KLINE_ENDPOINT: &str = "/v5/market/mark-price-kline";

/// Request parameters for getting mark price kline data
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarkPriceKlineRequest {
    /// Product type (Linear or Inverse)
    pub category: Category,

    /// Symbol name (e.g., "BTCUSDT")
    pub symbol: String,

    /// Kline interval
    pub interval: Interval,

    /// Start timestamp in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,

    /// End timestamp in milliseconds. Default: current time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,

    /// Limit for data size per page. [1, 1000]. Default: 200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Mark price kline data
#[derive(Debug, Clone)]
pub struct MarkPriceKline {
    /// Start timestamp of the kline in milliseconds
    pub start_time: String,

    /// Open price
    pub open_price: String,

    /// High price
    pub high_price: String,

    /// Low price
    pub low_price: String,

    /// Close price
    pub close_price: String,
}

impl<'de> Deserialize<'de> for MarkPriceKline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr: Vec<String> = Vec::deserialize(deserializer)?;
        if arr.len() != 5 {
            return Err(serde::de::Error::custom(format!(
                "Expected 5 elements in mark price kline array, got {}",
                arr.len()
            )));
        }
        #[allow(clippy::indexing_slicing)]
        Ok(MarkPriceKline {
            start_time: arr[0].clone(),
            open_price: arr[1].clone(),
            high_price: arr[2].clone(),
            low_price: arr[3].clone(),
            close_price: arr[4].clone(),
        })
    }
}

/// Mark price kline data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetMarkPriceKlineData {
    /// Product type
    pub category: Category,

    /// Symbol name
    pub symbol: String,

    /// Array of kline data
    pub list: Vec<MarkPriceKline>,
}

/// Response from the mark price kline endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetMarkPriceKlineResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetMarkPriceKlineData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl PublicRestClient {
    /// Get mark price kline data
    ///
    /// Query for historical mark price klines. This endpoint is only available for USDT/USDC perpetual
    /// and Inverse perpetual contracts.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/market/mark-price-kline)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The mark price kline request parameters including:
    ///   - `category`: Product type (Linear or Inverse)
    ///   - `symbol`: Symbol name
    ///   - `interval`: Kline interval
    ///   - `start`: Optional start timestamp
    ///   - `end`: Optional end timestamp
    ///   - `limit`: Optional result limit
    ///
    /// # Returns
    /// A result containing the mark price kline response with kline data or an error
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

    #[test]
    fn test_mark_price_kline_deserialization() {
        let json = r#"[
            "1670601600000",
            "17202.00",
            "17202.50",
            "17199.00",
            "17200.50"
        ]"#;

        let kline: MarkPriceKline = serde_json::from_str(json).unwrap();
        assert_eq!(kline.start_time, "1670601600000");
        assert_eq!(kline.open_price, "17202.00");
        assert_eq!(kline.high_price, "17202.50");
        assert_eq!(kline.low_price, "17199.00");
        assert_eq!(kline.close_price, "17200.50");
    }
}
