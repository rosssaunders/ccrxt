use serde::Deserialize;
use crate::binance::option::enums::{
    OrderType, TimeInForce, SymbolStatus, OptionType, Unit,
};
use crate::binance::option::rate_limit::{
    RateLimitInterval, RateLimitType,
};
use crate::binance::option::RestResult;
use crate::binance::option::public::rest::RestClient;

/// Represents an option symbol trading information.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    /// Symbol name (e.g., "BTC-240329-73000-C")
    pub symbol: String,

    /// Underlying asset (e.g., "BTC")
    pub underlying: String,

    /// Quote asset (e.g., "USDT")
    pub quote_asset: String,

    /// Option type (CALL or PUT)
    #[serde(rename = "side")]
    pub option_type: OptionType,

    /// Strike price
    pub strike_price: String,

    /// Expiry date timestamp
    pub expiry_date: i64,

    /// Price precision
    pub price_precision: i32,

    /// Quantity precision  
    pub quantity_precision: i32,

    /// Base asset precision
    pub base_asset_precision: i32,

    /// Quote precision
    pub quote_precision: i32,

    /// Symbol status
    pub status: SymbolStatus,

    /// Unit type
    pub unit: Unit,

    /// Minimum tick size for price
    pub min_tick: String,

    /// Minimum quantity
    pub min_qty: String,

    /// Available order types
    pub order_types: Vec<OrderType>,

    /// Available time in force options
    pub time_in_force: Vec<TimeInForce>,

    /// Trading filters
    pub filters: Vec<Filter>,
}

/// PRICE_FILTER defines the price rules for an option symbol.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    /// Minimum price allowed; disabled when minPrice == 0
    pub min_price: Option<String>,
    
    /// Maximum price allowed; disabled when maxPrice == 0
    pub max_price: Option<String>,
    
    /// Price tick size intervals; disabled when tickSize == 0
    pub tick_size: Option<String>,
}

/// LOT_SIZE filter defines the quantity rules for an option symbol.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    /// Minimum quantity allowed; disabled when minQty == 0
    pub min_qty: Option<String>,
    
    /// Maximum quantity allowed; disabled when maxQty == 0
    pub max_qty: Option<String>,
    
    /// Quantity step size intervals; disabled when stepSize == 0
    pub step_size: Option<String>,
}

/// Trading rule filters for option symbols.
#[derive(Debug, Deserialize)]
#[serde(tag = "filterType")]
pub enum Filter {
    /// PRICE_FILTER defines price rules
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter(PriceFilter),
    
    /// LOT_SIZE defines quantity rules
    #[serde(rename = "LOT_SIZE")]
    LotSizeFilter(LotSizeFilter),
    
    /// Unknown filter type
    #[serde(other)]
    Unknown,
}

/// Represents the response from the Binance Options Exchange Information endpoint.
///
/// See: https://developers.binance.com/docs/derivatives/option/common-definition
#[derive(Debug, Deserialize)]
pub struct ExchangeInfoResponse {
    /// The timezone of the exchange (e.g., "UTC").
    pub timezone: String,

    /// The list of rate limits applied to the exchange.
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,

    /// The list of option symbols available on the exchange.
    pub symbols: Vec<Symbol>,
}

/// Represents a rate limit object in the exchange info response.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    /// Type of rate limit
    pub rate_limit_type: RateLimitType,
    
    /// Interval for the rate limit
    pub interval: RateLimitInterval,
    
    /// Number of intervals
    pub interval_num: i32,
    
    /// Limit value
    pub limit: i32,
}

impl RestClient {
    /// Fetches current exchange trading rules and symbol information for Options.
    ///
    /// See: https://developers.binance.com/docs/derivatives/option/common-definition
    /// Corresponds to endpoint GET /eapi/v1/exchangeInfo.
    /// Weight: 1
    pub async fn get_exchange_info(&self) -> RestResult<ExchangeInfoResponse> {
        self.send_request(
            "/eapi/v1/exchangeInfo",
            reqwest::Method::GET,
            None,
            None,
            1, // weight
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_price_filter_deserialization() {
        let json = r#"{
            "filterType": "PRICE_FILTER",
            "minPrice": "0.00000100",
            "maxPrice": "100000.00000000",
            "tickSize": "0.00000100"
        }"#;

        let filter: Filter = serde_json::from_str(json).unwrap();
        match filter {
            Filter::PriceFilter(price_filter) => {
                assert_eq!(price_filter.min_price, Some("0.00000100".to_string()));
                assert_eq!(price_filter.max_price, Some("100000.00000000".to_string()));
                assert_eq!(price_filter.tick_size, Some("0.00000100".to_string()));
            }
            _ => panic!("Expected PriceFilter"),
        }
    }

    #[test]
    fn test_lot_size_filter_deserialization() {
        let json = r#"{
            "filterType": "LOT_SIZE",
            "minQty": "0.00100000",
            "maxQty": "100000.00000000",
            "stepSize": "0.00100000"
        }"#;

        let filter: Filter = serde_json::from_str(json).unwrap();
        match filter {
            Filter::LotSizeFilter(lot_size_filter) => {
                assert_eq!(lot_size_filter.min_qty, Some("0.00100000".to_string()));
                assert_eq!(lot_size_filter.max_qty, Some("100000.00000000".to_string()));
                assert_eq!(lot_size_filter.step_size, Some("0.00100000".to_string()));
            }
            _ => panic!("Expected LotSizeFilter"),
        }
    }

    #[test]
    fn test_price_filter_disabled_rules() {
        let json = r#"{
            "filterType": "PRICE_FILTER",
            "minPrice": "0",
            "maxPrice": "100000.00000000",
            "tickSize": "0"
        }"#;

        let filter: Filter = serde_json::from_str(json).unwrap();
        match filter {
            Filter::PriceFilter(price_filter) => {
                // When minPrice == "0", the min price rule is disabled
                assert_eq!(price_filter.min_price, Some("0".to_string()));
                // When tickSize == "0", the tick size rule is disabled
                assert_eq!(price_filter.tick_size, Some("0".to_string()));
                // Max price is still active
                assert_eq!(price_filter.max_price, Some("100000.00000000".to_string()));
            }
            _ => panic!("Expected PriceFilter"),
        }
    }

    #[test]
    fn test_unknown_filter_handling() {
        let json = r#"{
            "filterType": "UNKNOWN_FILTER",
            "someParam": "value"
        }"#;

        let filter: Filter = serde_json::from_str(json).unwrap();
        match filter {
            Filter::Unknown => {
                // Successfully handled unknown filter type
            }
            _ => panic!("Expected Unknown filter"),
        }
    }

    #[test]
    fn test_exchange_info_structure() {
        let json = r#"{
            "timezone": "UTC",
            "rateLimits": [
                {
                    "rateLimitType": "REQUEST_WEIGHT",
                    "interval": "MINUTE",
                    "intervalNum": 1,
                    "limit": 6000
                }
            ],
            "symbols": [
                {
                    "symbol": "BTC-240329-73000-C",
                    "underlying": "BTC",
                    "quoteAsset": "USDT",
                    "side": "CALL",
                    "strikePrice": "73000",
                    "expiryDate": 1711699200000,
                    "pricePrecision": 4,
                    "quantityPrecision": 3,
                    "baseAssetPrecision": 8,
                    "quotePrecision": 8,
                    "status": "TRADING",
                    "unit": "CONT",
                    "minTick": "0.0001",
                    "minQty": "0.001",
                    "orderTypes": ["LIMIT", "MARKET"],
                    "timeInForce": ["GTC", "IOC", "FOK"],
                    "filters": [
                        {
                            "filterType": "PRICE_FILTER",
                            "minPrice": "0.0001",
                            "maxPrice": "100000.0000",
                            "tickSize": "0.0001"
                        },
                        {
                            "filterType": "LOT_SIZE",
                            "minQty": "0.001",
                            "maxQty": "100000.000",
                            "stepSize": "0.001"
                        }
                    ]
                }
            ]
        }"#;

        let response: ExchangeInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.timezone, "UTC");
        assert_eq!(response.symbols.len(), 1);
        
        let symbol = &response.symbols[0];
        assert_eq!(symbol.symbol, "BTC-240329-73000-C");
        assert_eq!(symbol.underlying, "BTC");
        assert_eq!(symbol.option_type, OptionType::Call);
        assert_eq!(symbol.filters.len(), 2);
        
        // Verify price filter
        match &symbol.filters[0] {
            Filter::PriceFilter(price_filter) => {
                assert_eq!(price_filter.min_price, Some("0.0001".to_string()));
                assert_eq!(price_filter.max_price, Some("100000.0000".to_string()));
                assert_eq!(price_filter.tick_size, Some("0.0001".to_string()));
            }
            _ => panic!("Expected PriceFilter"),
        }
        
        // Verify lot size filter
        match &symbol.filters[1] {
            Filter::LotSizeFilter(lot_size_filter) => {
                assert_eq!(lot_size_filter.min_qty, Some("0.001".to_string()));
                assert_eq!(lot_size_filter.max_qty, Some("100000.000".to_string()));
                assert_eq!(lot_size_filter.step_size, Some("0.001".to_string()));
            }
            _ => panic!("Expected LotSizeFilter"),
        }
    }
}