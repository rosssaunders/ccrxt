use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures order book
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesOrderBookRequest {
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
pub struct OrderBookEntry {
    /// Price
    pub p: String,
    /// Size
    pub s: i64,
}

/// Futures order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesOrderBook {
    /// Order book ID
    pub id: Option<i64>,

    /// Current timestamp
    pub current: f64,

    /// Last update timestamp  
    pub update: f64,

    /// Asks (selling orders)
    pub asks: Vec<OrderBookEntry>,

    /// Bids (buying orders)
    pub bids: Vec<OrderBookEntry>,
}

impl RestClient {
    /// Get futures order book
    ///
    /// Retrieves the order book for a specific futures contract.
    /// Bids are sorted by price high to low, asks are sorted by price low to high.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#futures-order-book>
    pub async fn get_futures_order_book(
        &self,
        params: FuturesOrderBookRequest,
    ) -> crate::gateio::perpetual::Result<FuturesOrderBook> {
        let endpoint = format!("/futures/{}/order_book", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}
