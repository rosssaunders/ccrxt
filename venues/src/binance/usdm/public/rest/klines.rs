use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::{RestResult, enums::KlineInterval, public_client::RestClient};

const KLINES_ENDPOINT: &str = "/fapi/v1/klines";

/// Request parameters for kline/candlestick data.
#[derive(Debug, Clone, Serialize)]
pub struct KlinesRequest {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// Kline interval.
    pub interval: KlineInterval,

    /// Start time in ms.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in ms.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of klines to return. Default 500; max 1500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/// Represents a single kline/candlestick bar.
///
/// Klines are returned as arrays with the following structure:
/// [Open time, Open, High, Low, Close, Volume, Close time, Quote asset volume, Number of trades, Taker buy base asset volume, Taker buy quote asset volume, Ignore]
#[derive(Debug, Clone)]
pub struct Kline {
    /// Open time (milliseconds since epoch)
    pub open_time: u64,
    /// Open price
    pub open: String,
    /// High price
    pub high: String,
    /// Low price
    pub low: String,
    /// Close price
    pub close: String,
    /// Volume
    pub volume: String,
    /// Close time (milliseconds since epoch)
    pub close_time: u64,
    /// Quote asset volume
    pub quote_asset_volume: String,
    /// Number of trades
    pub number_of_trades: u64,
    /// Taker buy base asset volume
    pub taker_buy_base_asset_volume: String,
    /// Taker buy quote asset volume
    pub taker_buy_quote_asset_volume: String,
    /// Ignore field
    pub ignore: String,
}

impl<'de> Deserialize<'de> for Kline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array: (
            u64,
            String,
            String,
            String,
            String,
            String,
            u64,
            String,
            u64,
            String,
            String,
            String,
        ) = Deserialize::deserialize(deserializer)?;

        Ok(Kline {
            open_time: array.0,
            open: array.1,
            high: array.2,
            low: array.3,
            close: array.4,
            volume: array.5,
            close_time: array.6,
            quote_asset_volume: array.7,
            number_of_trades: array.8,
            taker_buy_base_asset_volume: array.9,
            taker_buy_quote_asset_volume: array.10,
            ignore: array.11,
        })
    }
}

impl RestClient {
    /// Kline/Candlestick Data
    ///
    /// Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Kline-Candlestick-Data)
    ///
    /// Rate limit:
    /// - \[1,100) klines: 1 weight
    /// - \[100, 500) klines: 2 weight  
    /// - \[500, 1000] klines: 5 weight
    /// - >1000 klines: 10 weight
    ///
    /// # Arguments
    /// * `params` - The klines request parameters including symbol, interval, and optional time range
    ///
    /// # Returns
    /// Vector of kline data arrays containing OHLCV and additional trading data
    pub async fn get_klines(&self, params: KlinesRequest) -> RestResult<Vec<Kline>> {
        self.send_get_request(KLINES_ENDPOINT, Some(params), 2)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_klines_request_serialization() {
        let request = KlinesRequest {
            symbol: "BTCUSDT".into(),
            interval: KlineInterval::I1m,
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("interval=1m"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_klines_request_minimal() {
        let request = KlinesRequest {
            symbol: "ETHUSDT".into(),
            interval: KlineInterval::I1h,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("interval=1h"));
        // Optional fields should not be present when None due to skip_serializing_if
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
        assert!(!serialized.contains("limit"));
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

        let klines: Vec<Kline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        let first_kline = &klines[0];
        assert_eq!(first_kline.open_time, 1625097600000); // Open time
        assert_eq!(first_kline.open, "45000.00"); // Open
        assert_eq!(first_kline.high, "45500.00"); // High
        assert_eq!(first_kline.low, "44800.00"); // Low
        assert_eq!(first_kline.close, "45200.00"); // Close
        assert_eq!(first_kline.volume, "1250.500"); // Volume
        assert_eq!(first_kline.close_time, 1625101199999); // Close time
        assert_eq!(first_kline.quote_asset_volume, "56475000.00"); // Quote asset volume
        assert_eq!(first_kline.number_of_trades, 2500); // Number of trades
        assert_eq!(first_kline.taker_buy_base_asset_volume, "625.250"); // Taker buy base asset volume
        assert_eq!(first_kline.taker_buy_quote_asset_volume, "28237500.00"); // Taker buy quote asset volume
        assert_eq!(first_kline.ignore, "0"); // Ignore

        let second_kline = &klines[1];
        assert_eq!(second_kline.open_time, 1625101200000);
        assert_eq!(second_kline.close, "45400.00"); // Close price
    }

    #[test]
    fn test_kline_different_intervals() {
        let intervals = vec![
            (KlineInterval::I1m, "1m"),
            (KlineInterval::I5m, "5m"),
            (KlineInterval::I15m, "15m"),
            (KlineInterval::I30m, "30m"),
            (KlineInterval::I1h, "1h"),
            (KlineInterval::I4h, "4h"),
            (KlineInterval::I1d, "1d"),
            (KlineInterval::I1w, "1w"),
            (KlineInterval::I1M, "1M"),
        ];

        for (interval, expected_str) in intervals {
            let request = KlinesRequest {
                symbol: "BTCUSDT".into(),
                interval,
                start_time: None,
                end_time: None,
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("interval={}", expected_str)));
        }
    }

    #[test]
    fn test_kline_max_limit() {
        let request = KlinesRequest {
            symbol: "BTCUSDT".into(),
            interval: KlineInterval::I1m,
            start_time: None,
            end_time: None,
            limit: Some(1500), // Max limit
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1500"));
    }
}
