use serde::{Deserialize, Deserializer, Serialize};

/// Endpoint for Binance USD-M Exchange Information
use crate::binance::usdm::public_client::RestClient;
use crate::binance::usdm::{
    ContractStatus, ContractType, MarginAsset, OrderType, RestResult, TimeInForce, UnderlyingType,
    rate_limit::{RateLimitInterval, RateLimitType},
};

const EXCHANGE_INFO_ENDPOINT: &str = "/fapi/v1/exchangeInfo";

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

/// Symbol information for USD-M futures exchange.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,

    /// Trading pair (e.g., "BTCUSDT").
    pub pair: String,

    /// Optional contract type (e.g., PERPETUAL) or None if not applicable.
    #[serde(deserialize_with = "deserialize_optional_contract_type")]
    pub contract_type: Option<ContractType>,

    /// Delivery date in milliseconds since epoch.
    pub delivery_date: i64,

    /// Onboard date in milliseconds since epoch.
    pub onboard_date: i64,

    /// Current status of the contract.
    #[serde(rename = "status")]
    pub contract_status: ContractStatus,

    /// Contract size (quantity per contract), if available.
    pub contract_size: Option<i64>,

    /// Margin asset used for the contract.
    pub margin_asset: MarginAsset,

    /// Maintenance margin percentage (as decimal string).
    pub maint_margin_percent: String,

    /// Required margin percentage (as decimal string).
    pub required_margin_percent: String,

    /// Base asset symbol of the contract.
    pub base_asset: String,

    /// Quote asset symbol of the contract.
    pub quote_asset: QuoteAsset,

    /// Price precision (number of decimal places).
    pub price_precision: i64,

    /// Quantity precision (number of decimal places).
    pub quantity_precision: i64,

    /// Base asset precision (decimal places for rounding).
    pub base_asset_precision: i64,

    /// Quote precision (number of decimal places for quote asset).
    pub quote_precision: i64,

    /// Equal quantity precision, if applicable.
    pub equal_qty_precision: Option<i64>,

    /// Maximum limit for MOVE orders.
    pub max_move_order_limit: i64,

    /// Trigger price protection threshold (decimal string).
    pub trigger_protect: String,

    /// Underlying type of the contract.
    pub underlying_type: UnderlyingType,

    /// Underlying sub-types of the contract.
    pub underlying_sub_type: Vec<String>,

    /// List of filters applied to this symbol.
    pub filters: Vec<Filter>,

    /// Supported order types for this symbol.
    pub order_types: Vec<OrderType>,

    /// Supported time-in-force options for this symbol.
    pub time_in_force: Vec<TimeInForce>,

    /// Liquidation fee rate (as decimal string).
    pub liquidation_fee: String,

    /// Market take bound rate (as decimal string).
    pub market_take_bound: String,
}

/// Filter details for price limits on a symbol.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    /// Minimum price allowed. Optional.
    pub min_price: Option<String>,

    /// Maximum price allowed. Optional.
    pub max_price: Option<String>,

    /// Tick size for price increments. Optional.
    pub tick_size: Option<String>,
}

/// Filter details for lot size limits on a symbol.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    /// Maximum order quantity. Optional.
    pub max_qty: Option<String>,

    /// Minimum order quantity. Optional.
    pub min_qty: Option<String>,

    /// Step size for quantity increments. Optional.
    pub step_size: Option<String>,
}

/// Filter details for market lot size limits on a symbol.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MarketLotSizeFilter {
    /// Maximum market order quantity. Optional.
    pub max_qty: Option<String>,

    /// Minimum market order quantity. Optional.
    pub min_qty: Option<String>,

    /// Step size for market order quantity increments. Optional.
    pub step_size: Option<String>,
}

/// Filter for maximum number of orders allowed on a symbol.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumOrdersFilter {
    /// Maximum number of orders. Optional.
    pub limit: Option<i64>,
}

/// Filter for maximum number of algorithmic orders allowed on a symbol.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MaxNumAlgoOrdersFilter {
    /// Maximum number of algo orders. Optional.
    pub limit: Option<i64>,
}

/// Filter for percentage-based price protection on a symbol.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PercentPriceFilter {
    /// Multiplier up for price. Optional.
    pub multiplier_up: Option<String>,

    /// Multiplier down for price. Optional.
    pub multiplier_down: Option<String>,

    /// Decimal precision for multiplier. Optional.
    pub multiplier_decimal: Option<String>,
}

/// Enum representing the various filters applied to a symbol.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "filterType")]
pub enum Filter {
    /// Price filter variant.
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter(PriceFilter),

    /// Lot size filter variant.
    #[serde(rename = "LOT_SIZE")]
    LotSizeFilter(LotSizeFilter),

    /// Market lot size filter variant.
    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSizeFilter(MarketLotSizeFilter),

    /// Maximum number of orders filter variant.
    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrdersFilter(MaxNumOrdersFilter),

    /// Maximum number of algo orders filter variant.
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrdersFilter(MaxNumAlgoOrdersFilter),

    /// Percentage price filter variant.
    #[serde(rename = "PERCENT_PRICE")]
    PercentPriceFilter(PercentPriceFilter),

    /// Catch-all for unknown filter types.
    #[serde(other)]
    Unknown,
}

/// Enum of filter types for serialization and usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FilterType {
    /// Lot size filter type.
    #[serde(rename = "LOT_SIZE")]
    LotSize,

    /// Market lot size filter type.
    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize,

    /// Maximum number of algo orders filter type.
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders,

    /// Maximum number of orders filter type.
    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders,

    /// Percentage price filter type.
    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice,

    /// Price filter type.
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter,
}

/// Quote asset for USD-M futures (e.g., USDT, USDC, BTC).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteAsset {
    /// Tether USD.
    USDT,
    /// USD Coin.
    USDC,
    /// Fiat Digital USD.
    FDUSD,
    /// Binance Fiat USD.
    BFUSD,
    /// Binance NFCR.
    BNFCR,
    /// Binance LDUSDT.
    LDUSDT,
    /// Bitcoin.
    BTC,
}

/// Response for the Exchange Information endpoint.
///
/// Contains exchange rules, assets, symbols, and rate limits.
#[derive(Debug, Clone, Deserialize)]
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

/// Asset available for margin trading returned by the Exchange Information endpoint.
#[derive(Debug, Clone, Deserialize)]
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

/// Rate limit object in the Exchange Information response.
#[derive(Debug, Clone, Deserialize)]
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
    /// Exchange Information
    ///
    /// Fetches current exchange trading rules and symbol information.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Exchange-Information)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// ExchangeInfoResponse - contains exchange rules, assets, symbols, and rate limits.
    pub async fn get_exchange_info(&self) -> RestResult<ExchangeInfoResponse> {
        self.send_get_request(EXCHANGE_INFO_ENDPOINT, Some(()), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_asset_deserialization() {
        assert!(matches!(
            serde_json::from_str::<QuoteAsset>(r#""USDT""#).unwrap(),
            QuoteAsset::USDT
        ));
        assert!(matches!(
            serde_json::from_str::<QuoteAsset>(r#""USDC""#).unwrap(),
            QuoteAsset::USDC
        ));
        assert!(matches!(
            serde_json::from_str::<QuoteAsset>(r#""FDUSD""#).unwrap(),
            QuoteAsset::FDUSD
        ));
        assert!(matches!(
            serde_json::from_str::<QuoteAsset>(r#""BFUSD""#).unwrap(),
            QuoteAsset::BFUSD
        ));
        assert!(matches!(
            serde_json::from_str::<QuoteAsset>(r#""BNFCR""#).unwrap(),
            QuoteAsset::BNFCR
        ));
        assert!(matches!(
            serde_json::from_str::<QuoteAsset>(r#""LDUSDT""#).unwrap(),
            QuoteAsset::LDUSDT
        ));
        assert!(matches!(
            serde_json::from_str::<QuoteAsset>(r#""BTC""#).unwrap(),
            QuoteAsset::BTC
        ));
    }

    #[test]
    fn test_filter_deserialization() {
        let price_filter = r#"{
            "filterType": "PRICE_FILTER",
            "minPrice": "0.01",
            "maxPrice": "1000000",
            "tickSize": "0.01"
        }"#;
        let filter: Filter = serde_json::from_str(price_filter).unwrap();
        assert!(matches!(filter, Filter::PriceFilter(_)));

        let lot_size = r#"{
            "filterType": "LOT_SIZE",
            "maxQty": "10000",
            "minQty": "0.001",
            "stepSize": "0.001"
        }"#;
        let filter: Filter = serde_json::from_str(lot_size).unwrap();
        assert!(matches!(filter, Filter::LotSizeFilter(_)));

        let unknown = r#"{
            "filterType": "UNKNOWN_FILTER",
            "someField": "value"
        }"#;
        let filter: Filter = serde_json::from_str(unknown).unwrap();
        assert!(matches!(filter, Filter::Unknown));
    }

    #[test]
    fn test_rate_limit_deserialization() {
        let json = r#"{
            "rateLimitType": "REQUEST_WEIGHT",
            "interval": "MINUTE",
            "intervalNum": 1,
            "limit": 2400
        }"#;
        let rate_limit: RateLimit = serde_json::from_str(json).unwrap();
        assert!(matches!(
            rate_limit.rate_limit_type,
            RateLimitType::RequestWeight
        ));
        assert!(matches!(rate_limit.interval, RateLimitInterval::Minute));
        assert_eq!(rate_limit.interval_num, 1);
        assert_eq!(rate_limit.limit, 2400);
    }

    #[test]
    fn test_asset_deserialization() {
        let json = r#"{
            "asset": "USDT",
            "marginAvailable": true,
            "autoAssetExchange": "0"
        }"#;
        let asset: Asset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.asset, "USDT");
        assert!(asset.margin_available);
        assert_eq!(asset.auto_asset_exchange, "0");
    }

    #[test]
    fn test_symbol_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "pair": "BTCUSDT",
            "contractType": "PERPETUAL",
            "deliveryDate": 4133404800000,
            "onboardDate": 1569398400000,
            "status": "TRADING",
            "contractSize": 1,
            "marginAsset": "USDT",
            "maintMarginPercent": "2.5000",
            "requiredMarginPercent": "5.0000",
            "baseAsset": "BTC",
            "quoteAsset": "USDT",
            "pricePrecision": 2,
            "quantityPrecision": 3,
            "baseAssetPrecision": 8,
            "quotePrecision": 8,
            "equalQtyPrecision": 4,
            "maxMoveOrderLimit": 10000,
            "triggerProtect": "0.0500",
            "underlyingType": "COIN",
            "underlyingSubType": ["PoR"],
            "filters": [
                {
                    "filterType": "PRICE_FILTER",
                    "minPrice": "556.80",
                    "maxPrice": "4529764",
                    "tickSize": "0.10"
                }
            ],
            "orderTypes": ["LIMIT", "MARKET", "STOP", "STOP_MARKET", "TAKE_PROFIT", "TAKE_PROFIT_MARKET", "TRAILING_STOP_MARKET"],
            "timeInForce": ["GTC", "IOC", "FOK", "GTX", "GTD"],
            "liquidationFee": "0.015000",
            "marketTakeBound": "0.05"
        }"#;

        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol.symbol, "BTCUSDT");
        assert_eq!(symbol.pair, "BTCUSDT");
        assert!(matches!(
            symbol.contract_type,
            Some(ContractType::Perpetual)
        ));
        assert_eq!(symbol.delivery_date, 4133404800000);
        assert_eq!(symbol.onboard_date, 1569398400000);
        assert!(matches!(symbol.contract_status, ContractStatus::Trading));
        assert_eq!(symbol.contract_size, Some(1));
        assert!(matches!(symbol.margin_asset, MarginAsset::Usdt));
        assert_eq!(symbol.maint_margin_percent, "2.5000");
        assert_eq!(symbol.required_margin_percent, "5.0000");
        assert_eq!(symbol.base_asset, "BTC");
        assert!(matches!(symbol.quote_asset, QuoteAsset::USDT));
        assert_eq!(symbol.price_precision, 2);
        assert_eq!(symbol.quantity_precision, 3);
        assert_eq!(symbol.base_asset_precision, 8);
        assert_eq!(symbol.quote_precision, 8);
        assert_eq!(symbol.equal_qty_precision, Some(4));
        assert_eq!(symbol.max_move_order_limit, 10000);
        assert_eq!(symbol.trigger_protect, "0.0500");
        assert!(matches!(symbol.underlying_type, UnderlyingType::Coin));
        assert_eq!(symbol.underlying_sub_type, vec!["PoR"]);
        assert_eq!(symbol.filters.len(), 1);
        assert_eq!(symbol.order_types.len(), 7);
        assert_eq!(symbol.time_in_force.len(), 5);
        assert_eq!(symbol.liquidation_fee, "0.015000");
        assert_eq!(symbol.market_take_bound, "0.05");
    }

    #[test]
    fn test_symbol_with_empty_contract_type() {
        let json = r#"{
            "symbol": "BTCUSDT_240329",
            "pair": "BTCUSDT",
            "contractType": "",
            "deliveryDate": 1711699200000,
            "onboardDate": 1703836800000,
            "status": "TRADING",
            "marginAsset": "USDT",
            "maintMarginPercent": "2.5000",
            "requiredMarginPercent": "5.0000",
            "baseAsset": "BTC",
            "quoteAsset": "USDT",
            "pricePrecision": 2,
            "quantityPrecision": 3,
            "baseAssetPrecision": 8,
            "quotePrecision": 8,
            "maxMoveOrderLimit": 10000,
            "triggerProtect": "0.0500",
            "underlyingType": "COIN",
            "underlyingSubType": [],
            "filters": [],
            "orderTypes": ["LIMIT"],
            "timeInForce": ["GTC"],
            "liquidationFee": "0.015000",
            "marketTakeBound": "0.05"
        }"#;

        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol.contract_type, None);
    }

    #[test]
    fn test_exchange_info_response_deserialization() {
        let json = r#"{
            "timezone": "UTC",
            "rateLimits": [
                {
                    "rateLimitType": "REQUEST_WEIGHT",
                    "interval": "MINUTE",
                    "intervalNum": 1,
                    "limit": 2400
                },
                {
                    "rateLimitType": "ORDERS",
                    "interval": "MINUTE",
                    "intervalNum": 1,
                    "limit": 1200
                }
            ],
            "exchangeFilters": [],
            "assets": [
                {
                    "asset": "USDT",
                    "marginAvailable": true,
                    "autoAssetExchange": "0"
                },
                {
                    "asset": "BTC",
                    "marginAvailable": false,
                    "autoAssetExchange": "-0.00100000"
                }
            ],
            "symbols": [
                {
                    "symbol": "BTCUSDT",
                    "pair": "BTCUSDT",
                    "contractType": "PERPETUAL",
                    "deliveryDate": 4133404800000,
                    "onboardDate": 1569398400000,
                    "status": "TRADING",
                    "marginAsset": "USDT",
                    "maintMarginPercent": "2.5000",
                    "requiredMarginPercent": "5.0000",
                    "baseAsset": "BTC",
                    "quoteAsset": "USDT",
                    "pricePrecision": 2,
                    "quantityPrecision": 3,
                    "baseAssetPrecision": 8,
                    "quotePrecision": 8,
                    "maxMoveOrderLimit": 10000,
                    "triggerProtect": "0.0500",
                    "underlyingType": "COIN",
                    "underlyingSubType": ["PoR"],
                    "filters": [],
                    "orderTypes": ["LIMIT", "MARKET"],
                    "timeInForce": ["GTC", "IOC"],
                    "liquidationFee": "0.015000",
                    "marketTakeBound": "0.05"
                }
            ]
        }"#;

        let response: ExchangeInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.timezone, "UTC");
        assert_eq!(response.rate_limits.len(), 2);
        assert_eq!(response.exchange_filters.len(), 0);
        assert_eq!(response.assets.len(), 2);
        assert_eq!(response.symbols.len(), 1);
    }

    #[test]
    fn test_filter_type_deserialization() {
        assert!(matches!(
            serde_json::from_str::<FilterType>(r#""LOT_SIZE""#).unwrap(),
            FilterType::LotSize
        ));
        assert!(matches!(
            serde_json::from_str::<FilterType>(r#""PRICE_FILTER""#).unwrap(),
            FilterType::PriceFilter
        ));
        assert!(matches!(
            serde_json::from_str::<FilterType>(r#""PERCENT_PRICE""#).unwrap(),
            FilterType::PercentPrice
        ));
    }

    #[test]
    fn test_all_filter_types() {
        // Test each filter variant
        let market_lot_size = r#"{
            "filterType": "MARKET_LOT_SIZE",
            "maxQty": "10000",
            "minQty": "0.001",
            "stepSize": "0.001"
        }"#;
        let filter: Filter = serde_json::from_str(market_lot_size).unwrap();
        assert!(matches!(filter, Filter::MarketLotSizeFilter(_)));

        let max_num_orders = r#"{
            "filterType": "MAX_NUM_ORDERS",
            "limit": 200
        }"#;
        let filter: Filter = serde_json::from_str(max_num_orders).unwrap();
        assert!(matches!(filter, Filter::MaxNumOrdersFilter(_)));

        let max_num_algo_orders = r#"{
            "filterType": "MAX_NUM_ALGO_ORDERS",
            "limit": 10
        }"#;
        let filter: Filter = serde_json::from_str(max_num_algo_orders).unwrap();
        assert!(matches!(filter, Filter::MaxNumAlgoOrdersFilter(_)));

        let percent_price = r#"{
            "filterType": "PERCENT_PRICE",
            "multiplierUp": "5",
            "multiplierDown": "0.2",
            "multiplierDecimal": "4"
        }"#;
        let filter: Filter = serde_json::from_str(percent_price).unwrap();
        assert!(matches!(filter, Filter::PercentPriceFilter(_)));
    }
}
