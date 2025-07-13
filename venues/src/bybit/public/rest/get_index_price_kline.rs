use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

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

#[derive(Debug, Clone)]
pub struct IndexPriceKline {
    pub start_time: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
}

impl<'de> Deserialize<'de> for IndexPriceKline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr: Vec<String> = Vec::deserialize(deserializer)?;
        if arr.len() != 5 {
            return Err(serde::de::Error::custom(format!(
                "Expected 5 elements in index price kline array, got {}",
                arr.len()
            )));
        }
        #[allow(clippy::indexing_slicing)]
        Ok(IndexPriceKline {
            start_time: arr[0].clone(),
            open_price: arr[1].clone(),
            high_price: arr[2].clone(),
            low_price: arr[3].clone(),
            close_price: arr[4].clone(),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetIndexPriceKlineData {
    pub category: Category,
    pub symbol: String,
    pub list: Vec<IndexPriceKline>,
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

    #[test]
    fn test_index_price_kline_deserialization() {
        let json = r#"[
            "1670601600000",
            "17202.00",
            "17202.50",
            "17199.00",
            "17200.50"
        ]"#;

        let kline: IndexPriceKline = serde_json::from_str(json).unwrap();
        assert_eq!(kline.start_time, "1670601600000");
        assert_eq!(kline.open_price, "17202.00");
        assert_eq!(kline.high_price, "17202.50");
        assert_eq!(kline.low_price, "17199.00");
        assert_eq!(kline.close_price, "17200.50");
    }
}
