use serde::{Deserialize, Serialize};

use super::client::RestClient;

use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;

const LATEST_KLINE_ENDPOINT: &str = "/spot/quotation/v3/lite-klines";

/// Request parameters for getting latest K-line
#[derive(Debug, Serialize)]
pub struct GetLatestKlineRequest {
    /// Trading pair (e.g. BMX_USDT)
    pub symbol: String,
    /// Query timestamp (unit: second, e.g. 1525760116), query the data before this time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<i64>,
    /// Query timestamp (unit: second, e.g. 1525769116), query the data after this time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<i64>,
    /// k-line step, value [1, 5, 15, 30, 60, 120, 240, 1440, 10080, 43200] unit: minute, default 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
    /// Return number, the maximum value is 200, default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// K-line data (array format)
/// [timestamp, open, high, low, close, volume, quote_volume]
pub type KlineData = Vec<String>;

/// Response for latest K-line endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLatestKlineResponse(pub Vec<KlineData>);

impl RestClient {
    /// Get Latest K-Line (V3)
    ///
    /// Query the latest K-line and return a maximum of 1000 data.
    /// Note that the latest K-line of the interface is not real-time data.
    /// If you want real-time data, please use websocket to subscribe to K-line channel
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/public_market_data.md
    ///
    /// Rate limit: 10 times/2sec per IP
    ///
    /// # Arguments
    /// * `request` - The request parameters including symbol and optional filters
    ///
    /// # Returns
    /// Latest K-line data for the specified trading pair
    ///
    /// # Query Modes
    /// A total of four query modes are supported:
    /// 1. If only before is passed, check forward according to the time
    /// 2. If only after is passed, check backward according to the time
    /// 3. Both before and after need to verify whether the time interval is legal, and if it is legal, check the interval
    /// 4. If neither before nor after is passed, the latest K-line will be returned in reverse order
    pub async fn get_latest_kline(
        &self,
        request: GetLatestKlineRequest,
    ) -> RestResult<GetLatestKlineResponse> {
        self.send_request(
            LATEST_KLINE_ENDPOINT,
            reqwest::Method::GET,
            Some(&request),
            EndpointType::SpotPublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_latest_kline_request() {
        let request = GetLatestKlineRequest {
            symbol: "BMX_ETH".to_string(),
            before: None,
            after: None,
            step: Some(15),
            limit: Some(10),
        };

        assert_eq!(request.symbol, "BMX_ETH");
        assert_eq!(request.before, None);
        assert_eq!(request.after, None);
        assert_eq!(request.step, Some(15));
        assert_eq!(request.limit, Some(10));
    }

    #[test]
    fn test_get_latest_kline_request_with_timestamps() {
        let request = GetLatestKlineRequest {
            symbol: "BTC_USDT".to_string(),
            before: Some(1525760116),
            after: Some(1525769116),
            step: Some(60),
            limit: Some(200),
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.before, Some(1525760116));
        assert_eq!(request.after, Some(1525769116));
        assert_eq!(request.step, Some(60));
        assert_eq!(request.limit, Some(200));
    }

    #[test]
    fn test_kline_data_parsing() {
        let kline_data = vec![
            "1689736680".to_string(),        // t
            "3.721".to_string(),             // o
            "3.743".to_string(),             // h
            "3.677".to_string(),             // l
            "3.708".to_string(),             // c
            "22698348.04828491".to_string(), // v
            "12698348.04828491".to_string(), // qv
        ];

        assert_eq!(kline_data.first().map(|s| s.as_str()), Some("1689736680"));
        assert_eq!(kline_data.get(1).map(|s| s.as_str()), Some("3.721"));
        assert_eq!(kline_data.get(2).map(|s| s.as_str()), Some("3.743"));
        assert_eq!(kline_data.get(3).map(|s| s.as_str()), Some("3.677"));
        assert_eq!(kline_data.get(4).map(|s| s.as_str()), Some("3.708"));
        assert_eq!(
            kline_data.get(5).map(|s| s.as_str()),
            Some("22698348.04828491")
        );
        assert_eq!(
            kline_data.get(6).map(|s| s.as_str()),
            Some("12698348.04828491")
        );
    }

    #[test]
    fn test_kline_data_incomplete() {
        let kline_data = vec![
            "1689736680".to_string(),
            "3.721".to_string(),
            "3.743".to_string(),
        ];

        assert_eq!(kline_data.first().map(|s| s.as_str()), Some("1689736680"));
        assert_eq!(kline_data.get(1).map(|s| s.as_str()), Some("3.721"));
        assert_eq!(kline_data.get(2).map(|s| s.as_str()), Some("3.743"));
        assert_eq!(kline_data.get(3).map(|s| s.as_str()), None);
        assert_eq!(kline_data.get(4).map(|s| s.as_str()), None);
        assert_eq!(kline_data.get(5).map(|s| s.as_str()), None);
        assert_eq!(kline_data.get(6).map(|s| s.as_str()), None);
    }

    #[test]
    fn test_get_latest_kline_response_structure() {
        let response = GetLatestKlineResponse(vec![
            vec![
                "1689736680".to_string(),
                "3.721".to_string(),
                "3.743".to_string(),
                "3.677".to_string(),
                "3.708".to_string(),
                "22698348.04828491".to_string(),
                "12698348.04828491".to_string(),
            ],
            vec![
                "1689736620".to_string(),
                "3.731".to_string(),
                "3.799".to_string(),
                "3.494".to_string(),
                "3.72".to_string(),
                "67632347.24399722".to_string(),
                "37632347.24399722".to_string(),
            ],
        ]);

        assert_eq!(response.0.len(), 2);
        assert_eq!(
            response.0[0].first().map(|s| s.as_str()),
            Some("1689736680")
        );
        assert_eq!(
            response.0[1].first().map(|s| s.as_str()),
            Some("1689736620")
        );
    }

    #[test]
    fn test_request_serialization() {
        let request = GetLatestKlineRequest {
            symbol: "BMX_ETH".to_string(),
            before: None,
            after: None,
            step: Some(15),
            limit: Some(10),
        };

        let serialized =
            serde_urlencoded::to_string(&request).expect("Failed to serialize request");
        assert!(serialized.contains("symbol=BMX_ETH"));
        assert!(serialized.contains("step=15"));
        assert!(serialized.contains("limit=10"));
        assert!(!serialized.contains("before"));
        assert!(!serialized.contains("after"));
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"[
            [
                "1689736680",
                "3.721",
                "3.743",
                "3.677",
                "3.708",
                "22698348.04828491",
                "12698348.04828491"
            ],
            [
                "1689736620",
                "3.731",
                "3.799",
                "3.494",
                "3.72",
                "67632347.24399722",
                "37632347.24399722"
            ]
        ]"#;

        let response: GetLatestKlineResponse =
            serde_json::from_str(json).expect("Failed to deserialize response");
        assert_eq!(response.0.len(), 2);
        assert_eq!(
            response.0[0].first().map(|s| s.as_str()),
            Some("1689736680")
        );
        assert_eq!(response.0[0].get(1).map(|s| s.as_str()), Some("3.721"));
        assert_eq!(
            response.0[1].first().map(|s| s.as_str()),
            Some("1689736620")
        );
        assert_eq!(response.0[1].get(1).map(|s| s.as_str()), Some("3.731"));
    }

    #[test]
    fn test_empty_response() {
        let json = r#"[]"#;

        let response: GetLatestKlineResponse =
            serde_json::from_str(json).expect("Failed to deserialize response");
        assert_eq!(response.0.len(), 0);
    }

    #[test]
    fn test_valid_kline_steps() {
        let valid_steps = [1, 5, 15, 30, 60, 120, 240, 1440, 10080, 43200];

        for step in valid_steps {
            let request = GetLatestKlineRequest {
                symbol: "BTC_USDT".to_string(),
                before: None,
                after: None,
                step: Some(step),
                limit: None,
            };
            assert_eq!(request.step, Some(step));
        }
    }
}
