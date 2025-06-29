use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{OrderSide, OrderType, ResponseHeaders, RestResponse, Result, TimeInForce};

use super::RestClient;

/// Request for placing a new order
#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrderRequest {
    /// Client order ID (optional, max 40 characters)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,
    /// Order side (buy/sell)
    pub side: OrderSide,
    /// Trading symbol (e.g., "BTC-USDT")
    pub symbol: String,
    /// Order type (limit/market)
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order remark (optional, max 20 characters)
    pub remark: Option<String>,
    /// Self-trade prevention (optional)
    pub stp: Option<String>,
    /// Trade type (optional, TRADE for spot trading)
    #[serde(rename = "tradeType")]
    pub trade_type: Option<String>,
    /// Price (required for limit orders)
    pub price: Option<String>,
    /// Order size (required)
    pub size: Option<String>,
    /// Funds (for market buy orders instead of size)
    pub funds: Option<String>,
    /// Time in force (optional)
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<TimeInForce>,
    /// Cancel after time (optional, for GTT orders)
    #[serde(rename = "cancelAfter")]
    pub cancel_after: Option<i64>,
    /// Post only flag (optional)
    #[serde(rename = "postOnly")]
    pub post_only: Option<bool>,
    /// Hidden order flag (optional)
    pub hidden: Option<bool>,
    /// Iceberg order flag (optional)
    pub iceberg: Option<bool>,
    /// Visible size for iceberg orders (optional)
    #[serde(rename = "visibleSize")]
    pub visible_size: Option<String>,
    /// Tags (optional)
    pub tags: Option<String>,
}

/// Request for cancelling an order
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// Order ID to cancel
    pub order_id: String,
}

/// Request for cancelling all orders
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllOrdersRequest {
    /// Symbol to cancel orders for (optional)
    pub symbol: Option<String>,
    /// Trade type (optional)
    #[serde(rename = "tradeType")]
    pub trade_type: Option<String>,
}

/// Order placement response
#[derive(Debug, Clone, Deserialize)]
pub struct PlaceOrderResponse {
    /// Order ID assigned by KuCoin
    #[serde(rename = "orderId")]
    pub order_id: String,
}

/// Order cancellation response
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    /// List of cancelled order IDs
    #[serde(rename = "cancelledOrderIds")]
    pub cancelled_order_ids: Vec<String>,
}

/// Cancel all orders response
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersResponse {
    /// List of cancelled order IDs
    #[serde(rename = "cancelledOrderIds")]
    pub cancelled_order_ids: Vec<String>,
}

impl RestClient {
    /// Place a new order
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, PlaceOrderRequest};
    /// use kucoin::{OrderSide, OrderType};
    /// use rest::secrets::SecretValue;
    /// use secrecy::SecretString;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let api_key = SecretValue::new(SecretString::new("your_api_key".to_string()));
    ///     let api_secret = SecretValue::new(SecretString::new("your_api_secret".to_string()));
    ///     let passphrase = SecretValue::new(SecretString::new("your_passphrase".to_string()));
    ///     
    ///     let client = RestClient::new_with_credentials(api_key, api_secret, passphrase);
    ///     
    ///     let request = PlaceOrderRequest {
    ///         client_order_id: Some("my-order-1".to_string()),
    ///         side: OrderSide::Buy,
    ///         symbol: "BTC-USDT".to_string(),
    ///         order_type: OrderType::Limit,
    ///         price: Some("50000.0".to_string()),
    ///         size: Some("0.001".to_string()),
    ///         ..Default::default()
    ///     };
    ///     
    ///     let (response, _headers) = client.place_order(request).await?;
    ///     println!("Order placed: {}", response.order_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn place_order(
        &self,
        request: PlaceOrderRequest,
    ) -> Result<(PlaceOrderResponse, ResponseHeaders)> {
        let body = serde_json::to_string(&request)?;

        let (response, headers): (RestResponse<PlaceOrderResponse>, ResponseHeaders) =
            self.post("/api/v1/hf/orders", &body).await?;

        Ok((response.data, headers))
    }

    /// Cancel an order by order ID
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, CancelOrderRequest};
    /// use rest::secrets::SecretValue;
    /// use secrecy::SecretString;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let api_key = SecretValue::new(SecretString::new("your_api_key".to_string()));
    ///     let api_secret = SecretValue::new(SecretString::new("your_api_secret".to_string()));
    ///     let passphrase = SecretValue::new(SecretString::new("your_passphrase".to_string()));
    ///     
    ///     let client = RestClient::new_with_credentials(api_key, api_secret, passphrase);
    ///     
    ///     let request = CancelOrderRequest {
    ///         order_id: "order_id_here".to_string(),
    ///     };
    ///     
    ///     let (response, _headers) = client.cancel_order(request).await?;
    ///     println!("Cancelled orders: {:?}", response.cancelled_order_ids);
    ///     Ok(())
    /// }
    /// ```
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<(CancelOrderResponse, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("orderId".to_string(), request.order_id);

        let (response, headers): (RestResponse<CancelOrderResponse>, ResponseHeaders) =
            self.delete("/api/v1/hf/orders", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Cancel all orders (optionally filtered by symbol)
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, CancelAllOrdersRequest};
    /// use rest::secrets::SecretValue;
    /// use secrecy::SecretString;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let api_key = SecretValue::new(SecretString::new("your_api_key".to_string()));
    ///     let api_secret = SecretValue::new(SecretString::new("your_api_secret".to_string()));
    ///     let passphrase = SecretValue::new(SecretString::new("your_passphrase".to_string()));
    ///     
    ///     let client = RestClient::new_with_credentials(api_key, api_secret, passphrase);
    ///     
    ///     let request = CancelAllOrdersRequest {
    ///         symbol: Some("BTC-USDT".to_string()),
    ///         trade_type: Some("TRADE".to_string()),
    ///     };
    ///     
    ///     let (response, _headers) = client.cancel_all_orders(request).await?;
    ///     println!("Cancelled {} orders", response.cancelled_order_ids.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> Result<(CancelAllOrdersResponse, ResponseHeaders)> {
        let mut params = HashMap::new();
        
        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        
        if let Some(trade_type) = request.trade_type {
            params.insert("tradeType".to_string(), trade_type);
        }

        let params_option = if params.is_empty() { None } else { Some(params) };

        let (response, headers): (RestResponse<CancelAllOrdersResponse>, ResponseHeaders) =
            self.delete("/api/v1/hf/orders", params_option).await?;

        Ok((response.data, headers))
    }
}

impl Default for PlaceOrderRequest {
    fn default() -> Self {
        Self {
            client_order_id: None,
            side: OrderSide::Buy,
            symbol: String::new(),
            order_type: OrderType::Limit,
            remark: None,
            stp: None,
            trade_type: Some("TRADE".to_string()),
            price: None,
            size: None,
            funds: None,
            time_in_force: None,
            cancel_after: None,
            post_only: None,
            hidden: None,
            iceberg: None,
            visible_size: None,
            tags: None,
        }
    }
}
