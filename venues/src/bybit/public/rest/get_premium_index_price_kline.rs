use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

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

#[derive(Debug, Clone)]
pub struct PremiumIndexPriceKline {
    pub start_time: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
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

#[derive(Debug, Clone, Deserialize)]
pub struct GetPremiumIndexPriceKlineData {
    pub category: Category,
    pub symbol: String,
    pub list: Vec<PremiumIndexPriceKline>,
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
