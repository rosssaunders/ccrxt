use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

const EXCHANGE_INFO_ENDPOINT: &str = "/api/v3/exchangeInfo";

/// Request parameters for exchange info
#[derive(Debug, Clone, Serialize, Default)]
pub struct ExchangeInfoRequest {
    /// Single symbol filter
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Multiple symbols filter (array format)
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,

    /// Permission filter
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,

    /// Control permissionSets field population
    #[serde(rename = "showPermissionSets", skip_serializing_if = "Option::is_none")]
    pub show_permission_sets: Option<bool>,

    /// Filter by trading status
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    pub symbol_status: Option<String>,
}

/// Exchange information response
#[derive(Debug, Clone, Deserialize)]
pub struct ExchangeInfoResponse {
    /// Time zone of the server
    #[serde(rename = "timezone")]
    pub timezone: String,

    /// Current server time
    #[serde(rename = "serverTime")]
    pub server_time: u64,

    /// Rate limits applied to the API
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,

    /// Exchange filters
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Vec<serde_json::Value>,

    /// Trading symbols available on the exchange
    #[serde(rename = "symbols")]
    pub symbols: Vec<Symbol>,
}

/// Rate limit information
#[derive(Debug, Clone, Deserialize)]
pub struct RateLimit {
    /// Type of rate limit
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,

    /// Interval for the rate limit
    #[serde(rename = "interval")]
    pub interval: String,

    /// Interval number
    #[serde(rename = "intervalNum")]
    pub interval_num: u32,

    /// Rate limit value
    #[serde(rename = "limit")]
    pub limit: u32,
}

/// Symbol information
#[derive(Debug, Clone, Deserialize)]
pub struct Symbol {
    /// Symbol name
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Symbol status
    #[serde(rename = "status")]
    pub status: String,

    /// Base asset
    #[serde(rename = "baseAsset")]
    pub base_asset: String,

    /// Base asset precision
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: u32,

    /// Quote asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,

    /// Quote precision
    #[serde(rename = "quotePrecision")]
    pub quote_precision: u32,

    /// Quote asset precision
    #[serde(rename = "quoteAssetPrecision")]
    pub quote_asset_precision: u32,

    /// Base commission precision
    #[serde(rename = "baseCommissionPrecision")]
    pub base_commission_precision: u32,

    /// Quote commission precision
    #[serde(rename = "quoteCommissionPrecision")]
    pub quote_commission_precision: u32,

    /// Order types supported
    #[serde(rename = "orderTypes")]
    pub order_types: Vec<String>,

    /// Is icebergs allowed
    #[serde(rename = "icebergAllowed")]
    pub iceberg_allowed: bool,

    /// Is OCO allowed
    #[serde(rename = "ocoAllowed")]
    pub oco_allowed: bool,

    /// Quote order qty market allowed
    #[serde(rename = "quoteOrderQtyMarketAllowed")]
    pub quote_order_qty_market_allowed: bool,

    /// Allow trailing stop
    #[serde(rename = "allowTrailingStop")]
    pub allow_trailing_stop: bool,

    /// Cancel replace allowed
    #[serde(rename = "cancelReplaceAllowed")]
    pub cancel_replace_allowed: bool,

    /// Is spot trading allowed
    #[serde(rename = "isSpotTradingAllowed")]
    pub is_spot_trading_allowed: bool,

    /// Is margin trading allowed
    #[serde(rename = "isMarginTradingAllowed")]
    pub is_margin_trading_allowed: bool,

    /// Filters applied to the symbol
    #[serde(rename = "filters")]
    pub filters: Vec<Filter>,

    /// Permissions for the symbol
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,

    /// Permission sets
    #[serde(rename = "permissionSets", skip_serializing_if = "Option::is_none")]
    pub permission_sets: Option<Vec<Vec<String>>>,

    /// Default self trade prevention mode
    #[serde(rename = "defaultSelfTradePreventionMode")]
    pub default_self_trade_prevention_mode: String,

    /// Allowed self trade prevention modes
    #[serde(rename = "allowedSelfTradePreventionModes")]
    pub allowed_self_trade_prevention_modes: Vec<String>,
}

/// Symbol filter
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter {
        #[serde(rename = "minPrice")]
        min_price: Decimal,
        #[serde(rename = "maxPrice")]
        max_price: Decimal,
        #[serde(rename = "tickSize")]
        tick_size: Decimal,
    },
    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice {
        #[serde(rename = "multiplierUp")]
        multiplier_up: Decimal,
        #[serde(rename = "multiplierDown")]
        multiplier_down: Decimal,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: u32,
    },
    #[serde(rename = "PERCENT_PRICE_BY_SIDE")]
    PercentPriceBySide {
        #[serde(rename = "bidMultiplierUp")]
        bid_multiplier_up: Decimal,
        #[serde(rename = "bidMultiplierDown")]
        bid_multiplier_down: Decimal,
        #[serde(rename = "askMultiplierUp")]
        ask_multiplier_up: Decimal,
        #[serde(rename = "askMultiplierDown")]
        ask_multiplier_down: Decimal,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: u32,
    },
    #[serde(rename = "LOT_SIZE")]
    LotSize {
        #[serde(rename = "minQty")]
        min_qty: Decimal,
        #[serde(rename = "maxQty")]
        max_qty: Decimal,
        #[serde(rename = "stepSize")]
        step_size: Decimal,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    MinNotional {
        #[serde(rename = "minNotional")]
        min_notional: Decimal,
        #[serde(rename = "applyToMarket")]
        apply_to_market: bool,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: u32,
    },
    #[serde(rename = "NOTIONAL")]
    Notional {
        #[serde(rename = "minNotional")]
        min_notional: Decimal,
        #[serde(rename = "applyMinToMarket")]
        apply_min_to_market: bool,
        #[serde(rename = "maxNotional")]
        max_notional: Decimal,
        #[serde(rename = "applyMaxToMarket")]
        apply_max_to_market: bool,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: u32,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    IcebergParts {
        #[serde(rename = "limit")]
        limit: u32,
    },
    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize {
        #[serde(rename = "minQty")]
        min_qty: Decimal,
        #[serde(rename = "maxQty")]
        max_qty: Decimal,
        #[serde(rename = "stepSize")]
        step_size: Decimal,
    },
    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders {
        #[serde(rename = "maxNumOrders")]
        max_num_orders: u32,
    },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders {
        #[serde(rename = "maxNumAlgoOrders")]
        max_num_algo_orders: u32,
    },
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    MaxNumIcebergOrders {
        #[serde(rename = "maxNumIcebergOrders")]
        max_num_iceberg_orders: u32,
    },
    #[serde(rename = "MAX_POSITION")]
    MaxPosition {
        #[serde(rename = "maxPosition")]
        max_position: Decimal,
    },
    #[serde(rename = "TRAILING_DELTA")]
    TrailingDelta {
        #[serde(rename = "minTrailingAboveDelta")]
        min_trailing_above_delta: u32,
        #[serde(rename = "maxTrailingAboveDelta")]
        max_trailing_above_delta: u32,
        #[serde(rename = "minTrailingBelowDelta")]
        min_trailing_below_delta: u32,
        #[serde(rename = "maxTrailingBelowDelta")]
        max_trailing_below_delta: u32,
    },
    #[serde(other)]
    Unknown,
}

impl RestClient {
    /// Get exchange information
    ///
    /// Returns current exchange trading rules and symbol information.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#exchange-information)
    /// Method: GET /api/v3/exchangeInfo
    /// Weight: 20
    /// Security: None
    pub async fn get_exchange_info(
        &self,
        params: Option<ExchangeInfoRequest>,
    ) -> RestResult<ExchangeInfoResponse> {
        self.send_public_request(EXCHANGE_INFO_ENDPOINT, reqwest::Method::GET, params, 20)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_info_request_serialization_empty() {
        let request = ExchangeInfoRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_exchange_info_request_serialization_single_symbol() {
        let request = ExchangeInfoRequest {
            symbol: Some("BTCUSDT".to_string()),
            symbols: None,
            permissions: None,
            show_permission_sets: None,
            symbol_status: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_exchange_info_request_serialization_multiple_symbols() {
        let request = ExchangeInfoRequest {
            symbol: None,
            symbols: Some("[\"BTCUSDT\",\"ETHUSDT\"]".to_string()),
            permissions: None,
            show_permission_sets: None,
            symbol_status: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbols=%5B%22BTCUSDT%22%2C%22ETHUSDT%22%5D"));
    }

    #[test]
    fn test_exchange_info_request_serialization_permissions() {
        let request = ExchangeInfoRequest {
            symbol: None,
            symbols: None,
            permissions: Some("SPOT".to_string()),
            show_permission_sets: Some(true),
            symbol_status: Some("TRADING".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("permissions=SPOT"));
        assert!(serialized.contains("showPermissionSets=true"));
        assert!(serialized.contains("symbolStatus=TRADING"));
    }

    #[test]
    fn test_rate_limit_deserialization() {
        let json = r#"{
            "rateLimitType": "REQUEST_WEIGHT",
            "interval": "MINUTE",
            "intervalNum": 1,
            "limit": 6000
        }"#;

        let rate_limit: RateLimit = serde_json::from_str(json).unwrap();
        assert_eq!(rate_limit.rate_limit_type, "REQUEST_WEIGHT");
        assert_eq!(rate_limit.interval, "MINUTE");
        assert_eq!(rate_limit.interval_num, 1);
        assert_eq!(rate_limit.limit, 6000);
    }

    #[test]
    fn test_filter_price_filter_deserialization() {
        let json = r#"{
            "filterType": "PRICE_FILTER",
            "minPrice": "0.00010000",
            "maxPrice": "100000.00000000",
            "tickSize": "0.00010000"
        }"#;

        let filter: Filter = serde_json::from_str(json).unwrap();
        match filter {
            Filter::PriceFilter {
                min_price,
                max_price,
                tick_size,
            } => {
                assert_eq!(min_price.to_string(), "0.00010000");
                assert_eq!(max_price.to_string(), "100000.00000000");
                assert_eq!(tick_size.to_string(), "0.00010000");
            }
            _ => panic!("Expected PriceFilter"),
        }
    }

    #[test]
    fn test_filter_lot_size_deserialization() {
        let json = r#"{
            "filterType": "LOT_SIZE",
            "minQty": "0.00100000",
            "maxQty": "100000.00000000",
            "stepSize": "0.00100000"
        }"#;

        let filter: Filter = serde_json::from_str(json).unwrap();
        match filter {
            Filter::LotSize {
                min_qty,
                max_qty,
                step_size,
            } => {
                assert_eq!(min_qty.to_string(), "0.00100000");
                assert_eq!(max_qty.to_string(), "100000.00000000");
                assert_eq!(step_size.to_string(), "0.00100000");
            }
            _ => panic!("Expected LotSize"),
        }
    }

    #[test]
    fn test_filter_min_notional_deserialization() {
        let json = r#"{
            "filterType": "MIN_NOTIONAL",
            "minNotional": "10.00000000",
            "applyToMarket": true,
            "avgPriceMins": 5
        }"#;

        let filter: Filter = serde_json::from_str(json).unwrap();
        match filter {
            Filter::MinNotional {
                min_notional,
                apply_to_market,
                avg_price_mins,
            } => {
                assert_eq!(min_notional.to_string(), "10.00000000");
                assert!(apply_to_market);
                assert_eq!(avg_price_mins, 5);
            }
            _ => panic!("Expected MinNotional"),
        }
    }

    #[test]
    fn test_filter_max_num_orders_deserialization() {
        let json = r#"{
            "filterType": "MAX_NUM_ORDERS",
            "maxNumOrders": 200
        }"#;

        let filter: Filter = serde_json::from_str(json).unwrap();
        match filter {
            Filter::MaxNumOrders { max_num_orders } => {
                assert_eq!(max_num_orders, 200);
            }
            _ => panic!("Expected MaxNumOrders"),
        }
    }

    #[test]
    fn test_filter_unknown_deserialization() {
        let json = r#"{
            "filterType": "UNKNOWN_FILTER",
            "someField": "someValue"
        }"#;

        let filter: Filter = serde_json::from_str(json).unwrap();
        match filter {
            Filter::Unknown => {
                // This is expected for unknown filter types
            }
            _ => panic!("Expected Unknown filter"),
        }
    }

    #[test]
    fn test_symbol_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "status": "TRADING",
            "baseAsset": "BTC",
            "baseAssetPrecision": 8,
            "quoteAsset": "USDT",
            "quotePrecision": 8,
            "quoteAssetPrecision": 8,
            "baseCommissionPrecision": 8,
            "quoteCommissionPrecision": 8,
            "orderTypes": ["LIMIT", "LIMIT_MAKER", "MARKET", "STOP_LOSS_LIMIT", "TAKE_PROFIT_LIMIT"],
            "icebergAllowed": true,
            "ocoAllowed": true,
            "quoteOrderQtyMarketAllowed": true,
            "allowTrailingStop": true,
            "cancelReplaceAllowed": true,
            "isSpotTradingAllowed": true,
            "isMarginTradingAllowed": true,
            "filters": [],
            "permissions": ["SPOT", "MARGIN"],
            "defaultSelfTradePreventionMode": "NONE",
            "allowedSelfTradePreventionModes": ["NONE", "EXPIRE_TAKER", "EXPIRE_MAKER", "EXPIRE_BOTH"]
        }"#;

        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol.symbol, "BTCUSDT");
        assert_eq!(symbol.status, "TRADING");
        assert_eq!(symbol.base_asset, "BTC");
        assert_eq!(symbol.base_asset_precision, 8);
        assert_eq!(symbol.quote_asset, "USDT");
        assert!(symbol.iceberg_allowed);
        assert!(symbol.oco_allowed);
        assert!(symbol.is_spot_trading_allowed);
        assert!(symbol.is_margin_trading_allowed);
        assert_eq!(symbol.permissions, vec!["SPOT", "MARGIN"]);
        assert_eq!(symbol.default_self_trade_prevention_mode, "NONE");
    }

    #[test]
    fn test_exchange_info_response_deserialization() {
        let json = r#"{
            "timezone": "UTC",
            "serverTime": 1625184000000,
            "rateLimits": [
                {
                    "rateLimitType": "REQUEST_WEIGHT",
                    "interval": "MINUTE",
                    "intervalNum": 1,
                    "limit": 6000
                }
            ],
            "exchangeFilters": [],
            "symbols": [
                {
                    "symbol": "BTCUSDT",
                    "status": "TRADING",
                    "baseAsset": "BTC",
                    "baseAssetPrecision": 8,
                    "quoteAsset": "USDT",
                    "quotePrecision": 8,
                    "quoteAssetPrecision": 8,
                    "baseCommissionPrecision": 8,
                    "quoteCommissionPrecision": 8,
                    "orderTypes": ["LIMIT", "MARKET"],
                    "icebergAllowed": true,
                    "ocoAllowed": true,
                    "quoteOrderQtyMarketAllowed": true,
                    "allowTrailingStop": false,
                    "cancelReplaceAllowed": false,
                    "isSpotTradingAllowed": true,
                    "isMarginTradingAllowed": false,
                    "filters": [],
                    "permissions": ["SPOT"],
                    "defaultSelfTradePreventionMode": "NONE",
                    "allowedSelfTradePreventionModes": ["NONE"]
                }
            ]
        }"#;

        let response: ExchangeInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.timezone, "UTC");
        assert_eq!(response.server_time, 1625184000000);
        assert_eq!(response.rate_limits.len(), 1);
        assert_eq!(response.symbols.len(), 1);
        assert_eq!(response.symbols[0].symbol, "BTCUSDT");
    }

    #[test]
    fn test_exchange_info_response_with_filters() {
        let json = r#"{
            "timezone": "UTC",
            "serverTime": 1625184000000,
            "rateLimits": [],
            "exchangeFilters": [],
            "symbols": [
                {
                    "symbol": "ETHUSDT",
                    "status": "TRADING",
                    "baseAsset": "ETH",
                    "baseAssetPrecision": 8,
                    "quoteAsset": "USDT",
                    "quotePrecision": 8,
                    "quoteAssetPrecision": 8,
                    "baseCommissionPrecision": 8,
                    "quoteCommissionPrecision": 8,
                    "orderTypes": ["LIMIT", "MARKET"],
                    "icebergAllowed": true,
                    "ocoAllowed": true,
                    "quoteOrderQtyMarketAllowed": true,
                    "allowTrailingStop": false,
                    "cancelReplaceAllowed": false,
                    "isSpotTradingAllowed": true,
                    "isMarginTradingAllowed": false,
                    "filters": [
                        {
                            "filterType": "PRICE_FILTER",
                            "minPrice": "0.01000000",
                            "maxPrice": "1000000.00000000",
                            "tickSize": "0.01000000"
                        },
                        {
                            "filterType": "LOT_SIZE",
                            "minQty": "0.00010000",
                            "maxQty": "100000.00000000",
                            "stepSize": "0.00010000"
                        }
                    ],
                    "permissions": ["SPOT"],
                    "defaultSelfTradePreventionMode": "NONE",
                    "allowedSelfTradePreventionModes": ["NONE"]
                }
            ]
        }"#;

        let response: ExchangeInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbols[0].filters.len(), 2);

        match &response.symbols[0].filters[0] {
            Filter::PriceFilter {
                min_price,
                max_price,
                tick_size,
            } => {
                assert_eq!(min_price.to_string(), "0.01000000");
                assert_eq!(max_price.to_string(), "1000000.00000000");
                assert_eq!(tick_size.to_string(), "0.01000000");
            }
            _ => panic!("Expected PriceFilter"),
        }

        match &response.symbols[0].filters[1] {
            Filter::LotSize {
                min_qty,
                max_qty,
                step_size,
            } => {
                assert_eq!(min_qty.to_string(), "0.00010000");
                assert_eq!(max_qty.to_string(), "100000.00000000");
                assert_eq!(step_size.to_string(), "0.00010000");
            }
            _ => panic!("Expected LotSize"),
        }
    }
}
