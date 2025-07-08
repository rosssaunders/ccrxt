use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for options order book
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsOrderBookRequest {
    /// Contract name
    pub contract: String,

    /// Order book interval (0, 0.1, 0.01, 0.001, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,

    /// Maximum number of records to return for asks and bids each (1-100, default 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// If true, response will include unique ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_id: Option<bool>,
}

/// Order book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsOrderBookEntry {
    /// Price (quote currency)
    pub p: String,

    /// Size
    pub s: i64,
}

/// Options order book information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsOrderBook {
    /// Unique ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Current timestamp
    pub current: i64,

    /// Last update timestamp
    pub update: i64,

    /// Ask orders (selling)
    pub asks: Vec<OptionsOrderBookEntry>,

    /// Bid orders (buying)
    pub bids: Vec<OptionsOrderBookEntry>,
}

impl RestClient {
    /// Options order book
    ///
    /// Retrieves order book for a specific options contract.
    pub async fn get_options_order_book(
        &self,
        params: OptionsOrderBookRequest,
    ) -> crate::gateio::Result<OptionsOrderBook> {
        self.get_with_query("/options/order_book", Some(&params))
            .await
    }
}
