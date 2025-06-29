//! Orders endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult, enums::*};

/// Request to create a new order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
    /// Command type for creating orders
    #[serde(rename = "commandType")]
    pub command_type: String,
    /// Client-generated order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Market symbol
    pub symbol: String,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order side
    pub side: OrderSide,
    /// Price for limit orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Stop price for stop orders
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,
    /// Order quantity
    pub quantity: String,
    /// Quote amount (for market orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quoteAmount")]
    pub quote_amount: Option<String>,
    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// Whether to allow borrowing for this order
    #[serde(rename = "allowBorrow")]
    pub allow_borrow: bool,
    /// Trading account ID
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,
}

impl Default for CreateOrderRequest {
    fn default() -> Self {
        Self {
            command_type: "V3CreateOrder".to_string(),
            client_order_id: String::new(),
            symbol: String::new(),
            order_type: OrderType::Limit,
            side: OrderSide::Buy,
            price: None,
            stop_price: None,
            quantity: String::new(),
            quote_amount: None,
            time_in_force: TimeInForce::Gtc,
            allow_borrow: false,
            trading_account_id: String::new(),
        }
    }
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

/// Order details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Unique order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Market symbol
    pub symbol: String,
    /// Order price
    pub price: String,
    /// Stop price
    #[serde(rename = "stopPrice")]
    pub stop_price: String,
    /// Average fill price
    #[serde(rename = "averageFillPrice")]
    pub average_fill_price: String,
    /// Whether borrowing is allowed
    #[serde(rename = "allowBorrow")]
    pub allow_borrow: bool,
    /// Order quantity
    pub quantity: String,
    /// Filled quantity
    #[serde(rename = "quantityFilled")]
    pub quantity_filled: String,
    /// Quote amount
    #[serde(rename = "quoteAmount")]
    pub quote_amount: String,
    /// Base fee
    #[serde(rename = "baseFee")]
    pub base_fee: String,
    /// Quote fee
    #[serde(rename = "quoteFee")]
    pub quote_fee: String,
    /// Whether this is a liquidation order
    #[serde(rename = "isLiquidation")]
    pub is_liquidation: bool,
    /// Order side
    pub side: OrderSide,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    /// Order status
    pub status: OrderStatus,
    /// Status reason
    #[serde(rename = "statusReason")]
    pub status_reason: String,
    /// Status reason code
    #[serde(rename = "statusReasonCode")]
    pub status_reason_code: String,
    /// Creation timestamp
    #[serde(rename = "createdAtTimestamp")]
    pub created_at_timestamp: u64,
    /// Creation datetime
    #[serde(rename = "createdAtDatetime")]
    pub created_at_datetime: String,
}

/// Parameters for querying orders
#[derive(Debug, Clone, Default)]
pub struct GetOrdersParams {
    /// Trading account ID (required)
    pub trading_account_id: String,
    /// Market symbol filter
    pub symbol: Option<String>,
    /// Client order ID filter
    pub client_order_id: Option<String>,
    /// Order side filter
    pub side: Option<OrderSide>,
    /// Order status filter
    pub status: Option<OrderStatus>,
}

impl RestClient {
    /// Get orders with optional filters
    ///
    /// Retrieve a list of orders placed by a trading account with specified filters.
    /// Only the last 24 hours of data is available for querying.
    ///
    /// # Arguments
    /// * `params` - Query parameters for filtering orders
    ///
    /// # Returns
    /// List of orders matching the filter criteria
    pub async fn get_orders(&mut self, params: GetOrdersParams) -> RestResult<Vec<Order>> {
        let mut query_params = vec![("tradingAccountId", params.trading_account_id)];

        if let Some(symbol) = params.symbol {
            query_params.push(("symbol", symbol));
        }
        if let Some(client_order_id) = params.client_order_id {
            query_params.push(("clientOrderId", client_order_id));
        }
        if let Some(side) = params.side {
            query_params.push(("side", format!("{:?}", side).to_uppercase()));
        }
        if let Some(status) = params.status {
            query_params.push(("status", format!("{:?}", status).to_uppercase()));
        }

        let mut url = "/v2/orders".to_string();
        if !query_params.is_empty() {
            url.push('?');
            let query_string: Vec<String> = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            url.push_str(&query_string.join("&"));
        }

        self.send_authenticated_request(
            &url,
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PrivateOrders,
        )
        .await
    }

    /// Create a new order
    ///
    /// Creates an order using the V3CreateOrder command type.
    /// Requires bearer token and HMAC signature in headers.
    ///
    /// # Arguments
    /// * `request` - Order creation request
    ///
    /// # Returns
    /// Order creation acknowledgment with order ID
    pub async fn create_order(
        &mut self,
        request: CreateOrderRequest,
    ) -> RestResult<CreateOrderResponse> {
        self.send_signed_request(
            "/v2/orders",
            reqwest::Method::POST,
            Some(&request),
            EndpointType::PrivateOrders,
        )
        .await
    }

    /// Get specific order by ID
    ///
    /// Retrieve details for a specific order by its order ID.
    ///
    /// # Arguments
    /// * `order_id` - The order ID to retrieve
    /// * `trading_account_id` - Trading account ID
    ///
    /// # Returns
    /// Order details
    pub async fn get_order(
        &mut self,
        order_id: &str,
        trading_account_id: &str,
    ) -> RestResult<Order> {
        let url = format!(
            "/v2/orders/{}?tradingAccountId={}",
            order_id, trading_account_id
        );

        self.send_authenticated_request(
            &url,
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PrivateOrders,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_order_request_serialization() {
        let request = CreateOrderRequest {
            command_type: "V3CreateOrder".to_string(),
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

    #[test]
    fn test_get_orders_params_default() {
        let params = GetOrdersParams::default();
        assert!(params.trading_account_id.is_empty());
        assert!(params.symbol.is_none());
        assert!(params.client_order_id.is_none());
        assert!(params.side.is_none());
        assert!(params.status.is_none());
    }
}
