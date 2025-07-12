use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for listing price orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPriceOrdersRequest {
    /// Currency pair
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Status filter (open, cancelled, finished)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Request to create a price order
#[derive(Debug, Clone, Serialize)]
pub struct CreatePriceOrderRequest {
    /// Currency pair
    pub currency_pair: String,

    /// Order type (limit, market)
    #[serde(rename = "type")]
    pub order_type: String,

    /// Account mode (spot, margin, cross_margin)
    pub account: String,

    /// Order side (buy or sell)
    pub side: String,

    /// Order amount
    pub amount: String,

    /// Order price (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force (gtc, ioc, poc, fok)
    pub time_in_force: String,

    /// Client order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Trigger price for price orders
    pub trigger_price: String,

    /// Rule for trigger (<=, >=)
    pub rule: String,

    /// Trigger expiration time (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
}

/// Price order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrder {
    /// Order ID
    pub id: i64,

    /// User ID
    pub user: i64,

    /// Currency pair
    pub currency_pair: String,

    /// Order status
    pub status: String,

    /// Account mode
    pub account: String,

    /// Order side (buy or sell)
    pub side: String,

    /// Order amount
    pub amount: String,

    /// Order price
    pub price: String,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,

    /// Time in force
    pub time_in_force: String,

    /// Trigger price
    pub trigger_price: String,

    /// Trigger rule
    pub rule: String,

    /// Trigger expiration time
    pub expiration: i64,

    /// Order fired ID (when triggered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fired_order_id: Option<String>,

    /// Create time timestamp
    pub create_time: i64,

    /// Put time timestamp
    pub put_time: i64,

    /// Client order id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Reason for cancellation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl RestClient {
    /// List price orders
    ///
    /// This endpoint returns price orders (conditional orders) for the authenticated user.
    /// Price orders are triggered when the market price reaches the specified trigger price.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#retrieve-running-auto-order-list>
    pub async fn list_price_orders(
        &self,
        params: ListPriceOrdersRequest,
    ) -> crate::gateio::Result<Vec<PriceOrder>> {
        self.get_with_query("/spot/price_orders", &params).await
    }

    /// Get a specific price order
    ///
    /// This endpoint returns details for a specific price order by ID.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-a-single-order>
    pub async fn get_price_order(&self, order_id: &str) -> crate::gateio::Result<PriceOrder> {
        let endpoint = format!("/spot/price_orders/{}", order_id);
        self.get(&endpoint).await
    }

    /// Create a price order
    ///
    /// This endpoint creates a new price order (conditional order) that will be
    /// triggered when the market price reaches the specified trigger price.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#create-a-price-triggered-order>
    pub async fn create_price_order(
        &self,
        request: CreatePriceOrderRequest,
    ) -> crate::gateio::Result<PriceOrder> {
        self.post("/spot/price_orders", &request).await
    }

    /// Cancel all price orders
    ///
    /// This endpoint cancels all price orders for the specified currency pair.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#cancel-all-open-orders-under-specified-currency-pair>
    pub async fn cancel_all_price_orders(
        &self,
        currency_pair: &str,
        account: &str,
    ) -> crate::gateio::Result<Vec<PriceOrder>> {
        let endpoint = format!(
            "/spot/price_orders?currency_pair={}&account={}",
            currency_pair, account
        );
        self.delete(&endpoint).await
    }

    /// Cancel a specific price order
    ///
    /// This endpoint cancels a specific price order by ID.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#cancel-a-price-triggered-order>
    pub async fn cancel_price_order(&self, order_id: &str) -> crate::gateio::Result<PriceOrder> {
        let endpoint = format!("/spot/price_orders/{}", order_id);
        self.delete(&endpoint).await
    }
}
