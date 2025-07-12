use serde::{Deserialize, Deserializer};

use super::RestClient;
use crate::binance::usdm::{
    ContractStatus, ContractType, MarginAsset, OrderType, RestResult, TimeInForce, UnderlyingType,
    rate_limit::{RateLimitInterval, RateLimitType},
};

// Custom deserializer for ContractType that handles empty strings
fn deserialize_optional_contract_type<'de, D>(
    deserializer: D,
) -> Result<Option<ContractType>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        match ContractType::deserialize(serde_json::Value::String(s)) {
            Ok(contract_type) => Ok(Some(contract_type)),
            Err(_) => Ok(None), // If deserialization fails, return None
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,

    pub pair: String,

    #[serde(deserialize_with = "deserialize_optional_contract_type")]
    pub contract_type: Option<ContractType>,

    pub delivery_date: i64,

    pub onboard_date: i64,

    #[serde(rename = "status")]
    pub contract_status: ContractStatus,

    pub contract_size: Option<i64>,

    pub margin_asset: MarginAsset,

    pub maint_margin_percent: String,

    pub required_margin_percent: String,

    pub base_asset: String,

    pub quote_asset: QuoteAsset,

    pub price_precision: i64,

    pub quantity_precision: i64,

    pub base_asset_precision: i64,

    pub quote_precision: i64,

    pub equal_qty_precision: Option<i64>,

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

#[derive(Debug, Deserialize, Clone)]
pub enum QuoteAsset {
    #[serde(rename = "USDT")]
    Usdt,
    #[serde(rename = "USDC")]
    Usdc,
    #[serde(rename = "FDUSD")]
    Fdusd,
    #[serde(rename = "BFUSD")]
    Bfusd,
    #[serde(rename = "BNFCR")]
    Bnfcr,
    #[serde(rename = "LDUSDT")]
    Ldusdt,
    #[serde(rename = "BTC")]
    Btc,
}

/// Represents the response from the Binance USD-M Futures Exchange Information endpoint.
///
/// See: <https://developers.binance.com/docs/derivatives/>
#[derive(Debug, Deserialize)]
pub struct ExchangeInfoResponse {
    /// The timezone of the exchange (e.g., "UTC").
    pub timezone: String,

    /// The list of rate limits applied to the account or exchange.
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,

    /// The list of exchange-wide filters (currently always empty for this endpoint).
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Vec<serde_json::Value>,

    /// The list of assets available for margin trading.
    pub assets: Vec<Asset>,

    /// The list of contract symbols available on the exchange.
    pub symbols: Vec<Symbol>,
}

/// Represents an asset available for margin trading.
#[derive(Debug, Deserialize)]
pub struct Asset {
    /// The asset symbol (e.g., "USDT", "BTC", "ETH").
    pub asset: String,

    /// Whether margin trading is available for this asset.
    #[serde(rename = "marginAvailable")]
    pub margin_available: bool,

    /// Auto asset exchange value.
    #[serde(rename = "autoAssetExchange")]
    pub auto_asset_exchange: String,
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
    /// Corresponds to endpoint GET /fapi/v1/exchangeInfo.
    /// Weight: 1
    pub async fn get_exchange_info(&self) -> RestResult<ExchangeInfoResponse> {
        self.send_request(
            "/fapi/v1/exchangeInfo",
            reqwest::Method::GET,
            None,
            None,
            1, // weight
        )
        .await
    }
}
