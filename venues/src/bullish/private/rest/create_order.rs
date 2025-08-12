use serde::{Deserialize, Serialize};

use crate::bullish::enums::{OrderSide, OrderType, TimeInForce};
use crate::bullish::private::rest::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL for orders operations
const ORDERS_ENDPOINT: &str = "/v2/orders";

/// Command type for order creation (currently only "V3CreateOrder" is supported)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CommandType {
    #[serde(rename = "V3CreateOrder")]
    V3CreateOrder,
}

impl Default for CommandType {
    fn default() -> Self {
        CommandType::V3CreateOrder
    }
}

/// Request parameters for creating a new order.
///
/// Fields marked with * are required.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
    /// The command type, it must be 'V3CreateOrder'.
    #[serde(rename = "commandType")]
    pub command_type: CommandType,

    /// Unique numeric (i64) identifier generated on the client side expressed as a string value.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Market symbol. Eg BTCUSDC for SPOT and BTC-USDC-PERP for PERPETUAL market.
    pub symbol: String,

    /// Order type. Can be "LIMIT", "MARKET", "STOP_LIMIT", or "POST_ONLY".
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side. Can be "BUY" or "SELL".
    pub side: OrderSide,

    /// Price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Stop price.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,

    /// Quantity.
    pub quantity: String,

    /// Quote amount (for market orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quoteAmount")]
    pub quote_amount: Option<String>,

    /// Time in force. Can be "GTC", "FOK", "IOC".
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Allows to borrow on the order. Default: false.
    #[serde(rename = "allowBorrow")]
    pub allow_borrow: bool,

    /// Unique trading account ID.
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,
}

/// Response for create order request
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResponse {
    /// Acknowledgment message
    pub message: String,

    /// Request ID
    #[serde(rename = "requestId")]
    pub request_id: String,

    /// Generated order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
}

impl RestClient {
    /// Create a new order
    ///
    /// Creates an order using the V3CreateOrder command type.
    /// Supports market, limit, and stop orders with various time-in-force options.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#post-/v2/orders
    ///
    /// Rate limit: per private trading endpoints
    ///
    /// # Arguments
    /// * `request` - Order creation request parameters including symbol, type, side, and quantity
    ///
    /// # Returns
    /// Order creation acknowledgment with assigned order ID and status
    pub async fn create_order(
        &mut self,
        request: CreateOrderRequest,
    ) -> RestResult<CreateOrderResponse> {
        self.send_post_request(ORDERS_ENDPOINT, request, EndpointType::PrivateOrders)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_order_request_serialization() {
        let request = CreateOrderRequest {
            command_type: CommandType::V3CreateOrder,
            client_order_id: "1234".to_string(),
            symbol: "BTCUSDC".to_string(),
            order_type: OrderType::Limit,
            side: OrderSide::Buy,
            price: Some("31000.1".to_string()),
            stop_price: None,
            quantity: "1.1".to_string(),
            quote_amount: None,
            time_in_force: TimeInForce::Gtc,
            allow_borrow: true,
            trading_account_id: "111000000000001".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("V3CreateOrder"));
        assert!(json.contains("BTCUSDC"));
        assert!(json.contains("LMT"));
        assert!(json.contains("BUY"));
    }
}
