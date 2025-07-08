use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const TICKER_ALL_PAIRS_ENDPOINT: &str = "/spot/quotation/v3/tickers";

/// Request parameters for getting ticker of all pairs
#[derive(Debug, Serialize, Default)]
pub struct GetTickerAllPairsRequest {
    // No parameters needed for this endpoint
}

/// Ticker data for a single trading pair (array format)
/// This endpoint returns data as arrays, not objects
pub type TickerData = Vec<String>;

/// Response for ticker of all pairs endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTickerAllPairsResponse(pub Vec<TickerData>);

impl RestClient {
    /// Get Ticker of All Pairs (V3)
    ///
    /// Get all trading pairs with a volume greater than 0 within 24 hours.
    /// Market data includes: latest transaction price, best bid price, best ask price
    /// and 24-hour transaction volume snapshot information.
    ///
    /// Note that the interface is not real-time data, if you need real-time data,
    /// please use websocket to subscribe Ticker channel
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/public_market_data.md
    ///
    /// Rate limit: 10 times/2sec per IP
    ///
    /// # Returns
    /// Ticker data for all trading pairs with volume > 0 in the last 24 hours
    pub async fn get_ticker_all_pairs(
        &self,
        _request: GetTickerAllPairsRequest,
    ) -> RestResult<GetTickerAllPairsResponse> {
        self.send_request(
            TICKER_ALL_PAIRS_ENDPOINT,
            reqwest::Method::GET,
            Option::<&()>::None, // No query parameters
            EndpointType::SpotPublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ticker_all_pairs_request_default() {
        let request = GetTickerAllPairsRequest::default();
        // Request has no fields to check
        let _ = request;
    }

    #[test]
    fn test_ticker_data_parsing() {
        let ticker_data = vec![
            "BTC_USDT".to_string(),      // symbol
            "30000.00".to_string(),      // last
            "582.08066".to_string(),     // v_24h
            "4793098.48".to_string(),    // qv_24h
            "28596.30".to_string(),      // open_24h
            "31012.44".to_string(),      // high_24h
            "12.44".to_string(),         // low_24h
            "0.04909".to_string(),       // fluctuation
            "30000".to_string(),         // bid_px
            "1".to_string(),             // bid_sz
            "31012.44".to_string(),      // ask_px
            "69994.75267".to_string(),   // ask_sz
            "1691671091933".to_string(), // ts
        ];

        assert_eq!(ticker_data.first().map(|s| s.as_str()), Some("BTC_USDT"));
        assert_eq!(ticker_data.get(1).map(|s| s.as_str()), Some("30000.00"));
        assert_eq!(ticker_data.get(2).map(|s| s.as_str()), Some("582.08066"));
        assert_eq!(ticker_data.get(3).map(|s| s.as_str()), Some("4793098.48"));
        assert_eq!(ticker_data.get(4).map(|s| s.as_str()), Some("28596.30"));
        assert_eq!(ticker_data.get(5).map(|s| s.as_str()), Some("31012.44"));
        assert_eq!(ticker_data.get(6).map(|s| s.as_str()), Some("12.44"));
        assert_eq!(ticker_data.get(7).map(|s| s.as_str()), Some("0.04909"));
        assert_eq!(ticker_data.get(8).map(|s| s.as_str()), Some("30000"));
        assert_eq!(ticker_data.get(9).map(|s| s.as_str()), Some("1"));
        assert_eq!(ticker_data.get(10).map(|s| s.as_str()), Some("31012.44"));
        assert_eq!(ticker_data.get(11).map(|s| s.as_str()), Some("69994.75267"));
        assert_eq!(
            ticker_data.get(12).map(|s| s.as_str()),
            Some("1691671091933")
        );
    }

    #[test]
    fn test_ticker_data_incomplete() {
        let ticker_data = ["BTC_USDT".to_string(),
            "30000.00".to_string(),
            "582.08066".to_string()];

        assert_eq!(ticker_data.first().map(|s| s.as_str()), Some("BTC_USDT"));
        assert_eq!(ticker_data.get(1).map(|s| s.as_str()), Some("30000.00"));
        assert_eq!(ticker_data.get(2).map(|s| s.as_str()), Some("582.08066"));
        assert_eq!(ticker_data.get(3).map(|s| s.as_str()), None);
        assert_eq!(ticker_data.get(12).map(|s| s.as_str()), None);
    }

    #[test]
    fn test_get_ticker_all_pairs_response_structure() {
        let response = GetTickerAllPairsResponse(vec![
            vec![
                "BTC_USDT".to_string(),
                "30000.00".to_string(),
                "582.08066".to_string(),
                "4793098.48".to_string(),
                "28596.30".to_string(),
                "31012.44".to_string(),
                "12.44".to_string(),
                "0.04909".to_string(),
                "30000".to_string(),
                "1".to_string(),
                "31012.44".to_string(),
                "69994.75267".to_string(),
                "1691671091933".to_string(),
            ],
            vec![
                "ETH_USDT".to_string(),
                "1840.00".to_string(),
                "2.00000".to_string(),
                "3680.00".to_string(),
                "1842.18".to_string(),
                "1842.18".to_string(),
                "1840.00".to_string(),
                "-0.00118".to_string(),
                "1812.35".to_string(),
                "4.61989".to_string(),
                "1859.34".to_string(),
                "4.07793".to_string(),
                "1691671094213".to_string(),
            ],
        ]);

        assert_eq!(response.0.len(), 2);
        assert_eq!(response.0[0].first().map(|s| s.as_str()), Some("BTC_USDT"));
        assert_eq!(response.0[1].first().map(|s| s.as_str()), Some("ETH_USDT"));
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"[
            [
                "BTC_USDT",
                "30000.00",
                "582.08066",
                "4793098.48",
                "28596.30",
                "31012.44",
                "12.44",
                "0.04909",
                "30000",
                "1",
                "31012.44",
                "69994.75267",
                "1691671091933"
            ],
            [
                "ETH_USDT",
                "1840.00",
                "2.00000",
                "3680.00",
                "1842.18",
                "1842.18",
                "1840.00",
                "-0.00118",
                "1812.35",
                "4.61989",
                "1859.34",
                "4.07793",
                "1691671094213"
            ]
        ]"#;

        let response: GetTickerAllPairsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.0.len(), 2);
        assert_eq!(response.0[0].first().map(|s| s.as_str()), Some("BTC_USDT"));
        assert_eq!(response.0[0].get(1).map(|s| s.as_str()), Some("30000.00"));
        assert_eq!(response.0[1].first().map(|s| s.as_str()), Some("ETH_USDT"));
        assert_eq!(response.0[1].get(1).map(|s| s.as_str()), Some("1840.00"));
    }

    #[test]
    fn test_empty_response() {
        let json = r#"[]"#;

        let response: GetTickerAllPairsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.0.len(), 0);
    }
}
