use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, enums::KlineInterval, public::rest::RestClient};

const INDEX_PRICE_KLINES_ENDPOINT: &str = "/dapi/v1/indexPriceKlines";

/// Request parameters for the index price kline/candlestick data endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceKlineRequest {
    /// Trading pair (e.g., "BTCUSD").
    pub pair: String,

    /// Kline interval.
    pub interval: KlineInterval,

    /// Start time in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 500; max 1500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single index price kline/candlestick.
///
/// Klines are arrays with the following structure:
/// [Open time, Open, High, Low, Close, Ignore, Close time, Ignore, Number of basic data, Ignore, Ignore, Ignore]
#[derive(Debug, Clone, Deserialize)]
pub struct IndexPriceKline(
    /// Open time
    pub u64,
    /// Open
    pub String,
    /// High
    pub String,
    /// Low
    pub String,
    /// Close (or latest price)
    pub String,
    /// Ignore
    pub String,
    /// Close time
    pub u64,
    /// Ignore
    pub String,
    /// Number of basic data
    pub u64,
    /// Ignore
    pub String,
    /// Ignore
    pub String,
    /// Ignore
    pub String,
);

/// Response from the index price kline/candlestick data endpoint.
pub type IndexPriceKlineResponse = Vec<IndexPriceKline>;

impl RestClient {
    /// Index Price Kline/Candlestick Data
    ///
    /// Kline/candlestick bars for the index price of a pair. Klines are uniquely identified by their open time.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Index-Price-Kline-Candlestick-Data)
    ///
    /// Rate limit: based on parameter LIMIT:
    /// - [1,100): 1
    /// - [100, 500): 2
    /// - [500, 1000]: 5
    /// - > 1000: 10
    ///
    /// # Arguments
    /// * `params` - The index price kline request parameters
    ///
    /// # Returns
    /// Vector of index price klines for the specified pair and interval
    pub async fn get_index_price_klines(
        &self,
        params: IndexPriceKlineRequest,
    ) -> RestResult<IndexPriceKlineResponse> {
        let weight = match params.limit.unwrap_or(500) {
            1..=99 => 1,
            100..=499 => 2,
            500..=1000 => 5,
            _ => 10,
        };

        self.send_get_request(INDEX_PRICE_KLINES_ENDPOINT, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_price_kline_request_serialization() {
        let request = IndexPriceKlineRequest {
            pair: "BTCUSD".to_string(),
            interval: KlineInterval::I1m,
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=BTCUSD"));
        assert!(serialized.contains("interval=1m"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_index_price_kline_request_serialization_minimal() {
        let request = IndexPriceKlineRequest {
            pair: "ETHUSD".to_string(),
            interval: KlineInterval::I1h,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "pair=ETHUSD&interval=1h");
    }

    #[test]
    fn test_index_price_kline_response_deserialization() {
        let json = r#"[
            [
                1625097600000,
                "45000.00",
                "45500.00",
                "44800.00",
                "45200.00",
                "0",
                1625101199999,
                "0",
                5,
                "0",
                "0",
                "0"
            ],
            [
                1625101200000,
                "45200.00",
                "45600.00",
                "45100.00",
                "45400.00",
                "0",
                1625104799999,
                "0",
                7,
                "0",
                "0",
                "0"
            ]
        ]"#;

        let klines: IndexPriceKlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        let first_kline = &klines[0];
        assert_eq!(first_kline.0, 1625097600000); // Open time
        assert_eq!(first_kline.1, "45000.00"); // Open
        assert_eq!(first_kline.2, "45500.00"); // High
        assert_eq!(first_kline.3, "44800.00"); // Low
        assert_eq!(first_kline.4, "45200.00"); // Close
        assert_eq!(first_kline.5, "0"); // Ignore
        assert_eq!(first_kline.6, 1625101199999); // Close time
        assert_eq!(first_kline.7, "0"); // Ignore
        assert_eq!(first_kline.8, 5); // Number of basic data
        assert_eq!(first_kline.9, "0"); // Ignore
        assert_eq!(first_kline.10, "0"); // Ignore
        assert_eq!(first_kline.11, "0"); // Ignore

        let second_kline = &klines[1];
        assert_eq!(second_kline.0, 1625101200000);
        assert_eq!(second_kline.4, "45400.00"); // Close price
        assert_eq!(second_kline.8, 7); // Number of basic data
    }

    #[test]
    fn test_index_price_kline_weight_calculation() {
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
    fn test_index_price_kline_different_intervals() {
        let intervals = vec![
            KlineInterval::I1m,
            KlineInterval::I5m,
            KlineInterval::I15m,
            KlineInterval::I30m,
            KlineInterval::I1h,
            KlineInterval::I4h,
            KlineInterval::I1d,
            KlineInterval::I1w,
            KlineInterval::I1M,
        ];

        for interval in intervals {
            let request = IndexPriceKlineRequest {
                pair: "BTCUSD".to_string(),
                interval,
                start_time: None,
                end_time: None,
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("interval={}", interval)));
        }
    }
}
