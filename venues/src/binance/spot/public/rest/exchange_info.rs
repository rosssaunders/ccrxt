use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

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
        let query_string = if let Some(p) = params {
            Some(serde_urlencoded::to_string(&p).map_err(|e| {
                crate::binance::spot::Errors::Error(format!("URL encoding error: {e}"))
            })?)
        } else {
            None
        };

        self.send_request(
            "/api/v3/exchangeInfo",
            reqwest::Method::GET,
            query_string.as_deref(),
            20,
        )
        .await
    }
}
