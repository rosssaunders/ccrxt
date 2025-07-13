//! Markets endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL path for markets
const ENDPOINT_PATH: &str = "/trading-api/v1/markets";

/// Endpoint URL path for single market (with parameter)
const SINGLE_MARKET_ENDPOINT_PATH: &str = "/trading-api/v1/markets/{}";

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
    #[serde(rename = "marketId")]
    pub market_id: String,
    /// Market symbol
    pub symbol: String,
    /// Quote asset ID
    #[serde(rename = "quoteAssetId")]
    pub quote_asset_id: String,
    /// Base asset ID
    #[serde(rename = "baseAssetId")]
    pub base_asset_id: String,
    /// Quote symbol
    #[serde(rename = "quoteSymbol")]
    pub quote_symbol: String,
    /// Base symbol
    #[serde(rename = "baseSymbol")]
    pub base_symbol: String,
    /// Quote precision
    #[serde(rename = "quotePrecision")]
    pub quote_precision: String,
    /// Base precision
    #[serde(rename = "basePrecision")]
    pub base_precision: String,
    /// Price precision
    #[serde(rename = "pricePrecision")]
    pub price_precision: String,
    /// Quantity precision
    #[serde(rename = "quantityPrecision")]
    pub quantity_precision: String,
    /// Cost precision
    #[serde(rename = "costPrecision")]
    pub cost_precision: String,
    /// Minimum quantity limit
    #[serde(rename = "minQuantityLimit")]
    pub min_quantity_limit: String,
    /// Maximum quantity limit
    #[serde(rename = "maxQuantityLimit")]
    pub max_quantity_limit: String,
    /// Maximum price limit
    #[serde(rename = "maxPriceLimit")]
    pub max_price_limit: Option<String>,
    /// Minimum price limit
    #[serde(rename = "minPriceLimit")]
    pub min_price_limit: Option<String>,
    /// Maximum cost limit
    #[serde(rename = "maxCostLimit")]
    pub max_cost_limit: Option<String>,
    /// Minimum cost limit
    #[serde(rename = "minCostLimit")]
    pub min_cost_limit: Option<String>,
    /// Time zone
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    /// Tick size
    #[serde(rename = "tickSize")]
    pub tick_size: String,
    /// Liquidity tick size
    #[serde(rename = "liquidityTickSize")]
    pub liquidity_tick_size: String,
    /// Liquidity precision
    #[serde(rename = "liquidityPrecision")]
    pub liquidity_precision: String,
    /// Maker fee
    #[serde(rename = "makerFee")]
    pub maker_fee: String,
    /// Taker fee
    #[serde(rename = "takerFee")]
    pub taker_fee: String,
    /// Rounding correction factor
    #[serde(rename = "roundingCorrectionFactor")]
    pub rounding_correction_factor: String,
    /// Maker minimum liquidity addition
    #[serde(rename = "makerMinLiquidityAddition")]
    pub maker_min_liquidity_addition: String,
    /// Order types
    #[serde(rename = "orderTypes")]
    pub order_types: Vec<String>,
    /// Whether spot trading is enabled
    #[serde(rename = "spotTradingEnabled")]
    pub spot_trading_enabled: bool,
    /// Whether margin trading is enabled
    #[serde(rename = "marginTradingEnabled")]
    pub margin_trading_enabled: bool,
    /// Whether market is enabled
    #[serde(rename = "marketEnabled")]
    pub market_enabled: bool,
    /// Whether create order is enabled
    #[serde(rename = "createOrderEnabled")]
    pub create_order_enabled: bool,
    /// Whether cancel order is enabled
    #[serde(rename = "cancelOrderEnabled")]
    pub cancel_order_enabled: bool,
    /// Whether amend order is enabled
    #[serde(rename = "amendOrderEnabled")]
    pub amend_order_enabled: bool,
    /// Whether liquidity invest is enabled
    #[serde(rename = "liquidityInvestEnabled")]
    pub liquidity_invest_enabled: bool,
    /// Whether liquidity withdraw is enabled
    #[serde(rename = "liquidityWithdrawEnabled")]
    pub liquidity_withdraw_enabled: bool,
    /// Fee tiers
    #[serde(rename = "feeTiers")]
    pub fee_tiers: Vec<FeeTier>,
    /// Market type
    #[serde(rename = "marketType")]
    pub market_type: String,
    /// Price buffer
    #[serde(rename = "priceBuffer")]
    pub price_buffer: String,
    /// Fee group ID
    #[serde(rename = "feeGroupId")]
    pub fee_group_id: String,
}

/// Fee tier information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeTier {
    /// Fee tier ID
    #[serde(rename = "feeTierId")]
    pub fee_tier_id: String,
    /// Static spread fee
    #[serde(rename = "staticSpreadFee")]
    pub static_spread_fee: String,
    /// Whether dislocation is enabled
    #[serde(rename = "isDislocationEnabled")]
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
    /// # Returns
    /// List of all markets with their trading parameters and statistics
    ///
    /// https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets
    pub async fn get_markets(&self) -> RestResult<Vec<Market>> {
        self.send_request(
            ENDPOINT_PATH,
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
    /// # Arguments
    /// * `symbol` - Market symbol
    ///
    /// # Returns
    /// Detailed market information including trading parameters and 24h statistics
    ///
    /// https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/markets/-symbol-
    pub async fn get_market(&self, symbol: &str) -> RestResult<Market> {
        let url = SINGLE_MARKET_ENDPOINT_PATH.replace("{}", symbol);

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
