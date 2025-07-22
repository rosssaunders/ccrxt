use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::{ContractType, KlineInterval, RestResult};

/// Request parameters for continuous contract kline/candlestick data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKlinesRequest {
    /// Pair (e.g., "BTCUSDT").
    pub pair: Cow<'static, str>,

    /// Contract type (PERPETUAL, CURRENT_QUARTER, NEXT_QUARTER).
    pub contract_type: ContractType,

    /// Kline interval.
    pub interval: KlineInterval,

    /// Start time in ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of klines to return. Default 500; max 1500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}
const CONTINUOUS_KLINES_ENDPOINT: &str = "/fapi/v1/continuousKlines";

/// Represents a single continuous contract kline/candlestick bar.
#[derive(Debug, Clone)]
pub struct ContinuousKline {
    /// Open time in ms.
    pub open_time: u64,
    /// Open price.
    pub open: String,
    /// High price.
    pub high: String,
    /// Low price.
    pub low: String,
    /// Close price.
    pub close: String,
    /// Volume.
    pub volume: String,
    /// Close time in ms.
    pub close_time: u64,
    /// Quote asset volume.
    pub quote_asset_volume: String,
    /// Number of trades.
    pub number_of_trades: u64,
    /// Taker buy base asset volume.
    pub taker_buy_base_asset_volume: String,
    /// Taker buy quote asset volume.
    pub taker_buy_quote_asset_volume: String,
    /// Ignore field.
    pub ignore: String,
}

impl<'de> Deserialize<'de> for ContinuousKline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array: (u64, String, String, String, String, String, u64, String, u64, String, String, String) = 
            Deserialize::deserialize(deserializer)?;
        
        Ok(ContinuousKline {
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
    /// Continuous Contract Kline/Candlestick Data (GET /fapi/v1/continuousKlines)
    ///
    /// Gets continuous contract kline/candlestick bars for USDS margined futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data
    ///
    /// Rate limit: 2 weight
    ///
    /// # Arguments
    /// * `params` - Request parameters for continuous contract kline/candlestick data.
    ///
    /// # Returns
    /// A vector of [`ContinuousKline`] structs.
    pub async fn get_continuous_klines(
        &self,
        params: ContinuousKlinesRequest,
    ) -> RestResult<Vec<ContinuousKline>> {
        self.send_public_request(
            "/fapi/v1/continuousKlines",
            reqwest::Method::GET,
            Some(params),
            2,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continuous_klines_request_serialization() {
        let request = ContinuousKlinesRequest {
            pair: "BTCUSDT".into(),
            contract_type: ContractType::Perpetual,
            interval: KlineInterval::I1m,
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=BTCUSDT"));
        assert!(serialized.contains("contractType=PERPETUAL"));
        assert!(serialized.contains("interval=1m"));
        assert!(serialized.contains("startTime=1625184000000"));
        assert!(serialized.contains("endTime=1625270400000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_continuous_klines_request_minimal() {
        let request = ContinuousKlinesRequest {
            pair: "ETHUSDT".into(),
            contract_type: ContractType::CurrentQuarter,
            interval: KlineInterval::I1h,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("pair=ETHUSDT"));
        assert!(serialized.contains("contractType=CURRENT_QUARTER"));
        assert!(serialized.contains("interval=1h"));
        assert!(!serialized.contains("startTime="));
        assert!(!serialized.contains("endTime="));
        assert!(!serialized.contains("limit="));
    }

    #[test]
    fn test_continuous_kline_deserialization() {
        let json = r#"[
            [
                1625184000000,
                "45380.10",
                "45400.20",
                "45360.00",
                "45390.30",
                "1234.567",
                1625184059999,
                "56012345.67890",
                5678,
                "567.890",
                "25801234.56789",
                "0"
            ],
            [
                1625184060000,
                "45390.30",
                "45410.50",
                "45385.10",
                "45405.40",
                "2345.678",
                1625184119999,
                "106523456.78901",
                6789,
                "1234.567",
                "56012345.67890",
                "0"
            ]
        ]"#;

        let klines: Vec<ContinuousKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 2);

        assert_eq!(klines[0].open_time, 1625184000000); // open_time
        assert_eq!(klines[0].open, "45380.10"); // open
        assert_eq!(klines[0].high, "45400.20"); // high
        assert_eq!(klines[0].low, "45360.00"); // low
        assert_eq!(klines[0].close, "45390.30"); // close
        assert_eq!(klines[0].volume, "1234.567"); // volume
        assert_eq!(klines[0].close_time, 1625184059999); // close_time
        assert_eq!(klines[0].quote_asset_volume, "56012345.67890"); // quote_asset_volume
        assert_eq!(klines[0].number_of_trades, 5678); // number_of_trades
        assert_eq!(klines[0].taker_buy_base_asset_volume, "567.890"); // taker_buy_base_asset_volume
        assert_eq!(klines[0].taker_buy_quote_asset_volume, "25801234.56789"); // taker_buy_quote_asset_volume

        assert_eq!(klines[1].open_time, 1625184060000);
        assert_eq!(klines[1].volume, "2345.678");
        assert_eq!(klines[1].number_of_trades, 6789);
    }

    #[test]
    fn test_continuous_klines_different_intervals() {
        let intervals = vec![
            KlineInterval::I1m,
            KlineInterval::I3m,
            KlineInterval::I5m,
            KlineInterval::I15m,
            KlineInterval::I30m,
            KlineInterval::I1h,
            KlineInterval::I2h,
            KlineInterval::I4h,
            KlineInterval::I6h,
            KlineInterval::I8h,
            KlineInterval::I12h,
            KlineInterval::I1d,
            KlineInterval::I3d,
            KlineInterval::I1w,
            KlineInterval::I1M,
        ];

        for interval in intervals {
            let request = ContinuousKlinesRequest {
                pair: "BTCUSDT".into(),
                contract_type: ContractType::Perpetual,
                interval,
                start_time: None,
                end_time: None,
                limit: None,
            };
            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("interval={}", interval.as_str())));
        }
    }

    #[test]
    fn test_continuous_klines_different_contract_types() {
        let contract_types = vec![
            (ContractType::Perpetual, "PERPETUAL"),
            (ContractType::CurrentQuarter, "CURRENT_QUARTER"),
            (ContractType::NextQuarter, "NEXT_QUARTER"),
        ];

        for (contract_type, expected_str) in contract_types {
            let request = ContinuousKlinesRequest {
                pair: "BTCUSDT".into(),
                contract_type,
                interval: KlineInterval::I1m,
                start_time: None,
                end_time: None,
                limit: None,
            };
            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("contractType={}", expected_str)));
        }
    }

    #[test]
    fn test_continuous_kline_empty_response() {
        let json = r#"[]"#;
        let klines: Vec<ContinuousKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 0);
    }

    #[test]
    fn test_continuous_klines_max_limit() {
        let request = ContinuousKlinesRequest {
            pair: "BTCUSDT".into(),
            contract_type: ContractType::Perpetual,
            interval: KlineInterval::I1m,
            start_time: None,
            end_time: None,
            limit: Some(1500), // max limit
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1500"));
    }

    #[test]
    fn test_continuous_kline_high_volume() {
        let json = r#"[
            [
                1625184000000,
                "45380.10",
                "45400.20",
                "45360.00",
                "45390.30",
                "1000000.000",
                1625184059999,
                "45390300000.00",
                100000,
                "500000.000",
                "22695150000.00",
                "0"
            ]
        ]"#;

        let klines: Vec<ContinuousKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 1);
        assert_eq!(klines[0].volume, "1000000.000"); // volume
        assert_eq!(klines[0].quote_asset_volume, "45390300000.00"); // quote_asset_volume
        assert_eq!(klines[0].number_of_trades, 100000); // number_of_trades
    }

    #[test]
    fn test_continuous_kline_small_values() {
        let json = r#"[
            [
                1625184000000,
                "0.00001234",
                "0.00001240",
                "0.00001230",
                "0.00001235",
                "0.001",
                1625184059999,
                "0.00001235",
                1,
                "0.0005",
                "0.000006175",
                "0"
            ]
        ]"#;

        let klines: Vec<ContinuousKline> = serde_json::from_str(json).unwrap();
        assert_eq!(klines.len(), 1);
        assert_eq!(klines[0].open, "0.00001234");
        assert_eq!(klines[0].volume, "0.001");
        assert_eq!(klines[0].number_of_trades, 1);
    }
}
