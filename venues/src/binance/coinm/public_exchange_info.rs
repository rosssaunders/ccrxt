use serde::{Serialize, Deserialize};
use super::{
    api_errors::BinanceCoinMResult,
    public_rest::BinanceCoinMPublicRest,
    types::BinanceResponse,
    common::request::send_request,
    enums::{
        RateLimitType,
        RateLimitInterval,
        ExchangeFilterType,
        SymbolStatus,
        ContractType,
        UnderlyingType,
        SymbolFilterType,
    },
};

/// Response struct for exchange info endpoint
/// Contains information about the exchange's current configuration and trading rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeInfoResponse {
    /// Server timezone
    pub timezone: String,
    /// Current server time in milliseconds
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    /// Rate limit rules
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,
    /// Exchange level filters
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Vec<ExchangeFilter>,
    /// Trading pair information
    pub symbols: Vec<Symbol>,
}

/// Rate limit rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// Type of rate limit
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: RateLimitType,
    /// Time interval for the rate limit
    pub interval: RateLimitInterval,
    /// Number of intervals
    #[serde(rename = "intervalNum")]
    pub interval_num: i32,
    /// Maximum number of requests allowed in the interval
    pub limit: i32,
}

/// Exchange level filter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeFilter {
    /// Type of exchange filter
    #[serde(rename = "filterType")]
    pub filter_type: ExchangeFilterType,
    /// Filter specific data
    #[serde(flatten)]
    pub filter_data: serde_json::Value,
}

/// Trading pair information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    /// Trading pair symbol
    pub symbol: String,
    /// Trading pair name
    pub pair: String,
    /// Type of contract
    #[serde(rename = "contractType")]
    pub contract_type: ContractType,
    /// Delivery date in milliseconds
    #[serde(rename = "deliveryDate")]
    pub delivery_date: i64,
    /// Onboarding date in milliseconds
    #[serde(rename = "onboardDate")]
    pub onboard_date: i64,
    /// Current status of the symbol
    pub status: SymbolStatus,
    /// Maintenance margin percentage
    #[serde(rename = "maintMarginPercent")]
    pub maint_margin_percent: String,
    /// Required margin percentage
    #[serde(rename = "requiredMarginPercent")]
    pub required_margin_percent: String,
    /// Base asset of the trading pair
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    /// Quote asset of the trading pair
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    /// Margin asset
    #[serde(rename = "marginAsset")]
    pub margin_asset: String,
    /// Price precision
    #[serde(rename = "pricePrecision")]
    pub price_precision: i32,
    /// Quantity precision
    #[serde(rename = "quantityPrecision")]
    pub quantity_precision: i32,
    /// Base asset precision
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: i32,
    /// Quote asset precision
    #[serde(rename = "quotePrecision")]
    pub quote_precision: i32,
    /// Type of underlying asset
    #[serde(rename = "underlyingType")]
    pub underlying_type: UnderlyingType,
    /// Settlement plan
    #[serde(rename = "settlePlan")]
    pub settle_plan: i32,
    /// Trigger protection
    #[serde(rename = "triggerProtect")]
    pub trigger_protect: String,
    /// Trading rules and filters
    pub filters: Vec<SymbolFilter>,
}

/// Symbol level filter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolFilter {
    /// Type of symbol filter
    #[serde(rename = "filterType")]
    pub filter_type: SymbolFilterType,
    /// Filter specific data
    #[serde(flatten)]
    pub filter_data: serde_json::Value,
}

impl BinanceCoinMPublicRest {
    /// Get exchange information including trading rules and filters
    /// 
    /// # Returns
    /// * `BinanceCoinMResult<BinanceResponse<ExchangeInfoResponse>>` - Exchange information response
    pub async fn get_exchange_info(&self) -> BinanceCoinMResult<BinanceResponse<ExchangeInfoResponse>> {
        let endpoint = "/dapi/v1/exchangeInfo";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            None,
            None,
            || self.rate_limiter.check_weight_limit("exchangeInfo", 1)
        ).await?;
        Ok(response.data)
    }
}