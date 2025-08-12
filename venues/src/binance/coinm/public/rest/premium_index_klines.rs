use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, enums::KlineInterval, public::rest::RestClient};

/// Endpoint path for premium index kline data.
const PREMIUM_INDEX_KLINES_ENDPOINT: &str = "/dapi/v1/premiumIndexKlines";

/// Request parameters for the premium index kline data endpoint.
///
/// All fields are mapped to the Binance API as per official documentation.
#[derive(Debug, Clone, Serialize)]
pub struct PremiumIndexKlineRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    ///
    /// Must be a valid symbol listed on Binance Coin-M Futures.
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Kline interval. See `KlineInterval` enum for valid values.
    #[serde(rename = "interval")]
    pub interval: KlineInterval,

    /// Start time in milliseconds since epoch. Optional.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds since epoch. Optional.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of klines to return. Default: 500; max: 1500. Optional.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single premium index kline returned by the API.
///
/// Klines are returned as arrays with the following structure:
/// [
///   0: Open time (u64),
///   1: Open price (String),
///   2: High price (String),
///   3: Low price (String),
///   4: Close price (String),
///   5: Ignore (String),
///   6: Close time (u64),
///   7: Ignore (String),
///   8: Ignore (u64),
///   9: Ignore (String),
///   10: Ignore (String),
///   11: Ignore (String)
/// ]
#[derive(Debug, Clone, Deserialize)]
pub struct PremiumIndexKline(
    pub u64,    // Open time
    pub String, // Open price
    pub String, // High price
    pub String, // Low price
    pub String, // Close price
    pub String, // Ignore
    pub u64,    // Close time
    pub String, // Ignore
    pub u64,    // Ignore
    pub String, // Ignore
    pub String, // Ignore
    pub String, // Ignore
);

/// Response from the premium index kline data endpoint.
///
/// The API returns an array of klines, each represented as a tuple (see `PremiumIndexKline`).
pub type PremiumIndexKlineResponse = Vec<PremiumIndexKline>;

impl RestClient {
    /// Premium Index Kline Data
    ///
    /// Retrieves premium index kline bars for a given symbol and interval. Klines are uniquely identified by their open time.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Premium-Index-Kline-Data)
    ///
    /// Weight: Based on parameter LIMIT:
    /// - [1, 100): 1
    /// - [100, 500): 2
    /// - [500, 1000]: 5
    /// - >1000: 10
    ///
    /// # Arguments
    /// * `params` - Request parameters for premium index kline data
    ///
    /// # Returns
    /// A vector of premium index kline tuples, each representing a kline bar
    pub async fn get_premium_index_klines(
        &self,
        params: PremiumIndexKlineRequest,
    ) -> RestResult<PremiumIndexKlineResponse> {
        let weight = match params.limit.unwrap_or(500) {
            1..=99 => 1,
            100..=499 => 2,
            500..=1000 => 5,
            _ => 10,
        };

        self.send_get_request(PREMIUM_INDEX_KLINES_ENDPOINT, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_premium_index_kline_request_serialization() {
        let request = PremiumIndexKlineRequest {
            symbol: "BTCUSD_PERP".to_string(),
            interval: KlineInterval::I1m,
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("interval=1m"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_premium_index_kline_request_serialization_minimal() {
        let request = PremiumIndexKlineRequest {
            symbol: "ETHUSD_PERP".to_string(),
            interval: KlineInterval::I1h,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSD_PERP&interval=1h");
    }

    #[test]
    fn test_premium_index_kline_response_deserialization() {
        let json = r#"[
            [
                1625097600000,
                "0.00010000",
                "0.00015000",
                "0.00008000",
                "0.00012000",
                "0",
                1625101199999,
                "0",
                0,
                "0",
                "0",
                "0"
            ],
            [
                1625101200000,
                "0.00012000",
                "0.00016000",
                "0.00011000",
                "0.00014000",
                "0",
                1625104799999,
                "0",
                0,
                "0",
                "0",
                "0"
            ]
        ]"#;

        let klines: PremiumIndexKlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        let first_kline = &klines[0];
        assert_eq!(first_kline.0, 1625097600000); // Open time
        assert_eq!(first_kline.1, "0.00010000"); // Open
        assert_eq!(first_kline.2, "0.00015000"); // High
        assert_eq!(first_kline.3, "0.00008000"); // Low
        assert_eq!(first_kline.4, "0.00012000"); // Close
        assert_eq!(first_kline.5, "0"); // Ignore
        assert_eq!(first_kline.6, 1625101199999); // Close time
        assert_eq!(first_kline.7, "0"); // Ignore
        assert_eq!(first_kline.8, 0); // Ignore
        assert_eq!(first_kline.9, "0"); // Ignore
        assert_eq!(first_kline.10, "0"); // Ignore
        assert_eq!(first_kline.11, "0"); // Ignore

        let second_kline = &klines[1];
        assert_eq!(second_kline.0, 1625101200000);
        assert_eq!(second_kline.4, "0.00014000"); // Close price
    }

    #[test]
    fn test_premium_index_kline_weight_calculation() {
        let test_cases = vec![
            (Some(50), 1),    // 1-99 -> weight 1
            (Some(99), 1),    // 1-99 -> weight 1
            (Some(100), 2),   // 100-499 -> weight 2
            (Some(499), 2),   // 100-499 -> weight 2
            (Some(500), 5),   // 500-1000 -> weight 5
            (Some(1000), 5),  // 500-1000 -> weight 5
            (Some(1001), 10), // >1000 -> weight 10
            (None, 5),        // default 500 -> weight 5
        ];

        for (limit, expected_weight) in test_cases {
            let weight = match limit.unwrap_or(500) {
                1..=99 => 1,
                100..=499 => 2,
                500..=1000 => 5,
                _ => 10,
            };
            assert_eq!(weight, expected_weight, "Failed for limit: {:?}", limit);
        }
    }

    #[test]
    fn test_premium_index_kline_negative_values() {
        // Test with negative premium index values
        let json = r#"[
            [
                1625097600000,
                "-0.00010000",
                "0.00005000",
                "-0.00020000",
                "-0.00005000",
                "0",
                1625101199999,
                "0",
                0,
                "0",
                "0",
                "0"
            ]
        ]"#;

        let klines: PremiumIndexKlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 1);
        assert_eq!(klines[0].1, "-0.00010000"); // Open can be negative
        assert_eq!(klines[0].3, "-0.00020000"); // Low can be negative
        assert_eq!(klines[0].4, "-0.00005000"); // Close can be negative
    }
}
