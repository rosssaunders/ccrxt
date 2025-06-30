use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for retrieving order book
#[derive(Debug, Clone, Serialize, Default)]
pub struct OrderBookRequest {
    /// Currency pair to query order book for
    pub currency_pair: String,

    /// Maximum number of order book levels to return (default: 10, max: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Whether to include order IDs in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_id: Option<bool>,
}

/// Order book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookEntry {
    /// Price
    #[serde(rename = "0")]
    pub price: String,

    /// Amount
    #[serde(rename = "1")]
    pub amount: String,
}

/// Order book response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Current order book ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Last updated timestamp
    pub current: i64,

    /// Last update timestamp (in milliseconds)
    pub update: i64,

    /// Ask orders (price, amount)
    pub asks: Vec<Vec<String>>,

    /// Bid orders (price, amount)
    pub bids: Vec<Vec<String>>,
}

impl RestClient {
    /// Get order book for a currency pair
    ///
    /// This endpoint returns the current order book (bid/ask) for the specified currency pair.
    /// You can optionally include order IDs and limit the number of levels returned.
    pub async fn get_order_book(
        &self,
        params: OrderBookRequest,
    ) -> crate::gateio::Result<OrderBook> {
        self.get_with_query("/spot/order_book", Some(&params)).await
    }
}
