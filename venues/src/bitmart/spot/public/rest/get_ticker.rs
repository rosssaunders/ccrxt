use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const TICKER_ENDPOINT: &str = "/spot/quotation/v3/ticker";

/// Request parameters for getting ticker of a trading pair
#[derive(Debug, Serialize)]
pub struct GetTickerRequest {
    /// Trading pair (e.g. BMX_USDT)
    pub symbol: String,
}

/// Ticker data for a single trading pair (object format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerData {
    /// Trading pair
    pub symbol: String,
    /// Latest price
    pub last: String,
    /// 24-hour trade volume in base currency
    pub v_24h: String,
    /// 24-hour trade volume in quote currency
    pub qv_24h: String,
    /// 24-hour open price
    pub open_24h: String,
    /// 24-hour highest price
    pub high_24h: String,
    /// 24-hour lowest price
    pub low_24h: String,
    /// 24-hour price change
    pub fluctuation: String,
    /// top buy price
    pub bid_px: String,
    /// Size of top buy order
    pub bid_sz: String,
    /// top sell price
    pub ask_px: String,
    /// Size of top sell order
    pub ask_sz: String,
    /// Time of generation(in milliseconds)
    pub ts: String,
}

/// Response for ticker of a trading pair endpoint
pub type GetTickerResponse = TickerData;

impl RestClient {
    /// Get Ticker of a Trading Pair (V3)
    ///
    /// Applicable to query the aggregated market price of a certain trading pair,
    /// and return the latest ticker information.
    ///
    /// Note that the interface is not real-time data, if you need real-time data,
    /// please use websocket to subscribe Ticker channel
    ///
    /// [docs]: https://developer-pro.bitmart.com/en/spot/#get-ticker-of-a-trading-pair-v3
    ///
    /// Rate limit: 10 times/2sec per IP
    ///
    /// # Arguments
    /// * `request` - The request parameters including the symbol
    ///
    /// # Returns
    /// Ticker data for the specified trading pair
    ///
    /// # Notes
    /// 1. If no corresponding trading pair is found, this trading pair has been delisted.
    /// 2. For frequent query needs, we recommend using this endpoint to obtain aggregated ticker for a single trading pair.
    pub async fn get_ticker(&self, request: GetTickerRequest) -> RestResult<GetTickerResponse> {
        self.send_request(
            TICKER_ENDPOINT,
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
    fn test_get_ticker_request() {
        let request = GetTickerRequest {
            symbol: "BTC_USDT".to_string(),
        };

        assert_eq!(request.symbol, "BTC_USDT");
    }

    #[test]
    fn test_ticker_data_structure() {
        let ticker = TickerData {
            symbol: "BTC_USDT".to_string(),
            last: "30000.00".to_string(),
            v_24h: "582.08066".to_string(),
            qv_24h: "4793098.48".to_string(),
            open_24h: "28596.30".to_string(),
            high_24h: "31012.44".to_string(),
            low_24h: "12.44".to_string(),
            fluctuation: "0.04909".to_string(),
            bid_px: "30000".to_string(),
            bid_sz: "1".to_string(),
            ask_px: "31012.44".to_string(),
            ask_sz: "69994.75267".to_string(),
            ts: "1691671061919".to_string(),
        };

        assert_eq!(ticker.symbol, "BTC_USDT");
        assert_eq!(ticker.last, "30000.00");
        assert_eq!(ticker.v_24h, "582.08066");
        assert_eq!(ticker.qv_24h, "4793098.48");
        assert_eq!(ticker.open_24h, "28596.30");
        assert_eq!(ticker.high_24h, "31012.44");
        assert_eq!(ticker.low_24h, "12.44");
        assert_eq!(ticker.fluctuation, "0.04909");
        assert_eq!(ticker.bid_px, "30000");
        assert_eq!(ticker.bid_sz, "1");
        assert_eq!(ticker.ask_px, "31012.44");
        assert_eq!(ticker.ask_sz, "69994.75267");
        assert_eq!(ticker.ts, "1691671061919");
    }

    #[test]
    fn test_ticker_data_serialization_roundtrip() {
        let ticker = TickerData {
            symbol: "ETH_USDT".to_string(),
            last: "1840.00".to_string(),
            v_24h: "2.00000".to_string(),
            qv_24h: "3680.00".to_string(),
            open_24h: "1842.18".to_string(),
            high_24h: "1842.18".to_string(),
            low_24h: "1840.00".to_string(),
            fluctuation: "-0.00118".to_string(),
            bid_px: "1812.35".to_string(),
            bid_sz: "4.61989".to_string(),
            ask_px: "1859.34".to_string(),
            ask_sz: "4.07793".to_string(),
            ts: "1691671094213".to_string(),
        };

        let serialized = serde_json::to_string(&ticker).expect("Failed to serialize ticker");
        let deserialized: TickerData =
            serde_json::from_str(&serialized).expect("Failed to deserialize ticker");

        assert_eq!(ticker.symbol, deserialized.symbol);
        assert_eq!(ticker.last, deserialized.last);
        assert_eq!(ticker.v_24h, deserialized.v_24h);
        assert_eq!(ticker.qv_24h, deserialized.qv_24h);
        assert_eq!(ticker.open_24h, deserialized.open_24h);
        assert_eq!(ticker.high_24h, deserialized.high_24h);
        assert_eq!(ticker.low_24h, deserialized.low_24h);
        assert_eq!(ticker.fluctuation, deserialized.fluctuation);
        assert_eq!(ticker.bid_px, deserialized.bid_px);
        assert_eq!(ticker.bid_sz, deserialized.bid_sz);
        assert_eq!(ticker.ask_px, deserialized.ask_px);
        assert_eq!(ticker.ask_sz, deserialized.ask_sz);
        assert_eq!(ticker.ts, deserialized.ts);
    }

    #[test]
    fn test_request_serialization() {
        let request = GetTickerRequest {
            symbol: "BMX_USDT".to_string(),
        };

        let serialized =
            serde_urlencoded::to_string(&request).expect("Failed to serialize request");
        assert!(serialized.contains("symbol=BMX_USDT"));
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "symbol": "BTC_USDT",
            "last": "30000.00",
            "v_24h": "582.08066",
            "qv_24h": "4793098.48",
            "open_24h": "28596.30",
            "high_24h": "31012.44",
            "low_24h": "12.44",
            "fluctuation": "0.04909",
            "bid_px": "30000",
            "bid_sz": "1",
            "ask_px": "31012.44",
            "ask_sz": "69994.75267",
            "ts": "1691671061919"
        }"#;

        let response: GetTickerResponse =
            serde_json::from_str(json).expect("Failed to deserialize response");
        assert_eq!(response.symbol, "BTC_USDT");
        assert_eq!(response.last, "30000.00");
        assert_eq!(response.v_24h, "582.08066");
        assert_eq!(response.qv_24h, "4793098.48");
        assert_eq!(response.open_24h, "28596.30");
        assert_eq!(response.high_24h, "31012.44");
        assert_eq!(response.low_24h, "12.44");
        assert_eq!(response.fluctuation, "0.04909");
        assert_eq!(response.bid_px, "30000");
        assert_eq!(response.bid_sz, "1");
        assert_eq!(response.ask_px, "31012.44");
        assert_eq!(response.ask_sz, "69994.75267");
        assert_eq!(response.ts, "1691671061919");
    }
}
