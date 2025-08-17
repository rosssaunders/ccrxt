//! Markets endpoint for Bullish Exchange API

use serde::{Deserialize, Deserializer, Serialize, de::Error as DeError};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL path for markets
const MARKETS_ENDPOINT: &str = "/trading-api/v1/markets";

/// Endpoint URL path for single market (with parameter)
const SINGLE_MARKET_ENDPOINT: &str = "/trading-api/v1/markets/{}";

/// Market status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MarketStatus {
    Active,
    Inactive,
    Suspended,
}

/// Market type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MarketType {
    Spot,
    Perpetual,
    #[serde(alias = "DATED_FUTURE")] // Accept both DATEDFUTURE and DATED_FUTURE from API
    DatedFuture,
}

/// Market information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    /// Unique market ID
    pub market_id: String,

    /// Market symbol
    pub symbol: String,

    /// Base asset symbol (only for spot markets)
    pub base_symbol: Option<String>,

    /// Underlying base asset symbol (only for derivative markets)
    pub underlying_base_symbol: Option<String>,

    /// Quote asset symbol (only for spot markets)
    pub quote_symbol: Option<String>,

    /// Underlying quote asset symbol (only for derivative markets)
    pub underlying_quote_symbol: Option<String>,

    /// Quote asset ID
    pub quote_asset_id: String,

    /// Base asset ID
    pub base_asset_id: String,

    /// Quote precision (API may return number or string)
    #[serde(deserialize_with = "de_u32_from_string_or_number")]
    pub quote_precision: u32,

    /// Base precision (API may return number or string)
    #[serde(deserialize_with = "de_u32_from_string_or_number")]
    pub base_precision: u32,

    /// Price precision (number or string)
    #[serde(deserialize_with = "de_u32_from_string_or_number")]
    pub price_precision: u32,

    /// Quantity precision (number or string)
    #[serde(deserialize_with = "de_u32_from_string_or_number")]
    pub quantity_precision: u32,

    /// Cost precision (number or string)
    #[serde(deserialize_with = "de_u32_from_string_or_number")]
    pub cost_precision: u32,

    /// Buffer range of limit price from the last traded price
    pub price_buffer: String,

    /// Minimum quantity limit
    pub min_quantity_limit: String,

    /// Maximum quantity limit
    pub max_quantity_limit: String,

    /// Maximum price limit
    pub max_price_limit: Option<String>,

    /// Minimum price limit
    pub min_price_limit: Option<String>,

    /// Maximum cost limit
    pub max_cost_limit: Option<String>,

    /// Minimum cost limit
    pub min_cost_limit: Option<String>,

    /// Time zone
    pub time_zone: String,

    /// Tick size
    pub tick_size: String,

    /// Liquidity tick size
    pub liquidity_tick_size: String,

    /// Liquidity precision (number or string)
    #[serde(deserialize_with = "de_u32_from_string_or_number")]
    pub liquidity_precision: u32,

    /// Rounding correction factor for market
    pub rounding_correction_factor: String,

    /// Minimum amount required to invest liquidity to market
    pub maker_min_liquidity_addition: String,

    /// Order types (e.g. "LMT", "MKT", "STOP_LIMIT", "POST_ONLY")
    pub order_types: Vec<String>,

    /// Spot trading enabled (only for spot markets)
    pub spot_trading_enabled: bool,

    /// Margin trading enabled (only for spot markets)
    pub margin_trading_enabled: bool,

    /// Market enabled
    pub market_enabled: bool,

    /// Able to create order
    pub create_order_enabled: bool,

    /// Able to cancel order
    pub cancel_order_enabled: bool,

    /// Able to invest liquidity to market
    pub liquidity_invest_enabled: bool,

    /// Able to withdraw liquidity from market
    pub liquidity_withdraw_enabled: bool,

    /// Identifier to the trade fee assigned to this market (can be a string like "default")
    pub fee_group_id: String,

    /// All available fee tiers
    pub fee_tiers: Vec<FeeTier>,

    /// Market type (SPOT, PERPETUAL, DATED_FUTURE)
    pub market_type: MarketType,

    /// Contract multiplier (only for perpetual markets, number or string)
    #[serde(default, deserialize_with = "de_opt_u32_from_string_or_number")]
    pub contract_multiplier: Option<u32>,

    /// Settlement asset symbol (only for perpetual markets)
    pub settlement_asset_symbol: Option<String>,

    /// Cumulative notional value of all open interest for a specific derivative contract
    pub open_interest_usd: Option<String>,

    /// Open interest notional of an account for a specific derivative contract
    pub concentration_risk_threshold_usd: Option<String>,

    /// Percentage of the total open interest for a specific derivative contract
    pub concentration_risk_percentage: Option<String>,

    /// Expiry datetime in ISO 8601 with millisecond format as string
    pub expiry_datetime: Option<String>,
}

/// Fee tier information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeTier {
    /// Fee tier ID
    pub fee_tier_id: String,

    /// Static spread fee
    pub static_spread_fee: String,

    /// Whether dislocation is enabled
    pub is_dislocation_enabled: bool,
}

/// Response for markets query
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketsResponse {
    /// List of markets
    pub data: Vec<Market>,
}

/// Single market response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleMarketResponse {
    /// Market details
    pub data: Market,
}

// Helper deserializers: Bullish may send numeric fields as numbers or strings.
fn de_u32_from_string_or_number<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum U32OrString {
        U32(u32),
        U64(u64),
        I64(i64),
        String(String),
    }

    match U32OrString::deserialize(deserializer)? {
        U32OrString::U32(v) => Ok(v),
        U32OrString::U64(v) => {
            u32::try_from(v).map_err(|_| DeError::custom("value out of range for u32"))
        }
        U32OrString::I64(v) => {
            u32::try_from(v).map_err(|_| DeError::custom("negative or out of range for u32"))
        }
        U32OrString::String(s) => s
            .parse::<u32>()
            .map_err(|_| DeError::custom("invalid u32 string")),
    }
}

fn de_opt_u32_from_string_or_number<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum U32OrString {
        U32(u32),
        U64(u64),
        I64(i64),
        String(String),
    }

    let v = Option::<U32OrString>::deserialize(deserializer)?;
    match v {
        None => Ok(None),
        Some(U32OrString::U32(v)) => Ok(Some(v)),
        Some(U32OrString::U64(v)) => u32::try_from(v)
            .map(Some)
            .map_err(|_| DeError::custom("value out of range for u32")),
        Some(U32OrString::I64(v)) => u32::try_from(v)
            .map(Some)
            .map_err(|_| DeError::custom("negative or out of range for u32")),
        Some(U32OrString::String(s)) => {
            if s.is_empty() {
                Ok(None)
            } else {
                s.parse::<u32>()
                    .map(Some)
                    .map_err(|_| DeError::custom("invalid u32 string"))
            }
        }
    }
}

impl RestClient {
    /// Get all markets
    ///
    /// Retrieve information for all available markets on the exchange.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets)
    ///
    /// # Returns
    /// List of all markets with their trading parameters and statistics
    pub async fn get_markets(&self) -> RestResult<Vec<Market>> {
        self.send_get_request(MARKETS_ENDPOINT, EndpointType::PublicMarkets)
            .await
    }

    /// Get specific market by symbol
    ///
    /// Retrieve detailed information for a specific market.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets/-symbol-)
    ///
    /// # Arguments
    /// * `symbol` - Market symbol
    ///
    /// # Returns
    /// Detailed market information including trading parameters and 24h statistics
    pub async fn get_market(&self, symbol: &str) -> RestResult<Market> {
        let url = SINGLE_MARKET_ENDPOINT.replace("{}", symbol);

        self.send_get_request(&url, EndpointType::PublicMarkets)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_type_serialization() {
        assert_eq!(
            serde_json::to_string(&MarketType::Spot).unwrap(),
            "\"SPOT\""
        );
        // No "Futures" variant exists; only test existing variants.
        assert_eq!(
            serde_json::to_string(&MarketType::Perpetual).unwrap(),
            "\"PERPETUAL\""
        );
    }

    #[test]
    fn test_market_status_serialization() {
        assert_eq!(
            serde_json::to_string(&MarketStatus::Active).unwrap(),
            "\"ACTIVE\""
        );
        assert_eq!(
            serde_json::to_string(&MarketStatus::Inactive).unwrap(),
            "\"INACTIVE\""
        );
        assert_eq!(
            serde_json::to_string(&MarketStatus::Suspended).unwrap(),
            "\"SUSPENDED\""
        );
    }

    #[test]
    fn test_market_deserialization() {
        let json = r#"{
            "marketId": "BTCUSDC",
            "symbol": "BTCUSDC",
            "quoteAssetId": "USDC",
            "baseAssetId": "BTC",
            "quoteSymbol": "USDC",
            "baseSymbol": "BTC",
            "quotePrecision": "2",
            "basePrecision": "8",
            "pricePrecision": "2",
            "quantityPrecision": "4",
            "costPrecision": "2",
            "minQuantityLimit": "0.0001",
            "maxQuantityLimit": "1000",
            "maxPriceLimit": "1000000",
            "minPriceLimit": "0.01",
            "maxCostLimit": "1000000",
            "minCostLimit": "10",
            "timeZone": "UTC",
            "tickSize": "0.01",
            "liquidityTickSize": "0.01",
            "liquidityPrecision": "2",
            "makerFee": "0.001",
            "takerFee": "0.002",
            "roundingCorrectionFactor": "1",
            "makerMinLiquidityAddition": "0",
            "orderTypes": ["LIMIT", "MARKET"],
            "spotTradingEnabled": true,
            "marginTradingEnabled": false,
            "marketEnabled": true,
            "createOrderEnabled": true,
            "cancelOrderEnabled": true,
            "amendOrderEnabled": true,
            "liquidityInvestEnabled": true,
            "liquidityWithdrawEnabled": true,
            "feeTiers": [],
            "marketType": "SPOT",
            "priceBuffer": "0.01",
            "feeGroupId": "default"
        }"#;

        let market: Market = serde_json::from_str(json).unwrap();
        assert_eq!(market.symbol, "BTCUSDC");
        assert_eq!(market.base_asset_id, "BTC");
        assert_eq!(market.quote_asset_id, "USDC");
        assert_eq!(market.market_type, MarketType::Spot);
        assert!(market.market_enabled);
        assert!(market.spot_trading_enabled);
    }
}
