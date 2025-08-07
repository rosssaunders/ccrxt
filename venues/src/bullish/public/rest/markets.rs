//! Markets endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

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
    Futures,
    Options,
    Perpetual,
}

/// Market information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    /// Market ID
    pub market_id: String,

    /// Market symbol
    pub symbol: String,

    /// Quote asset ID
    pub quote_asset_id: String,

    /// Base asset ID
    pub base_asset_id: String,

    /// Quote symbol
    pub quote_symbol: String,

    /// Base symbol
    pub base_symbol: String,

    /// Quote precision
    pub quote_precision: String,

    /// Base precision
    pub base_precision: String,

    /// Price precision
    pub price_precision: String,

    /// Quantity precision
    pub quantity_precision: String,

    /// Cost precision
    pub cost_precision: String,

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

    /// Liquidity precision
    pub liquidity_precision: String,

    /// Maker fee
    pub maker_fee: String,

    /// Taker fee
    pub taker_fee: String,

    /// Rounding correction factor
    pub rounding_correction_factor: String,

    /// Maker minimum liquidity addition
    pub maker_min_liquidity_addition: String,

    /// Order types
    pub order_types: Vec<String>,

    /// Whether spot trading is enabled
    pub spot_trading_enabled: bool,

    /// Whether margin trading is enabled
    pub margin_trading_enabled: bool,

    /// Whether market is enabled
    pub market_enabled: bool,

    /// Whether create order is enabled
    pub create_order_enabled: bool,

    /// Whether cancel order is enabled
    pub cancel_order_enabled: bool,

    /// Whether amend order is enabled
    pub amend_order_enabled: bool,

    /// Whether liquidity invest is enabled
    pub liquidity_invest_enabled: bool,

    /// Whether liquidity withdraw is enabled
    pub liquidity_withdraw_enabled: bool,

    /// Fee tiers
    pub fee_tiers: Vec<FeeTier>,

    /// Market type
    pub market_type: String,

    /// Price buffer
    pub price_buffer: String,

    /// Fee group ID
    pub fee_group_id: String,
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

impl RestClient {
    /// Get all markets
    ///
    /// Retrieve information for all available markets on the exchange.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets
    ///
    /// # Returns
    /// List of all markets with their trading parameters and statistics
    pub async fn get_markets(&self) -> RestResult<Vec<Market>> {
        self.send_request(
            MARKETS_ENDPOINT,
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PublicMarkets,
        )
        .await
    }

    /// Get specific market by symbol
    ///
    /// Retrieve detailed information for a specific market.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets/-symbol-
    ///
    /// # Arguments
    /// * `symbol` - Market symbol
    ///
    /// # Returns
    /// Detailed market information including trading parameters and 24h statistics
    pub async fn get_market(&self, symbol: &str) -> RestResult<Market> {
        let url = SINGLE_MARKET_ENDPOINT.replace("{}", symbol);

        self.send_request(
            &url,
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PublicMarkets,
        )
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
        assert_eq!(
            serde_json::to_string(&MarketType::Futures).unwrap(),
            "\"FUTURES\""
        );
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
        assert_eq!(market.market_type, "SPOT");
        assert!(market.market_enabled);
        assert!(market.spot_trading_enabled);
    }
}
