use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for Mark Price and Funding Rate.
const PREMIUM_INDEX_ENDPOINT: &str = "/fapi/v1/premiumIndex";

/// Request parameters for the Mark Price and Funding Rate endpoint.
///
/// Used to query mark price and funding rate for a specific symbol or all symbols.
#[derive(Debug, Clone, Serialize, Default)]
pub struct PremiumIndexRequest {
    /// The symbol to query (e.g., "BTCUSDT"). If not sent, returns all symbols.
    /// Optional. Must be a valid trading symbol supported by Binance USDM futures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,
}

/// Response data for a single symbol from the Mark Price and Funding Rate endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexResponse {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// Mark price for the symbol (as a string, e.g., "45384.10000000").
    pub mark_price: Cow<'static, str>,

    /// Index price for the symbol (as a string).
    pub index_price: Cow<'static, str>,

    /// Estimated settlement price (as a string).
    pub estimated_settle_price: Cow<'static, str>,

    /// Last funding rate (as a string, may be negative).
    pub last_funding_rate: Cow<'static, str>,

    /// Interest rate (as a string).
    pub interest_rate: Cow<'static, str>,

    /// Timestamp of the next funding event (milliseconds since epoch).
    pub next_funding_time: u64,

    /// Current server time (milliseconds since epoch).
    pub time: u64,
}

/// Result type for Mark Price and Funding Rate endpoint.
///
/// The API may return either a single object or an array of objects.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PremiumIndexResult {
    /// Multiple results (all symbols).
    Multiple(Vec<PremiumIndexResponse>),

    /// Single result (one symbol).
    Single(PremiumIndexResponse),
}

impl RestClient {
    /// Mark Price
    ///
    /// Mark Price and Funding Rate
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Mark-Price)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `params` - Request parameters for Mark Price and Funding Rate
    ///
    /// # Returns
    /// * `PremiumIndexResult` - Single or multiple mark price/funding rate objects
    pub async fn premium_index(
        &self,
        params: PremiumIndexRequest,
    ) -> RestResult<PremiumIndexResult> {
        self.send_get_request(PREMIUM_INDEX_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_premium_index_request_serialization() {
        let request = PremiumIndexRequest {
            symbol: Some("BTCUSDT".into()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_premium_index_request_no_symbol() {
        let request = PremiumIndexRequest { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_premium_index_response_deserialization() {
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

        let response: PremiumIndexResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.mark_price, "45384.10000000");
        assert_eq!(response.index_price, "45380.25000000");
        assert_eq!(response.estimated_settle_price, "45382.00000000");
        assert_eq!(response.last_funding_rate, "0.00010000");
        assert_eq!(response.interest_rate, "0.00010000");
        assert_eq!(response.next_funding_time, 1625270400000);
        assert_eq!(response.time, 1625184000000);
    }

    #[test]
    fn test_premium_index_result_single_deserialization() {
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

        let result: PremiumIndexResult = serde_json::from_str(json).unwrap();
        match result {
            PremiumIndexResult::Single(response) => {
                assert_eq!(response.symbol, "BTCUSDT");
                assert_eq!(response.mark_price, "45384.10000000");
            }
            PremiumIndexResult::Multiple(_) => panic!("Expected Single variant"),
        }
    }

    #[test]
    fn test_premium_index_result_multiple_deserialization() {
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

        let result: PremiumIndexResult = serde_json::from_str(json).unwrap();
        match result {
            PremiumIndexResult::Multiple(responses) => {
                assert_eq!(responses.len(), 2);
                assert_eq!(responses[0].symbol, "BTCUSDT");
                assert_eq!(responses[0].last_funding_rate, "0.00010000");
                assert_eq!(responses[1].symbol, "ETHUSDT");
                assert_eq!(responses[1].last_funding_rate, "-0.00005000");
            }
            PremiumIndexResult::Single(_) => panic!("Expected Multiple variant"),
        }
    }

    #[test]
    fn test_premium_index_negative_funding_rate() {
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

        let response: PremiumIndexResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.last_funding_rate, "-0.00375000");
    }

    #[test]
    fn test_premium_index_zero_rates() {
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

        let response: PremiumIndexResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.last_funding_rate, "0.00000000");
        assert_eq!(response.interest_rate, "0.00000000");
    }

    #[test]
    fn test_premium_index_result_empty_array() {
        let json = r#"[]"#;
        let result: PremiumIndexResult = serde_json::from_str(json).unwrap();
        match result {
            PremiumIndexResult::Multiple(responses) => {
                assert_eq!(responses.len(), 0);
            }
            PremiumIndexResult::Single(_) => panic!("Expected Multiple variant for empty array"),
        }
    }

    #[test]
    fn test_premium_index_default_request() {
        let request = PremiumIndexRequest::default();
        assert!(request.symbol.is_none());
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_premium_index_high_precision_values() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "markPrice": "45384.12345678",
            "indexPrice": "45380.98765432",
            "estimatedSettlePrice": "45382.55555555",
            "lastFundingRate": "0.00012345",
            "interestRate": "0.00098765",
            "nextFundingTime": 1625270400000,
            "time": 1625184000000
        }"#;

        let response: PremiumIndexResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.mark_price, "45384.12345678");
        assert_eq!(response.index_price, "45380.98765432");
        assert_eq!(response.estimated_settle_price, "45382.55555555");
        assert_eq!(response.last_funding_rate, "0.00012345");
        assert_eq!(response.interest_rate, "0.00098765");
    }
}
