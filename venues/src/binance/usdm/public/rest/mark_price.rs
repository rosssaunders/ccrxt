use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::{RestResult, public_client::RestClient};

const MARK_PRICE_ENDPOINT: &str = "/fapi/v1/premiumIndex";

/// Request parameters for the mark price endpoint.
#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
pub struct MarkPriceRequest {
    /// Trading pair symbol (e.g., "BTCUSDT"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,
}

/// Represents a mark price response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// Mark price as a string.
    pub mark_price: Cow<'static, str>,

    /// Index price as a string.
    pub index_price: Cow<'static, str>,

    /// Estimated settle price as a string, only useful in the last hour before settlement starts.
    pub estimated_settle_price: Cow<'static, str>,

    /// Last funding rate as a string.
    pub last_funding_rate: Cow<'static, str>,

    /// Interest rate as a string.
    pub interest_rate: Cow<'static, str>,

    /// Next funding time in milliseconds since epoch.
    pub next_funding_time: u64,

    /// Timestamp in milliseconds since epoch.
    pub time: u64,
}

/// Response wrapper for mark price endpoint that handles both single and multiple results.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkPriceResult {
    /// Multiple mark prices when no symbol is specified.
    Multiple(Vec<MarkPrice>),

    /// Single mark price when a specific symbol is requested.
    Single(MarkPrice),
}

impl RestClient {
    /// Mark Price
    ///
    /// Mark Price and Funding Rate
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `request` - The mark price request parameters
    ///
    /// # Returns
    /// Mark price result that can be either a single mark price or multiple mark prices
    pub async fn get_mark_price(&self, params: MarkPriceRequest) -> RestResult<MarkPriceResult> {
        self.send_get_request(MARK_PRICE_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_price_request_serialization() {
        let request = MarkPriceRequest {
            symbol: Some("BTCUSDT".into()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_mark_price_request_no_symbol() {
        let request = MarkPriceRequest { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_mark_price_single_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "markPrice": "45384.10000000",
            "indexPrice": "45380.25000000",
            "estimatedSettlePrice": "45382.00000000",
            "lastFundingRate": "0.00010000",
            "interestRate": "0.00010000",
            "nextFundingTime": 1625270400000,
            "time": 1625184000000
        }"#;

        let mark_price: MarkPrice = serde_json::from_str(json).unwrap();
        assert_eq!(mark_price.symbol, "BTCUSDT");
        assert_eq!(mark_price.mark_price, "45384.10000000");
        assert_eq!(mark_price.index_price, "45380.25000000");
        assert_eq!(mark_price.estimated_settle_price, "45382.00000000");
        assert_eq!(mark_price.last_funding_rate, "0.00010000");
        assert_eq!(mark_price.interest_rate, "0.00010000");
        assert_eq!(mark_price.next_funding_time, 1625270400000);
        assert_eq!(mark_price.time, 1625184000000);
    }

    #[test]
    fn test_mark_price_result_single_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "markPrice": "45384.10000000",
            "indexPrice": "45380.25000000",
            "estimatedSettlePrice": "45382.00000000",
            "lastFundingRate": "0.00010000",
            "interestRate": "0.00010000",
            "nextFundingTime": 1625270400000,
            "time": 1625184000000
        }"#;

        let result: MarkPriceResult = serde_json::from_str(json).unwrap();
        match result {
            MarkPriceResult::Single(mark_price) => {
                assert_eq!(mark_price.symbol, "BTCUSDT");
                assert_eq!(mark_price.mark_price, "45384.10000000");
            }
            MarkPriceResult::Multiple(_) => panic!("Expected Single variant"),
        }
    }

    #[test]
    fn test_mark_price_result_multiple_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "markPrice": "45384.10000000",
                "indexPrice": "45380.25000000",
                "estimatedSettlePrice": "45382.00000000",
                "lastFundingRate": "0.00010000",
                "interestRate": "0.00010000",
                "nextFundingTime": 1625270400000,
                "time": 1625184000000
            },
            {
                "symbol": "ETHUSDT",
                "markPrice": "3072.84000000",
                "indexPrice": "3071.95000000",
                "estimatedSettlePrice": "3072.50000000",
                "lastFundingRate": "-0.00005000",
                "interestRate": "0.00010000",
                "nextFundingTime": 1625270400000,
                "time": 1625184000000
            }
        ]"#;

        let result: MarkPriceResult = serde_json::from_str(json).unwrap();
        match result {
            MarkPriceResult::Multiple(marks) => {
                assert_eq!(marks.len(), 2);
                assert_eq!(marks[0].symbol, "BTCUSDT");
                assert_eq!(marks[0].last_funding_rate, "0.00010000");
                assert_eq!(marks[1].symbol, "ETHUSDT");
                assert_eq!(marks[1].last_funding_rate, "-0.00005000");
            }
            MarkPriceResult::Single(_) => panic!("Expected Multiple variant"),
        }
    }

    #[test]
    fn test_mark_price_negative_funding_rate() {
        let json = r#"{
            "symbol": "DOGEUSDT",
            "markPrice": "0.12340000",
            "indexPrice": "0.12350000",
            "estimatedSettlePrice": "0.12345000",
            "lastFundingRate": "-0.00375000",
            "interestRate": "0.00010000",
            "nextFundingTime": 1625270400000,
            "time": 1625184000000
        }"#;

        let mark_price: MarkPrice = serde_json::from_str(json).unwrap();
        assert_eq!(mark_price.last_funding_rate, "-0.00375000");
    }

    #[test]
    fn test_mark_price_zero_values() {
        let json = r#"{
            "symbol": "NEWUSDT",
            "markPrice": "1.00000000",
            "indexPrice": "1.00000000",
            "estimatedSettlePrice": "1.00000000",
            "lastFundingRate": "0.00000000",
            "interestRate": "0.00000000",
            "nextFundingTime": 1625270400000,
            "time": 1625184000000
        }"#;

        let mark_price: MarkPrice = serde_json::from_str(json).unwrap();
        assert_eq!(mark_price.last_funding_rate, "0.00000000");
        assert_eq!(mark_price.interest_rate, "0.00000000");
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(MARK_PRICE_ENDPOINT, "/fapi/v1/premiumIndex");
    }
}
