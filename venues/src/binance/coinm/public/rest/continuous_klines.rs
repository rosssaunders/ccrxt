use serde::{Deserialize, Serialize};

use crate::binance::coinm::{
    RestResult,
    enums::{ContractType, KlineInterval},
    public_client::RestClient,
};

const CONTINUOUS_KLINES_ENDPOINT: &str = "/dapi/v1/continuousKlines";

/// Request parameters for the continuous contract kline/candlestick data endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct ContinuousKlineRequest {
    /// Trading pair (e.g., "BTCUSD").
    #[serde(rename = "pair")]
    pub pair: String,

    /// Contract type.
    #[serde(rename = "contractType")]
    pub contract_type: ContractType,

    /// Kline interval.
    #[serde(rename = "interval")]
    pub interval: KlineInterval,

    /// Start time in milliseconds.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 500; max 1500.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single continuous contract kline/candlestick.
///
/// Klines are arrays with the following structure:
/// [Open time, Open, High, Low, Close, Volume, Close time, Base asset volume, Number of trades, Taker buy volume, Taker buy base asset volume, Ignore]
#[derive(Debug, Clone, Deserialize)]
pub struct ContinuousKline(
    pub u64,    // Open time
    pub String, // Open
    pub String, // High
    pub String, // Low
    pub String, // Close (or latest price)
    pub String, // Volume
    pub u64,    // Close time
    pub String, // Base asset volume
    pub u64,    // Number of trades
    pub String, // Taker buy volume
    pub String, // Taker buy base asset volume
    pub String, // Ignore
);

/// Response from the continuous contract kline/candlestick data endpoint.
pub type ContinuousKlineResponse = Vec<ContinuousKline>;

impl RestClient {
    /// Continuous contract kline/candlestick data
    ///
    /// Kline/candlestick bars for a specific contract type. Klines are uniquely identified by their open time.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data)
    ///
    /// Rate limit: Weight based on parameter LIMIT:
    /// - \[1,100): 1
    /// - \[100, 500): 2
    /// - \[500, 1000]: 5
    /// - > 1000: 10
    ///
    /// # Arguments
    /// * `params` - Request parameters including pair, contract type, interval, and optional time range
    ///
    /// # Returns
    /// List of continuous contract kline/candlestick data
    pub async fn get_continuous_klines(
        &self,
        params: ContinuousKlineRequest,
    ) -> RestResult<ContinuousKlineResponse> {
        let weight = match params.limit.unwrap_or(500) {
            1..=99 => 1,
            100..=499 => 2,
            500..=1000 => 5,
            _ => 10,
        };

        self.send_get_request(CONTINUOUS_KLINES_ENDPOINT, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continuous_kline_request_serialization() {
        let request = ContinuousKlineRequest {
            pair: "BTCUSD".to_string(),
            contract_type: ContractType::Perpetual,
            interval: KlineInterval::I1m,
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=BTCUSD"));
        assert!(serialized.contains("contractType=PERPETUAL"));
        assert!(serialized.contains("interval=1m"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_continuous_kline_request_serialization_minimal() {
        let request = ContinuousKlineRequest {
            pair: "ETHUSD".to_string(),
            contract_type: ContractType::CurrentQuarter,
            interval: KlineInterval::I1h,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=ETHUSD"));
        assert!(serialized.contains("contractType=CURRENT_QUARTER"));
        assert!(serialized.contains("interval=1h"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_continuous_kline_response_deserialization() {
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

        let klines: ContinuousKlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        let first_kline = &klines[0];
        assert_eq!(first_kline.0, 1625097600000); // Open time
        assert_eq!(first_kline.1, "45000.00"); // Open
        assert_eq!(first_kline.2, "45500.00"); // High
        assert_eq!(first_kline.3, "44800.00"); // Low
        assert_eq!(first_kline.4, "45200.00"); // Close
        assert_eq!(first_kline.5, "1250.500"); // Volume
        assert_eq!(first_kline.6, 1625101199999); // Close time
        assert_eq!(first_kline.7, "56475000.00"); // Base asset volume
        assert_eq!(first_kline.8, 2500); // Number of trades
        assert_eq!(first_kline.9, "625.250"); // Taker buy volume
        assert_eq!(first_kline.10, "28237500.00"); // Taker buy base asset volume
        assert_eq!(first_kline.11, "0"); // Ignore
    }

    #[test]
    fn test_continuous_kline_weight_calculation() {
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
    fn test_continuous_kline_contract_types() {
        // Test different contract types serialize correctly
        let perpetual_request = ContinuousKlineRequest {
            pair: "BTCUSD".to_string(),
            contract_type: ContractType::Perpetual,
            interval: KlineInterval::I1m,
            start_time: None,
            end_time: None,
            limit: None,
        };
        let serialized = serde_urlencoded::to_string(&perpetual_request).unwrap();
        assert!(serialized.contains("contractType=PERPETUAL"));

        let next_quarter_request = ContinuousKlineRequest {
            pair: "BTCUSD".to_string(),
            contract_type: ContractType::NextQuarter,
            interval: KlineInterval::I1m,
            start_time: None,
            end_time: None,
            limit: None,
        };
        let serialized = serde_urlencoded::to_string(&next_quarter_request).unwrap();
        assert!(serialized.contains("contractType=NEXT_QUARTER"));
    }
}
