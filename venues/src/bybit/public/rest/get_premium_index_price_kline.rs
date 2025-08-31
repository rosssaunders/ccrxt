use serde::{Deserialize, Serialize};

use crate::bybit::{EndpointType, PublicRestClient, RestResult, enums::*};

const PREMIUM_INDEX_PRICE_KLINE_ENDPOINT: &str = "/v5/market/premium-index-price-kline";

/// Request parameters for getting premium index price kline data
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPremiumIndexPriceKlineRequest {
    /// Product type (Linear only)
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

/// Premium index price kline data
#[derive(Debug, Clone)]
pub struct PremiumIndexPriceKline {
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

impl<'de> Deserialize<'de> for PremiumIndexPriceKline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr: Vec<String> = Vec::deserialize(deserializer)?;
        if arr.len() != 5 {
            return Err(serde::de::Error::custom(format!(
                "Expected 5 elements in premium index price kline array, got {}",
                arr.len()
            )));
        }
        #[allow(clippy::indexing_slicing)]
        Ok(PremiumIndexPriceKline {
            start_time: arr[0].clone(),
            open_price: arr[1].clone(),
            high_price: arr[2].clone(),
            low_price: arr[3].clone(),
            close_price: arr[4].clone(),
        })
    }
}

/// Premium index price kline data container
#[derive(Debug, Clone, Deserialize)]
pub struct GetPremiumIndexPriceKlineData {
    /// Product type
    pub category: Category,

    /// Symbol name
    pub symbol: String,

    /// Array of kline data
    pub list: Vec<PremiumIndexPriceKline>,
}

/// Response from the premium index price kline endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetPremiumIndexPriceKlineResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: GetPremiumIndexPriceKlineData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl PublicRestClient {
    /// Get premium index price kline data
    ///
    /// Query for historical premium index klines. This endpoint is only available for USDT and USDC
    /// perpetual contracts.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/market/premium-index-price-kline)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The premium index price kline request parameters including:
    ///   - `category`: Product type (must be Linear)
    ///   - `symbol`: Symbol name
    ///   - `interval`: Kline interval
    ///   - `start`: Optional start timestamp
    ///   - `end`: Optional end timestamp
    ///   - `limit`: Optional result limit
    ///
    /// # Returns
    /// A result containing the premium index price kline response with kline data or an error
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

    #[test]
    fn test_premium_index_price_kline_deserialization() {
        let json = r#"[
            "1670601600000",
            "17202.00",
            "17202.50",
            "17199.00",
            "17200.50"
        ]"#;

        let kline: PremiumIndexPriceKline = serde_json::from_str(json).unwrap();
        assert_eq!(kline.start_time, "1670601600000");
        assert_eq!(kline.open_price, "17202.00");
        assert_eq!(kline.high_price, "17202.50");
        assert_eq!(kline.low_price, "17199.00");
        assert_eq!(kline.close_price, "17200.50");
    }
}
