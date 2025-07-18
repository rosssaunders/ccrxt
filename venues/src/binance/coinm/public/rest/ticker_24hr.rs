use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Parameters for 24hr Ticker Price Change Statistics
#[derive(Debug, Clone, Serialize, Default)]
pub struct Ticker24hrParams {
    /// Symbol name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Contract type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
}

/// 24hr ticker price change statistics
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hr {
    /// Symbol name
    pub symbol: String,
    /// Pair name
    pub pair: String,
    /// Price change
    pub price_change: Decimal,
    /// Price change percent
    pub price_change_percent: Decimal,
    /// Weighted average price
    pub weighted_avg_price: Decimal,
    /// Last price
    pub last_price: Decimal,
    /// Last quantity
    pub last_qty: Decimal,
    /// Open price
    pub open_price: Decimal,
    /// High price
    pub high_price: Decimal,
    /// Low price
    pub low_price: Decimal,
    /// Total traded base asset volume
    pub volume: Decimal,
    /// Total traded quote asset volume (called baseVolume in API response)
    #[serde(rename = "baseVolume")]
    pub quote_volume: Decimal,
    /// Statistics open time
    pub open_time: i64,
    /// Statistics close time
    pub close_time: i64,
    /// First trade id
    pub first_id: i64,
    /// Last trade id
    pub last_id: i64,
    /// Trade count
    pub count: i64,
}

impl RestClient {
    /// Get 24hr ticker price change statistics
    ///
    /// https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics
    ///
    /// Weight: 1 for a single symbol; 40 when the symbol parameter is omitted
    pub async fn get_ticker_24hr(&self, params: Ticker24hrParams) -> RestResult<Vec<Ticker24hr>> {
        let weight = if params.symbol.is_some() || params.pair.is_some() {
            1
        } else {
            40
        };

        // The API always returns an array, even for single symbols
        self.send_request(
            "/dapi/v1/ticker/24hr",
            reqwest::Method::GET,
            Some(params),
            weight,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_ticker_24hr_request_serialization_with_symbol() {
        let request = Ticker24hrParams {
            symbol: Some("BTCUSD_PERP".to_string()),
            pair: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSD_PERP");
    }

    #[test]
    fn test_ticker_24hr_request_serialization_with_pair() {
        let request = Ticker24hrParams {
            symbol: None,
            pair: Some("BTCUSD".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "pair=BTCUSD");
    }

    #[test]
    fn test_ticker_24hr_request_serialization_empty() {
        let request = Ticker24hrParams {
            symbol: None,
            pair: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_ticker_24hr_response_deserialization() {
        let json = r#"[{
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "priceChange": "500.50",
            "priceChangePercent": "1.12",
            "weightedAvgPrice": "45250.75",
            "lastPrice": "45500.00",
            "lastQty": "0.500",
            "openPrice": "45000.00",
            "highPrice": "46000.00",
            "lowPrice": "44500.00",
            "volume": "1250.750",
            "baseVolume": "56875250.125",
            "openTime": 1625011200000,
            "closeTime": 1625097599999,
            "firstId": 123456789,
            "lastId": 123567890,
            "count": 111102
        }]"#;

        let response: Vec<Ticker24hr> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);

        let ticker = &response[0];
        assert_eq!(ticker.symbol, "BTCUSD_PERP");
        assert_eq!(ticker.pair, "BTCUSD");
        assert_eq!(ticker.price_change, Decimal::new(50050, 2));
        assert_eq!(ticker.price_change_percent, Decimal::new(112, 2));
        assert_eq!(ticker.weighted_avg_price, Decimal::new(4525075, 2));
        assert_eq!(ticker.last_price, Decimal::new(45500, 0));
        assert_eq!(ticker.last_qty, Decimal::new(500, 3));
        assert_eq!(ticker.open_price, Decimal::new(45000, 0));
        assert_eq!(ticker.high_price, Decimal::new(46000, 0));
        assert_eq!(ticker.low_price, Decimal::new(44500, 0));
        assert_eq!(ticker.volume, Decimal::new(1250750, 3));
        assert_eq!(ticker.quote_volume, Decimal::new(56875250125, 3));
        assert_eq!(ticker.open_time, 1625011200000);
        assert_eq!(ticker.close_time, 1625097599999);
        assert_eq!(ticker.first_id, 123456789);
        assert_eq!(ticker.last_id, 123567890);
        assert_eq!(ticker.count, 111102);
    }

    #[test]
    fn test_ticker_24hr_response_negative_change() {
        let json = r#"[{
            "symbol": "ETHUSD_PERP",
            "pair": "ETHUSD",
            "priceChange": "-125.75",
            "priceChangePercent": "-3.84",
            "weightedAvgPrice": "3125.50",
            "lastPrice": "3150.25",
            "lastQty": "2.500",
            "openPrice": "3276.00",
            "highPrice": "3300.00",
            "lowPrice": "3100.00",
            "volume": "875.250",
            "baseVolume": "2734531.875",
            "openTime": 1625011200000,
            "closeTime": 1625097599999,
            "firstId": 987654321,
            "lastId": 987765432,
            "count": 111110
        }]"#;

        let response: Vec<Ticker24hr> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);

        let ticker = &response[0];
        assert_eq!(ticker.symbol, "ETHUSD_PERP");
        assert_eq!(ticker.price_change, Decimal::new(-12575, 2));
        assert_eq!(ticker.price_change_percent, Decimal::new(-384, 2));
    }
}
