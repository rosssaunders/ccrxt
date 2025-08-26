use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

/// Endpoint path for 24hr Ticker Price Change Statistics.
const TICKER_24HR_ENDPOINT: &str = "/dapi/v1/ticker/24hr";

/// Request parameters for the 24hr Ticker Price Change Statistics endpoint.
///
/// Used to filter results by symbol or pair. Symbol and pair cannot be sent together.
/// If neither is provided, tickers for all symbols of all pairs will be returned.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Ticker24hrParams {
    /// Trading symbol (e.g., "BTCUSD_PERP"). Optional.
    /// If provided, returns statistics for the specified symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Contract pair (e.g., "BTCUSD"). Optional.
    /// If provided, returns statistics for all symbols of the pair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
}

/// 24hr ticker price change statistics returned by the endpoint.
///
/// All fields are direct mappings from the Binance API response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker24hr {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    /// Matches the symbol field in the Binance API response.
    pub symbol: String,

    /// Contract pair (e.g., "BTCUSD").
    /// Matches the pair field in the Binance API response.
    pub pair: String,

    /// Absolute price change over the last 24 hours.
    /// Value is a signed decimal string from the API.
    pub price_change: Decimal,

    /// Price change percent over the last 24 hours.
    /// Value is a signed decimal string from the API.
    pub price_change_percent: Decimal,

    /// Weighted average price over the last 24 hours.
    /// Value is a decimal string from the API.
    pub weighted_avg_price: Decimal,

    /// Last traded price.
    /// Value is a decimal string from the API.
    pub last_price: Decimal,

    /// Quantity of the last trade.
    /// Value is a decimal string from the API.
    pub last_qty: Decimal,

    /// Opening price 24 hours ago.
    /// Value is a decimal string from the API.
    pub open_price: Decimal,

    /// Highest price in the last 24 hours.
    /// Value is a decimal string from the API.
    pub high_price: Decimal,

    /// Lowest price in the last 24 hours.
    /// Value is a decimal string from the API.
    pub low_price: Decimal,

    /// Total traded base asset volume in the last 24 hours.
    /// Value is a decimal string from the API.
    pub volume: Decimal,

    /// Total traded base asset volume in the last 24 hours.
    /// This field is called baseVolume in the API response.
    #[serde(rename = "baseVolume")]
    pub base_volume: Decimal,

    /// Statistics open time (milliseconds since epoch).
    /// Value is an integer from the API.
    pub open_time: i64,

    /// Statistics close time (milliseconds since epoch).
    /// Value is an integer from the API.
    pub close_time: i64,

    /// First trade ID in the 24hr window.
    /// Value is an integer from the API.
    pub first_id: i64,

    /// Last trade ID in the 24hr window.
    /// Value is an integer from the API.
    pub last_id: i64,

    /// Number of trades in the 24hr window.
    /// Value is an integer from the API.
    pub count: i64,
}

impl RestClient {
    /// 24hr Ticker Price Change Statistics
    ///
    /// Returns 24 hour rolling window price change statistics for coin-margined futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/24hr-Ticker-Price-Change-Statistics)
    ///
    /// Weight: 1 for a single symbol; 40 when the symbol parameter is omitted
    ///
    /// # Arguments
    /// * `params` - Request parameters for filtering by symbol or pair. If both are None, returns all tickers.
    ///
    /// # Returns
    /// A vector of `Ticker24hr` structs containing price change statistics for each symbol.
    pub async fn get_ticker_24hr(&self, params: Ticker24hrParams) -> RestResult<Vec<Ticker24hr>> {
        let weight = if params.symbol.is_some() || params.pair.is_some() {
            1
        } else {
            40
        };

        // The API always returns an array, even for single symbols
        self.send_get_request(TICKER_24HR_ENDPOINT, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

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
        assert_eq!(ticker.base_volume, Decimal::new(56875250125, 3));
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
