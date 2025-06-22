use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;

/// Request parameters for getting history K-line
#[derive(Debug, Serialize)]
pub struct GetHistoryKlineRequest {
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

/// Response for history K-line endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHistoryKlineResponse(pub Vec<KlineData>);

impl GetHistoryKlineResponse {
    /// Get the create timestamp (in seconds) from K-line data
    /// It can be used as the unique identification of K line
    pub fn timestamp(kline: &KlineData) -> Option<&str> {
        kline.first().map(|s| s.as_str())
    }

    /// Get the open price from K-line data
    pub fn open_price(kline: &KlineData) -> Option<&str> {
        kline.get(1).map(|s| s.as_str())
    }

    /// Get the highest price from K-line data
    pub fn high_price(kline: &KlineData) -> Option<&str> {
        kline.get(2).map(|s| s.as_str())
    }

    /// Get the lowest price from K-line data
    pub fn low_price(kline: &KlineData) -> Option<&str> {
        kline.get(3).map(|s| s.as_str())
    }

    /// Get the close price from K-line data
    pub fn close_price(kline: &KlineData) -> Option<&str> {
        kline.get(4).map(|s| s.as_str())
    }

    /// Get the trading volume from K-line data
    /// With a unit of currency (If in BTC_USDT, The unit is BTC)
    pub fn volume(kline: &KlineData) -> Option<&str> {
        kline.get(5).map(|s| s.as_str())
    }

    /// Get the trading volume in quote currency from K-line data
    /// The value is the quantity in quote currency (If in BTC_USDT, The unit is USDT)
    pub fn quote_volume(kline: &KlineData) -> Option<&str> {
        kline.get(6).map(|s| s.as_str())
    }
}

impl RestClient {
    /// Get History K-Line (V3)
    ///
    /// Get k-line data within a specified time range of a specified trading pair.
    /// Note that the interface is not real-time data, if you need real-time data,
    /// please use websocket to subscribe KLine channel
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/public_market_data.md
    ///
    /// Rate limit: 10 times/2sec per IP
    ///
    /// # Arguments
    /// * `request` - The request parameters including symbol and optional filters
    ///
    /// # Returns
    /// Historical K-line data for the specified trading pair within the specified time range
    ///
    /// # Query Modes
    /// A total of four query modes are supported:
    /// 1. If only before is passed, check forward according to the time
    /// 2. If only after is passed, check backward according to the time
    /// 3. Both before and after need to verify whether the time interval is legal, and if it is legal, check the interval
    /// 4. If neither before nor after is passed, the latest K-line will be returned in reverse order
    pub async fn get_history_kline(&self, request: GetHistoryKlineRequest) -> RestResult<GetHistoryKlineResponse> {
        self.send_request(
            "/spot/quotation/v3/klines",
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
    fn test_get_history_kline_request() {
        let request = GetHistoryKlineRequest {
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
    fn test_get_history_kline_request_with_timestamps() {
        let request = GetHistoryKlineRequest {
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

        assert_eq!(
            GetHistoryKlineResponse::timestamp(&kline_data),
            Some("1689736680")
        );
        assert_eq!(
            GetHistoryKlineResponse::open_price(&kline_data),
            Some("3.721")
        );
        assert_eq!(
            GetHistoryKlineResponse::high_price(&kline_data),
            Some("3.743")
        );
        assert_eq!(
            GetHistoryKlineResponse::low_price(&kline_data),
            Some("3.677")
        );
        assert_eq!(
            GetHistoryKlineResponse::close_price(&kline_data),
            Some("3.708")
        );
        assert_eq!(
            GetHistoryKlineResponse::volume(&kline_data),
            Some("22698348.04828491")
        );
        assert_eq!(
            GetHistoryKlineResponse::quote_volume(&kline_data),
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

        assert_eq!(
            GetHistoryKlineResponse::timestamp(&kline_data),
            Some("1689736680")
        );
        assert_eq!(
            GetHistoryKlineResponse::open_price(&kline_data),
            Some("3.721")
        );
        assert_eq!(
            GetHistoryKlineResponse::high_price(&kline_data),
            Some("3.743")
        );
        assert_eq!(GetHistoryKlineResponse::low_price(&kline_data), None);
        assert_eq!(GetHistoryKlineResponse::close_price(&kline_data), None);
        assert_eq!(GetHistoryKlineResponse::volume(&kline_data), None);
        assert_eq!(GetHistoryKlineResponse::quote_volume(&kline_data), None);
    }

    #[test]
    fn test_get_history_kline_response_structure() {
        let response = GetHistoryKlineResponse(vec![
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
            GetHistoryKlineResponse::timestamp(&response.0[0]),
            Some("1689736680")
        );
        assert_eq!(
            GetHistoryKlineResponse::timestamp(&response.0[1]),
            Some("1689736620")
        );
    }

    #[test]
    fn test_request_serialization() {
        let request = GetHistoryKlineRequest {
            symbol: "BMX_ETH".to_string(),
            before: None,
            after: None,
            step: Some(15),
            limit: Some(10),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
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

        let response: GetHistoryKlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.0.len(), 2);
        assert_eq!(
            GetHistoryKlineResponse::timestamp(&response.0[0]),
            Some("1689736680")
        );
        assert_eq!(
            GetHistoryKlineResponse::open_price(&response.0[0]),
            Some("3.721")
        );
        assert_eq!(
            GetHistoryKlineResponse::timestamp(&response.0[1]),
            Some("1689736620")
        );
        assert_eq!(
            GetHistoryKlineResponse::open_price(&response.0[1]),
            Some("3.731")
        );
    }

    #[test]
    fn test_empty_response() {
        let json = r#"[]"#;

        let response: GetHistoryKlineResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.0.len(), 0);
    }

    #[test]
    fn test_valid_kline_steps() {
        let valid_steps = [1, 5, 15, 30, 60, 120, 240, 1440, 10080, 43200];

        for step in valid_steps {
            let request = GetHistoryKlineRequest {
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
