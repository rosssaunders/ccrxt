use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

/// Endpoint URL path for kline data
const ENDPOINT_PATH: &str = "/v5/market/kline";

/// Request parameters for getting kline data
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKlineRequest {
    /// Product type. Required for Inverse and USDC Futures, optional for others
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,

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

    /// Limit for data size per page. Spot: [1,1000], others: [1,200]. Default: 200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Kline (candlestick) data
#[derive(Debug, Clone)]
pub struct Kline {
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

    /// Trade volume. Unit of contract: pieces of contract. Unit of spot: quantity of coins
    pub volume: String,

    /// Turnover. Unit of contract: quote currency. Unit of spot: quote currency
    pub turnover: String,
}

impl<'de> Deserialize<'de> for Kline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr: Vec<String> = Vec::deserialize(deserializer)?;
        if arr.len() != 7 {
            return Err(serde::de::Error::custom(
                "Expected 7 elements in kline array",
            ));
        }
        #[allow(clippy::indexing_slicing)]
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

/// Kline data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetKlineData {
    /// Product type
    pub category: Category,

    /// Symbol name
    pub symbol: String,

    /// Array of kline data
    pub list: Vec<Kline>,
}

/// Response from the kline endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetKlineResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetKlineData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get kline (candlestick) data
    ///
    /// Query for historical klines. Charts are returned in groups based on the requested interval.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/market/kline)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The kline request parameters including:
    ///   - `category`: Optional product type (required for Inverse and USDC Futures)
    ///   - `symbol`: Symbol name
    ///   - `interval`: Kline interval
    ///   - `start`: Optional start timestamp
    ///   - `end`: Optional end timestamp
    ///   - `limit`: Optional result limit
    ///
    /// # Returns
    /// A result containing the kline response with historical candlestick data or an error
    pub async fn get_kline(&self, request: GetKlineRequest) -> RestResult<GetKlineResponse> {
        self.send_public_request(ENDPOINT_PATH, Some(&request), EndpointType::Market)
            .await
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
        let request = GetKlineRequest {
            category: Some(Category::Linear),
            symbol: "BTCUSDT".to_string(),
            interval: Interval::Min5,
            start: Some(1670601600000),
            end: None,
            limit: Some(100),
        };

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.interval, Interval::Min5);
        assert_eq!(request.category, Some(Category::Linear));
        assert_eq!(request.start, Some(1670601600000));
        assert_eq!(request.limit, Some(100));
        assert!(request.end.is_none());
    }
}
