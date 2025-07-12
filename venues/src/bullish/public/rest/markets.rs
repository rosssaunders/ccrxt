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
    /// Market symbol
    pub symbol: String,
    /// Market display name
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Base asset symbol
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    /// Quote asset symbol
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    /// Market type
    #[serde(rename = "type")]
    pub market_type: MarketType,
    /// Market status
    pub status: MarketStatus,
    /// Whether trading is enabled
    #[serde(rename = "tradingEnabled")]
    pub trading_enabled: bool,
    /// Minimum order quantity
    #[serde(rename = "minOrderQty")]
    pub min_order_qty: String,
    /// Maximum order quantity
    #[serde(rename = "maxOrderQty")]
    pub max_order_qty: String,
    /// Quantity increment (step size)
    #[serde(rename = "qtyIncrement")]
    pub qty_increment: String,
    /// Minimum order price
    #[serde(rename = "minOrderPrice")]
    pub min_order_price: String,
    /// Maximum order price
    #[serde(rename = "maxOrderPrice")]
    pub max_order_price: String,
    /// Price increment (tick size)
    #[serde(rename = "priceIncrement")]
    pub price_increment: String,
    /// Minimum notional value
    #[serde(rename = "minNotional")]
    pub min_notional: String,
    /// Maximum notional value
    #[serde(rename = "maxNotional")]
    pub max_notional: String,
    /// Maker fee rate
    #[serde(rename = "makerFeeRate")]
    pub maker_fee_rate: String,
    /// Taker fee rate
    #[serde(rename = "takerFeeRate")]
    pub taker_fee_rate: String,
    /// Last trade price
    #[serde(rename = "lastPrice")]
    pub last_price: Option<String>,
    /// 24h price change percentage
    #[serde(rename = "priceChange24h")]
    pub price_change_24h: Option<String>,
    /// 24h high price
    #[serde(rename = "high24h")]
    pub high_24h: Option<String>,
    /// 24h low price
    #[serde(rename = "low24h")]
    pub low_24h: Option<String>,
    /// 24h volume in base asset
    #[serde(rename = "volume24h")]
    pub volume_24h: Option<String>,
    /// 24h volume in quote asset
    #[serde(rename = "quoteVolume24h")]
    pub quote_volume_24h: Option<String>,
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
            "symbol": "BTCUSDC",
            "displayName": "BTC/USDC",
            "baseAsset": "BTC",
            "quoteAsset": "USDC",
            "type": "SPOT",
            "status": "ACTIVE",
            "tradingEnabled": true,
            "minOrderQty": "0.0001",
            "maxOrderQty": "1000",
            "qtyIncrement": "0.0001",
            "minOrderPrice": "0.01",
            "maxOrderPrice": "1000000",
            "priceIncrement": "0.01",
            "minNotional": "10",
            "maxNotional": "1000000",
            "makerFeeRate": "0.001",
            "takerFeeRate": "0.002",
            "lastPrice": "30000.0",
            "priceChange24h": "2.5",
            "high24h": "31000.0",
            "low24h": "29000.0",
            "volume24h": "100.0",
            "quoteVolume24h": "3000000.0"
        }"#;

        let market: Market = serde_json::from_str(json).unwrap();
        assert_eq!(market.symbol, "BTCUSDC");
        assert_eq!(market.base_asset, "BTC");
        assert_eq!(market.quote_asset, "USDC");
        assert_eq!(market.market_type, MarketType::Spot);
        assert_eq!(market.status, MarketStatus::Active);
        assert!(market.trading_enabled);
    }
}
