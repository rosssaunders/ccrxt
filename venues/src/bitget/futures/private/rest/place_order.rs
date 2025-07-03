use crate::bitget::{
    BitgetRestClient, HoldSide, MarginCoin, MarginMode, OrderSide, OrderType, ProductType,
    TimeInForce,
};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Trade side for futures orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeSide {
    /// Open position
    #[serde(rename = "open")]
    Open,
    /// Close position
    #[serde(rename = "close")]
    Close,
}

impl fmt::Display for TradeSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeSide::Open => write!(f, "open"),
            TradeSide::Close => write!(f, "close"),
        }
    }
}

/// Whether to reduce only
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReduceOnly {
    /// Reduce only - Yes
    #[serde(rename = "YES")]
    Yes,
    /// Reduce only - No
    #[serde(rename = "NO")]
    No,
}

impl fmt::Display for ReduceOnly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReduceOnly::Yes => write!(f, "YES"),
            ReduceOnly::No => write!(f, "NO"),
        }
    }
}

/// Request for placing a futures order
#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrderRequest {
    /// Trading pair, e.g. ETHUSDT
    pub symbol: String,

    /// Product type
    #[serde(rename = "productType")]
    pub product_type: ProductType,

    /// Position mode (isolated/crossed margin)
    #[serde(rename = "marginMode")]
    pub margin_mode: MarginMode,

    /// Margin coin (capitalized)
    #[serde(rename = "marginCoin")]
    pub margin_coin: MarginCoin,

    /// Amount (base coin)
    pub size: String,

    /// Price of the order (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Trade side (buy/sell)
    pub side: OrderSide,

    /// Trade type (open/close) - only required in hedge-mode
    #[serde(rename = "tradeSide", skip_serializing_if = "Option::is_none")]
    pub trade_side: Option<TradeSide>,

    /// Order type (limit/market)
    #[serde(rename = "orderType")]
    pub order_type: OrderType,

    /// Order expiration date (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<TimeInForce>,

    /// Custom order ID
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,

    /// Whether to reduce only (applicable only in one-way-position mode)
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<ReduceOnly>,
}

/// Response for placing a futures order
#[derive(Debug, Clone, Deserialize)]
pub struct PlaceOrderResponse {
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Client order ID
    #[serde(rename = "clientOid")]
    pub client_oid: String,
}

impl PlaceOrderRequest {
    /// Create a new limit order request
    pub fn limit(
        symbol: impl Into<String>,
        product_type: ProductType,
        margin_mode: MarginMode,
        margin_coin: MarginCoin,
        size: impl Into<String>,
        price: impl Into<String>,
        side: OrderSide,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            product_type,
            margin_mode,
            margin_coin,
            size: size.into(),
            price: Some(price.into()),
            side,
            trade_side: None,
            order_type: OrderType::Limit,
            force: Some(TimeInForce::GTC),
            client_oid: None,
            reduce_only: None,
        }
    }

    /// Create a new market order request
    pub fn market(
        symbol: impl Into<String>,
        product_type: ProductType,
        margin_mode: MarginMode,
        margin_coin: MarginCoin,
        size: impl Into<String>,
        side: OrderSide,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            product_type,
            margin_mode,
            margin_coin,
            size: size.into(),
            price: None,
            side,
            trade_side: None,
            order_type: OrderType::Market,
            force: None,
            client_oid: None,
            reduce_only: None,
        }
    }

    /// Set the trade side (open/close) - only required in hedge-mode
    pub fn trade_side(mut self, trade_side: TradeSide) -> Self {
        self.trade_side = Some(trade_side);
        self
    }

    /// Set the time in force for limit orders
    pub fn force(mut self, force: TimeInForce) -> Self {
        self.force = Some(force);
        self
    }

    /// Set a custom client order ID
    pub fn client_oid(mut self, client_oid: impl Into<String>) -> Self {
        self.client_oid = Some(client_oid.into());
        self
    }

    /// Set reduce only mode (applicable only in one-way-position mode)
    pub fn reduce_only(mut self, reduce_only: ReduceOnly) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }
}

impl BitgetRequest for PlaceOrderRequest {
    type Response = PlaceOrderResponse;

    fn path(&self) -> String {
        "/api/v2/mix/order/place-order".to_string()
    }

    fn method(&self) -> String {
        "POST".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_order_request() {
        let request = PlaceOrderRequest::limit(
            "ETHUSDT",
            ProductType::UsdtFutures,
            MarginMode::Isolated,
            MarginCoin::Usdt,
            "0.1",
            "2000",
            OrderSide::Sell,
        );

        assert_eq!(request.symbol, "ETHUSDT");
        assert_eq!(request.product_type, ProductType::UsdtFutures);
        assert_eq!(request.margin_mode, MarginMode::Isolated);
        assert_eq!(request.margin_coin, MarginCoin::Usdt);
        assert_eq!(request.size, "0.1");
        assert_eq!(request.price, Some("2000".to_string()));
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.order_type, OrderType::Limit);
        assert_eq!(request.force, Some(TimeInForce::GTC));
    }

    #[test]
    fn test_market_order_request() {
        let request = PlaceOrderRequest::market(
            "BTCUSDT",
            ProductType::UsdtFutures,
            MarginMode::Crossed,
            MarginCoin::Usdt,
            "0.01",
            OrderSide::Buy,
        )
        .trade_side(TradeSide::Open);

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_type, OrderType::Market);
        assert_eq!(request.price, None);
        assert_eq!(request.trade_side, Some(TradeSide::Open));
        assert_eq!(request.force, None);
    }
}
