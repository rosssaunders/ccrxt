use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for delivery order book
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryOrderBookRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Order book level (1-100, default 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Order book depth limit (1-100, default 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Request UTC timestamp in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_id: Option<bool>,
}

/// Order book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryOrderBookEntry {
    /// Price
    pub p: String,
    /// Size
    pub s: i64,
}

/// Delivery order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryOrderBook {
    /// Order book ID
    pub id: i64,

    /// Current timestamp
    pub current: f64,

    /// Last update timestamp  
    pub update: f64,

    /// Asks (selling orders)
    pub asks: Vec<DeliveryOrderBookEntry>,

    /// Bids (buying orders)
    pub bids: Vec<DeliveryOrderBookEntry>,
}

impl RestClient {
    /// Get delivery order book
    ///
    /// Retrieves the order book for a specific delivery contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#futures-order-book-2>
    /// Bids are sorted by price high to low, asks are sorted by price low to high.
    pub async fn get_delivery_order_book(
        &self,
        params: DeliveryOrderBookRequest,
    ) -> crate::gateio::spotandmargin::Result<DeliveryOrderBook> {
        let endpoint = format!("/delivery/{}/order_book", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}
