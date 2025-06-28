use serde::Deserialize;

use crate::binance::coinm::RestResult;
use crate::binance::coinm::enums::{
    ContractStatus, ContractType, OrderType, TimeInForce, UnderlyingType,
};
use crate::binance::coinm::public::rest::RestClient;
use crate::binance::coinm::rate_limit::{RateLimitInterval, RateLimitType};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,

    pub pair: String,

    pub contract_type: ContractType,

    pub delivery_date: i64,

    pub onboard_date: i64,

    pub contract_status: ContractStatus,

    pub contract_size: i64,

    pub margin_asset: String,

    pub maint_margin_percent: String,

    pub required_margin_percent: String,

    pub base_asset: String,

    pub quote_asset: QuoteAsset,

    pub price_precision: i64,

    pub quantity_precision: i64,

    pub base_asset_precision: i64,

    pub quote_precision: i64,

    pub equal_qty_precision: i64,

    pub max_move_order_limit: i64,

    pub trigger_protect: String,

    pub underlying_type: UnderlyingType,

    pub underlying_sub_type: Vec<String>,

    pub filters: Vec<Filter>,

    pub order_types: Vec<OrderType>,

    pub time_in_force: Vec<TimeInForce>,

    pub liquidation_fee: String,

    pub market_take_bound: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    pub min_price: Option<String>,
    pub max_price: Option<String>,
    pub tick_size: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    pub max_qty: Option<String>,
    pub min_qty: Option<String>,
    pub step_size: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketLotSizeFilter {
    pub max_qty: Option<String>,
    pub min_qty: Option<String>,
    pub step_size: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumOrdersFilter {
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumAlgoOrdersFilter {
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PercentPriceFilter {
    pub multiplier_up: Option<String>,
    pub multiplier_down: Option<String>,
    pub multiplier_decimal: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "filterType")]
pub enum Filter {
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter(PriceFilter),

    #[serde(rename = "LOT_SIZE")]
    LotSizeFilter(LotSizeFilter),

    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSizeFilter(MarketLotSizeFilter),

    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrdersFilter(MaxNumOrdersFilter),

    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrdersFilter(MaxNumAlgoOrdersFilter),

    #[serde(rename = "PERCENT_PRICE")]
    PercentPriceFilter(PercentPriceFilter),

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FilterType {
    #[serde(rename = "LOT_SIZE")]
    LotSize,

    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize,

    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders,

    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders,

    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice,

    #[serde(rename = "PRICE_FILTER")]
    PriceFilter,
}

#[derive(Debug, Deserialize)]
pub enum QuoteAsset {
    #[serde(rename = "USD")]
    Usd,
}

/// Represents the response from the Binance Coin-M Futures Exchange Information endpoint.
///
/// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Exchange-Information>
#[derive(Debug, Deserialize)]
pub struct ExchangeInfoResponse {
    /// The timezone of the exchange (e.g., "UTC").
    pub timezone: String,

    /// The server time in milliseconds.
    #[serde(rename = "serverTime")]
    pub server_time: u64,

    /// The list of rate limits applied to the account or exchange.
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,

    /// The list of contract symbols available on the exchange.
    pub symbols: Vec<Symbol>,

    /// The list of exchange-wide filters (currently always empty for this endpoint).
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Vec<serde_json::Value>,
}

/// Represents a rate limit object in the exchange info response.
#[derive(Debug, Deserialize)]
pub struct RateLimit {
    /// The type of rate limit (e.g., "REQUEST_WEIGHT", "ORDERS").
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: RateLimitType,

    /// The interval for the rate limit (e.g., "MINUTE").
    pub interval: RateLimitInterval,

    /// The number of intervals.
    #[serde(rename = "intervalNum")]
    pub interval_num: u32,

    /// The maximum number of requests or orders allowed in the interval.
    pub limit: u32,
}

impl RestClient {
    /// Fetches current exchange trading rules and symbol information.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Exchange-Information>
    /// Corresponds to endpoint GET /dapi/v1/exchangeInfo.
    /// Weight: 1
    pub async fn get_exchange_info(&self) -> RestResult<ExchangeInfoResponse> {
        self.send_request("/dapi/v1/exchangeInfo", reqwest::Method::GET, None::<()>, 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_info_response_deserialization() {
        let json_data = r#"{"timezone":"UTC","serverTime":1750968011440,"rateLimits":[{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":2400},{"rateLimitType":"ORDERS","interval":"MINUTE","intervalNum":1,"limit":1200}],"exchangeFilters":[],"symbols":[{"symbol":"BTCUSD_PERP","pair":"BTCUSD","contractType":"PERPETUAL","deliveryDate":4133404800000,"onboardDate":1597042800000,"contractStatus":"TRADING","contractSize":100,"marginAsset":"BTC","maintMarginPercent":"2.5000","requiredMarginPercent":"5.0000","baseAsset":"BTC","quoteAsset":"USD","pricePrecision":1,"quantityPrecision":0,"baseAssetPrecision":8,"quotePrecision":8,"equalQtyPrecision":4,"maxMoveOrderLimit":10000,"triggerProtect":"0.0500","underlyingType":"COIN","underlyingSubType":["PoW"],"filters":[{"minPrice":"1000","maxPrice":"4520958","filterType":"PRICE_FILTER","tickSize":"0.1"},{"stepSize":"1","filterType":"LOT_SIZE","maxQty":"1000000","minQty":"1"},{"stepSize":"1","filterType":"MARKET_LOT_SIZE","maxQty":"60000","minQty":"1"},{"limit":200,"filterType":"MAX_NUM_ORDERS"},{"limit":20,"filterType":"MAX_NUM_ALGO_ORDERS"},{"multiplierDown":"0.9500","multiplierUp":"1.0500","multiplierDecimal":"4","filterType":"PERCENT_PRICE"}],"orderTypes":["LIMIT","MARKET","STOP","STOP_MARKET","TAKE_PROFIT","TAKE_PROFIT_MARKET","TRAILING_STOP_MARKET"],"timeInForce":["GTC","IOC","FOK","GTX"],"liquidationFee":"0.015000","marketTakeBound":"0.05"}]}"#;

        let result: Result<ExchangeInfoResponse, _> = serde_json::from_str(json_data);

        assert!(
            result.is_ok(),
            "Failed to deserialize ExchangeInfoResponse: {:?}",
            result.err()
        );

        let response = result.unwrap();

        // Validate basic fields
        assert_eq!(response.timezone, "UTC");
        assert_eq!(response.server_time, 1750968011440);
        assert_eq!(response.rate_limits.len(), 2);
        assert_eq!(response.symbols.len(), 1);
        assert!(response.exchange_filters.is_empty());

        // Validate rate limits
        let request_weight_limit = &response.rate_limits[0];
        assert_eq!(request_weight_limit.limit, 2400);
        assert_eq!(request_weight_limit.interval_num, 1);

        let orders_limit = &response.rate_limits[1];
        assert_eq!(orders_limit.limit, 1200);
        assert_eq!(orders_limit.interval_num, 1);

        // Validate symbol
        let symbol = &response.symbols[0];
        assert_eq!(symbol.symbol, "BTCUSD_PERP");
        assert_eq!(symbol.pair, "BTCUSD");
        assert_eq!(symbol.contract_size, 100);
        assert_eq!(symbol.margin_asset, "BTC");
        assert_eq!(symbol.base_asset, "BTC");
        assert_eq!(symbol.price_precision, 1);
        assert_eq!(symbol.quantity_precision, 0);
        assert_eq!(symbol.base_asset_precision, 8);
        assert_eq!(symbol.quote_precision, 8);
        assert_eq!(symbol.equal_qty_precision, 4);
        assert_eq!(symbol.max_move_order_limit, 10000);
        assert_eq!(symbol.trigger_protect, "0.0500");
        assert_eq!(symbol.underlying_sub_type, vec!["PoW"]);
        assert_eq!(symbol.liquidation_fee, "0.015000");
        assert_eq!(symbol.market_take_bound, "0.05");

        // Validate filters
        assert_eq!(symbol.filters.len(), 6);

        // Validate order types
        assert_eq!(symbol.order_types.len(), 7);

        // Validate time in force
        assert_eq!(symbol.time_in_force.len(), 4);
    }
}
