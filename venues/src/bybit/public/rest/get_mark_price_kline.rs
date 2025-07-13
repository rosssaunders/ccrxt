use serde::{Deserialize, Serialize};

use super::client::RestClient;
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

#[derive(Debug, Clone)]
pub struct MarkPriceKline {
    pub start_time: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
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

#[derive(Debug, Clone, Deserialize)]
pub struct GetMarkPriceKlineData {
    pub category: Category,
    pub symbol: String,
    pub list: Vec<MarkPriceKline>,
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
