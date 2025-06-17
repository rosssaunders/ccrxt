use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult};

/// Order type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    /// Market order
    Market,
    /// Limit order
    Limit,
    /// Take stop limit order
    TakeStopLimit,
    /// Take stop market order
    TakeStopMarket,
    /// Trigger limit order
    TriggerLimit,
    /// Trigger market order
    TriggerMarket,
}

/// Order side enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

/// Time in force enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    /// Post only
    PostOnly,
    /// Good till canceled
    Gtc,
    /// Immediate or cancel
    Ioc,
    /// Fill or kill
    Fok,
}

/// Order status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// New order
    New,
    /// Pending order
    Pending,
    /// Partially filled
    PartiallyFilled,
    /// Filled
    Filled,
    /// Canceled
    Canceled,
    /// Failed
    Failed,
}

/// Request to place a new order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    /// Trading pair, e.g., BTC-USDT
    pub symbol: String,

    /// Order side: BUY/SELL
    pub side: OrderSide,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order trigger price (for stop orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Original quantity, e.g., 0.1BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    /// Quote order quantity, e.g., 100USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<String>,

    /// Price, e.g., 10000USDT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Customized order ID for users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Time in force
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Request valid time window value, Unit: milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Response from placing a new order
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    /// Trading pair
    pub symbol: String,

    /// Order ID
    pub order_id: i64,

    /// Transaction timestamp
    pub transact_time: i64,

    /// Price
    pub price: String,

    /// Original quantity
    pub orig_qty: String,

    /// Executed quantity
    pub executed_qty: String,

    /// Cumulative quote asset transacted quantity
    pub cummulative_quote_qty: String,

    /// Order status
    pub status: OrderStatus,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side
    pub side: OrderSide,

    /// Customized order ID for users
    #[serde(rename = "clientOrderID")]
    pub client_order_id: Option<String>,
}

impl RestClient {
    /// Place a new order
    ///
    /// Places a new order on the spot market.
    /// Rate limit: 5/s by UID
    ///
    /// # Arguments
    /// * `request` - The place order request containing order details
    ///
    /// # Returns
    /// A result containing the order response or an error
    ///
    /// # Notes
    /// - For limit orders, price is required
    /// - For limit orders, either quantity or quoteOrderQty is required
    /// - For buy-side market orders, quoteOrderQty is required
    /// - For sell-side market orders, quantity is required
    ///
    /// # Example
    /// ```rust,no_run
    /// use venues::bingx::{PrivateRestClient, PlaceOrderRequest, OrderType, OrderSide};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client: PrivateRestClient = unimplemented!();
    ///     let request = PlaceOrderRequest {
    ///         symbol: "BTC-USDT".to_string(),
    ///         side: OrderSide::Buy,
    ///         order_type: OrderType::Limit,
    ///         price: Some("50000.0".to_string()),
    ///         quantity: Some("0.001".to_string()),
    ///         stop_price: None,
    ///         quote_order_qty: None,
    ///         new_client_order_id: None,
    ///         time_in_force: None,
    ///         recv_window: None,
    ///     };
    ///     let order_response = client.place_order(&request).await?;
    ///     println!("Order placed: {:?}", order_response);
    ///     Ok(())
    /// }
    /// ```
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> RestResult<PlaceOrderResponse> {
        self.send_request(
            "/openApi/spot/v1/trade/order",
            reqwest::Method::POST,
            Some(request),
            EndpointType::Trading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_request_serialization() {
        let request = PlaceOrderRequest {
            symbol: "BTC-USDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            price: Some("50000.0".to_string()),
            quantity: Some("0.001".to_string()),
            stop_price: None,
            quote_order_qty: None,
            new_client_order_id: Some("my_order_123".to_string()),
            time_in_force: Some(TimeInForce::Gtc),
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("side=BUY"));
        assert!(serialized.contains("type=LIMIT"));
        assert!(serialized.contains("price=50000.0"));
        assert!(serialized.contains("quantity=0.001"));
        assert!(serialized.contains("newClientOrderId=my_order_123"));
        assert!(serialized.contains("timeInForce=GTC"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_place_order_response_deserialization() {
        let json = r#"{
            "symbol": "BTC-USDT",
            "orderId": 123456789,
            "transactTime": 1658748648396,
            "price": "50000.0",
            "origQty": "0.001",
            "executedQty": "0.0",
            "cummulativeQuoteQty": "0.0",
            "status": "NEW",
            "type": "LIMIT",
            "side": "BUY",
            "clientOrderID": "my_order_123"
        }"#;

        let response: PlaceOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-USDT");
        assert_eq!(response.order_id, 123456789);
        assert_eq!(response.transact_time, 1658748648396);
        assert_eq!(response.price, "50000.0");
        assert_eq!(response.orig_qty, "0.001");
        assert_eq!(response.executed_qty, "0.0");
        assert_eq!(response.cummulative_quote_qty, "0.0");
        assert!(matches!(response.status, OrderStatus::New));
        assert!(matches!(response.order_type, OrderType::Limit));
        assert!(matches!(response.side, OrderSide::Buy));
        assert_eq!(response.client_order_id, Some("my_order_123".to_string()));
    }

    #[test]
    fn test_order_type_serialization() {
        assert_eq!(serde_json::to_string(&OrderType::Market).unwrap(), "\"MARKET\"");
        assert_eq!(serde_json::to_string(&OrderType::Limit).unwrap(), "\"LIMIT\"");
        assert_eq!(serde_json::to_string(&OrderType::TakeStopLimit).unwrap(), "\"TAKE_STOP_LIMIT\"");
    }

    #[test]
    fn test_order_side_serialization() {
        assert_eq!(serde_json::to_string(&OrderSide::Buy).unwrap(), "\"BUY\"");
        assert_eq!(serde_json::to_string(&OrderSide::Sell).unwrap(), "\"SELL\"");
    }

    #[test]
    fn test_time_in_force_serialization() {
        assert_eq!(serde_json::to_string(&TimeInForce::Gtc).unwrap(), "\"GTC\"");
        assert_eq!(serde_json::to_string(&TimeInForce::Ioc).unwrap(), "\"IOC\"");
        assert_eq!(serde_json::to_string(&TimeInForce::Fok).unwrap(), "\"FOK\"");
        assert_eq!(serde_json::to_string(&TimeInForce::PostOnly).unwrap(), "\"POST_ONLY\"");
    }
}