use crate::binance::spot::public::rest::RestClient;
use crate::binance::spot::RestResult;
use crate::binance::spot::enums::{OrderType, SymbolStatus};
use crate::binance::spot::rate_limit::{RateLimitType, RateLimitInterval};
use serde::Deserialize;

/// Represents a rate limit in the exchange info response.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    /// Type of rate limit
    pub rate_limit_type: RateLimitType,
    /// Interval for the rate limit
    pub interval: RateLimitInterval,
    /// Number of intervals
    pub interval_num: u32,
    /// Maximum number of requests allowed
    pub limit: u32,
}

/// Represents a filter in the symbol information.
#[derive(Debug, Deserialize)]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter {
        #[serde(rename = "minPrice")]
        min_price: String,
        #[serde(rename = "maxPrice")]
        max_price: String,
        #[serde(rename = "tickSize")]
        tick_size: String,
    },
    #[serde(rename = "LOT_SIZE")]
    LotSize {
        #[serde(rename = "minQty")]
        min_qty: String,
        #[serde(rename = "maxQty")]
        max_qty: String,
        #[serde(rename = "stepSize")]
        step_size: String,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    IcebergParts {
        limit: u32,
    },
    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize {
        #[serde(rename = "minQty")]
        min_qty: String,
        #[serde(rename = "maxQty")]
        max_qty: String,
        #[serde(rename = "stepSize")]
        step_size: String,
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
    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice {
        #[serde(rename = "multiplierUp")]
        multiplier_up: String,
        #[serde(rename = "multiplierDown")]
        multiplier_down: String,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: u32,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    MinNotional {
        #[serde(rename = "minNotional")]
        min_notional: String,
        #[serde(rename = "applyToMarket")]
        apply_to_market: bool,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: u32,
    },
    #[serde(rename = "NOTIONAL")]
    Notional {
        #[serde(rename = "minNotional")]
        min_notional: String,
        #[serde(rename = "applyMinToMarket")]
        apply_min_to_market: bool,
        #[serde(rename = "maxNotional")]
        max_notional: String,
        #[serde(rename = "applyMaxToMarket")]
        apply_max_to_market: bool,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: u32,
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
    #[serde(other)]
    Unknown,
}

/// Represents a symbol in the exchange info response.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    /// Symbol name
    pub symbol: String,
    /// Symbol status
    pub status: SymbolStatus,
    /// Base asset
    pub base_asset: String,
    /// Base asset precision
    pub base_asset_precision: u32,
    /// Quote asset
    pub quote_asset: String,
    /// Quote asset precision
    pub quote_precision: u32,
    /// Quote asset precision
    pub quote_asset_precision: u32,
    /// Base commission precision
    pub base_commission_precision: u32,
    /// Quote commission precision
    pub quote_commission_precision: u32,
    /// Allowed order types
    pub order_types: Vec<OrderType>,
    /// Whether iceberg orders are allowed
    pub iceberg_allowed: bool,
    /// Whether OCO orders are allowed
    pub oco_allowed: bool,
    /// Whether quote order quantity market orders are allowed
    pub quote_order_qty_market_allowed: bool,
    /// Whether spot trading is allowed
    pub allow_trailing_stop: bool,
    /// Whether cancel replace is allowed
    pub cancel_replace_allowed: bool,
    /// Whether spot trading is allowed
    pub is_spot_trading_allowed: bool,
    /// Whether margin trading is allowed
    pub is_margin_trading_allowed: bool,
    /// Filters for the symbol
    pub filters: Vec<Filter>,
    /// Permissions for the symbol
    pub permissions: Vec<String>,
    /// Default self trade prevention mode
    #[serde(rename = "defaultSelfTradePreventionMode")]
    pub default_self_trade_prevention_mode: String,
    /// Allowed self trade prevention modes
    #[serde(rename = "allowedSelfTradePreventionModes")]
    pub allowed_self_trade_prevention_modes: Vec<String>,
}

/// Response from the exchange info endpoint.
///
/// See: <https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#exchange-information>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfoResponse {
    /// Exchange timezone
    pub timezone: String,
    /// Server time
    pub server_time: i64,
    /// Rate limits
    pub rate_limits: Vec<RateLimit>,
    /// Exchange filters (currently empty for spot)
    pub exchange_filters: Vec<serde_json::Value>,
    /// Trading symbols
    pub symbols: Vec<Symbol>,
}

impl RestClient {
    /// Get exchange information
    /// 
    /// Current exchange trading rules and symbol information
    /// 
    /// Weight: 20
    pub async fn exchange_info(&self) -> RestResult<ExchangeInfoResponse> {
        self.send_request("/api/v3/exchangeInfo", reqwest::Method::GET, None, None, 20)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::spot::RateLimiter;

    #[tokio::test]
    async fn test_exchange_info_method_exists() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        let rest_client = RestClient::new("https://api.binance.com", client, rate_limiter);

        // Test that the exchange_info method is accessible
        // We're not calling it to avoid network requests in tests
        let _ = &rest_client.exchange_info();
    }
}