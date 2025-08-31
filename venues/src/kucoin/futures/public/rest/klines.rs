use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::kucoin::futures::{ResponseHeaders, RestResponse, Result, public_client::RestClient};

// API endpoints
const KLINES_ENDPOINT: &str = "/api/v1/kline/query";

/// Kline granularity (candlestick timeframe)
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(i32)]
pub enum KlineGranularity {
    /// 1 minute
    Min1 = 1,
    /// 5 minutes
    Min5 = 5,
    /// 15 minutes
    Min15 = 15,
    /// 30 minutes
    Min30 = 30,
    /// 1 hour
    Hour1 = 60,
    /// 2 hours
    Hour2 = 120,
    /// 4 hours
    Hour4 = 240,
    /// 8 hours
    Hour8 = 480,
    /// 12 hours
    Hour12 = 720,
    /// 1 day
    Day1 = 1440,
    /// 1 week
    Week1 = 10080,
}

/// Get klines request
#[derive(Debug, Clone, Serialize)]
pub struct GetKlinesRequest {
    /// Symbol of the contract (can also be index symbols like .KXBTUSDT, .XBTUSDTMPI, .XBTUSDTMPI8H)
    pub symbol: String,

    /// Kline granularity
    #[serde(serialize_with = "serialize_granularity")]
    pub granularity: KlineGranularity,

    /// Start time (milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

fn serialize_granularity<S>(
    granularity: &KlineGranularity,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&(*granularity as i32).to_string())
}

/// Kline data
/// Format: [time, open, high, low, close, volume]
pub type Kline = [f64; 6];

/// Response for getting klines
pub type GetKlinesResponse = Vec<Kline>;

impl RestClient {
    /// Get kline/candlestick data for a symbol
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-klines)
    pub async fn get_klines(
        &self,
        request: GetKlinesRequest,
    ) -> Result<(RestResponse<GetKlinesResponse>, ResponseHeaders)> {
        self.get_with_request(KLINES_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kline_granularity_values() {
        assert_eq!(KlineGranularity::Min1 as i32, 1);
        assert_eq!(KlineGranularity::Min5 as i32, 5);
        assert_eq!(KlineGranularity::Hour1 as i32, 60);
        assert_eq!(KlineGranularity::Day1 as i32, 1440);
        assert_eq!(KlineGranularity::Week1 as i32, 10080);
    }

    #[test]
    fn test_get_klines_request() {
        let request = GetKlinesRequest {
            symbol: "XBTUSDTM".to_string(),
            granularity: KlineGranularity::Hour1,
            from: Some(1634567890000),
            to: Some(1634654290000),
        };

        assert_eq!(request.symbol, "XBTUSDTM");
        assert_eq!(request.granularity, KlineGranularity::Hour1);
        assert_eq!(request.from, Some(1634567890000));
        assert_eq!(request.to, Some(1634654290000));
    }

    #[test]
    fn test_klines_response_deserialization() {
        let json = r#"[
            [1634567890000, 49000.0, 49500.0, 48900.0, 49300.0, 1500.0],
            [1634571490000, 49300.0, 49800.0, 49200.0, 49700.0, 2000.0],
            [1634575090000, 49700.0, 50000.0, 49600.0, 49900.0, 1800.0]
        ]"#;

        let klines: GetKlinesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 3);

        // First kline
        assert_eq!(klines[0][0], 1634567890000.0); // time
        assert_eq!(klines[0][1], 49000.0); // open
        assert_eq!(klines[0][2], 49500.0); // high
        assert_eq!(klines[0][3], 48900.0); // low
        assert_eq!(klines[0][4], 49300.0); // close
        assert_eq!(klines[0][5], 1500.0); // volume

        // Last kline
        assert_eq!(klines[2][4], 49900.0); // close price of last kline
    }

    #[test]
    fn test_kline_granularity_serialization() {
        let json = serde_json::to_string(&KlineGranularity::Hour4).unwrap();
        assert_eq!(json, "240");

        let granularity: KlineGranularity = serde_json::from_str("60").unwrap();
        assert_eq!(granularity, KlineGranularity::Hour1);
    }
}
