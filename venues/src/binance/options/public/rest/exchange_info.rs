use rust_decimal::Decimal;
use serde::Deserialize;

use crate::binance::options::{OptionsContractType, RestResult};

use super::client::RestClient;

/// Exchange information response
#[derive(Debug, Clone, Deserialize)]
pub struct ExchangeInfoResponse {
    /// Time zone used by the server
    #[serde(rename = "timezone")]
    pub timezone: String,

    /// Current system time
    #[serde(rename = "serverTime")]
    pub server_time: u64,

    /// Option contract underlying asset info
    #[serde(rename = "optionContracts")]
    pub option_contracts: Vec<OptionContract>,

    /// Option asset info
    #[serde(rename = "optionAssets")]
    pub option_assets: Vec<OptionAsset>,

    /// Option trading pair info
    #[serde(rename = "optionSymbols")]
    pub option_symbols: Vec<OptionSymbol>,

    /// Rate limits
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,
}

/// Option contract underlying asset information
#[derive(Debug, Clone, Deserialize)]
pub struct OptionContract {
    /// Base currency
    #[serde(rename = "baseAsset")]
    pub base_asset: String,

    /// Quotation asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,

    /// Name of the underlying asset of the option contract
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Settlement currency
    #[serde(rename = "settleAsset")]
    pub settle_asset: String,
}

/// Option asset information
#[derive(Debug, Clone, Deserialize)]
pub struct OptionAsset {
    /// Asset name
    #[serde(rename = "name")]
    pub name: String,
}

/// Option trading pair information
#[derive(Debug, Clone, Deserialize)]
pub struct OptionSymbol {
    /// Expiry time
    #[serde(rename = "expiryDate")]
    pub expiry_date: u64,

    /// Filters
    #[serde(rename = "filters")]
    pub filters: Vec<Filter>,

    /// Trading pair name
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Direction: CALL long, PUT short
    #[serde(rename = "side")]
    pub side: OptionsContractType,

    /// Strike price
    #[serde(rename = "strikePrice")]
    pub strike_price: Decimal,

    /// Underlying asset of the contract
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Contract unit, the quantity of the underlying asset represented by a single contract
    #[serde(rename = "unit")]
    pub unit: u32,

    /// Maker commission rate
    #[serde(rename = "makerFeeRate")]
    pub maker_fee_rate: Decimal,

    /// Taker commission rate
    #[serde(rename = "takerFeeRate")]
    pub taker_fee_rate: Decimal,

    /// Minimum order quantity
    #[serde(rename = "minQty")]
    pub min_qty: Decimal,

    /// Maximum order quantity
    #[serde(rename = "maxQty")]
    pub max_qty: Decimal,

    /// Initial Margin Ratio
    #[serde(rename = "initialMargin")]
    pub initial_margin: Decimal,

    /// Maintenance Margin Ratio
    #[serde(rename = "maintenanceMargin")]
    pub maintenance_margin: Decimal,

    /// Min Initial Margin Ratio
    #[serde(rename = "minInitialMargin")]
    pub min_initial_margin: Decimal,

    /// Min Maintenance Margin Ratio
    #[serde(rename = "minMaintenanceMargin")]
    pub min_maintenance_margin: Decimal,

    /// Price precision
    #[serde(rename = "priceScale")]
    pub price_scale: u32,

    /// Quantity precision
    #[serde(rename = "quantityScale")]
    pub quantity_scale: u32,

    /// Quotation asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
}

/// Trading rule filter
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
    #[serde(rename = "LOT_SIZE")]
    LotSize {
        #[serde(rename = "minQty")]
        min_qty: Decimal,
        #[serde(rename = "maxQty")]
        max_qty: Decimal,
        #[serde(rename = "stepSize")]
        step_size: Decimal,
    },
}

/// Rate limit information
#[derive(Debug, Clone, Deserialize)]
pub struct RateLimit {
    /// Rate limit type
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,

    /// Interval type
    #[serde(rename = "interval")]
    pub interval: String,

    /// Interval number
    #[serde(rename = "intervalNum")]
    pub interval_num: u32,

    /// Limit value
    #[serde(rename = "limit")]
    pub limit: u32,
}

impl RestClient {
    /// Get exchange information
    ///
    /// Returns current exchange trading rules and symbol information.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Exchange-Information)
    /// Method: GET /eapi/v1/exchangeInfo
    /// Weight: 1
    /// Security: None
    pub async fn get_exchange_info(&self) -> RestResult<ExchangeInfoResponse> {
        self.send_request("/eapi/v1/exchangeInfo", reqwest::Method::GET, None, None, 1)
            .await
    }
}
