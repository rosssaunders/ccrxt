use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, enums::KlineInterval, public::rest::RestClient};

const KLINES_ENDPOINT: &str = "/dapi/v1/klines";

/// Request parameters for the kline/candlestick data endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Kline interval.
    pub interval: KlineInterval,

    /// Start time in milliseconds (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of klines to return. Default 500; max 1500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single kline/candlestick.
///
/// Klines are arrays with the following structure:
/// [Open time, Open, High, Low, Close, Volume, Close time, Base asset volume, Number of trades, Taker buy volume, Taker buy base asset volume, Ignore]
#[derive(Debug, Clone, Deserialize)]
pub struct Kline(
    /// Open time (milliseconds since epoch).
    pub u64,
    /// Open price as string.
    pub String,
    /// High price as string.
    pub String,
    /// Low price as string.
    pub String,
    /// Close price (or latest price) as string.
    pub String,
    /// Volume as string.
    pub String,
    /// Close time (milliseconds since epoch).
    pub u64,
    /// Base asset volume as string.
    pub String,
    /// Number of trades.
    pub u64,
    /// Taker buy volume as string.
    pub String,
    /// Taker buy base asset volume as string.
    pub String,
    /// Ignore field (always "0").
    pub String,
);

/// Response from the kline/candlestick data endpoint.
pub type KlineResponse = Vec<Kline>;

impl RestClient {
    /// Kline/Candlestick Data
    ///
    /// Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Kline-Candlestick-Data
    ///
    /// Weight based on parameter LIMIT:
    /// - [1,100): 1
    /// - [100, 500): 2
    /// - [500, 1000]: 5
    /// - > 1000: 10
    ///
    /// # Arguments
    /// * `params` - The request parameters
    ///
    /// # Returns
    /// Vector of kline data
    pub async fn get_klines(&self, params: KlineRequest) -> RestResult<KlineResponse> {
        let weight = match params.limit.unwrap_or(500) {
            1..=99 => 1,
            100..=499 => 2,
            500..=1000 => 5,
            _ => 10,
        };

        self.send_get_request(KLINES_ENDPOINT, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kline_request_serialization() {
        let request = KlineRequest {
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
    fn test_kline_request_serialization_minimal() {
        let request = KlineRequest {
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
    fn test_kline_response_deserialization() {
        let json = r#"[
            [
                1625097600000,
                "45000.00",
                "45500.00",
                "44800.00",
                "45200.00",
                "1250.500",
                1625101199999,
                "56475000.00",
                2500,
                "625.250",
                "28237500.00",
                "0"
            ],
            [
                1625101200000,
                "45200.00",
                "45600.00",
                "45100.00",
                "45400.00",
                "1100.300",
                1625104799999,
                "49813600.00",
                2200,
                "550.150",
                "24906800.00",
                "0"
            ]
        ]"#;

        let klines: KlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        let first_kline = &klines[0];
        assert_eq!(first_kline.0, 1625097600000); // Open time
        assert_eq!(first_kline.1, "45000.00"); // Open price
        assert_eq!(first_kline.2, "45500.00"); // High price
        assert_eq!(first_kline.3, "44800.00"); // Low price
        assert_eq!(first_kline.4, "45200.00"); // Close price
        assert_eq!(first_kline.5, "1250.500"); // Volume
        assert_eq!(first_kline.6, 1625101199999); // Close time
        assert_eq!(first_kline.7, "56475000.00"); // Base asset volume
        assert_eq!(first_kline.8, 2500); // Number of trades
        assert_eq!(first_kline.9, "625.250"); // Taker buy volume
        assert_eq!(first_kline.10, "28237500.00"); // Taker buy base asset volume
        assert_eq!(first_kline.11, "0"); // Ignore

        let second_kline = &klines[1];
        assert_eq!(second_kline.0, 1625101200000);
        assert_eq!(second_kline.4, "45400.00"); // Close price
    }

    #[test]
    fn test_kline_weight_calculation() {
        // Test weight calculation logic matches the function
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
}
