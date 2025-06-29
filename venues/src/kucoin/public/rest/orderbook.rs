use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

/// Request for getting partial order book
#[derive(Debug, Clone, Serialize)]
pub struct GetPartOrderBookRequest {
    /// Trading symbol (e.g., "BTC-USDT")
    pub symbol: String,
    /// Depth level (20 or 100)
    pub level: OrderBookLevel,
}

/// Request for getting full order book
#[derive(Debug, Clone, Serialize)]
pub struct GetFullOrderBookRequest {
    /// Trading symbol (e.g., "BTC-USDT")
    pub symbol: String,
}

/// Order book depth levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OrderBookLevel {
    #[serde(rename = "20")]
    Twenty,
    #[serde(rename = "100")]
    OneHundred,
}

/// Price level in order book
#[derive(Debug, Clone, Deserialize)]
pub struct PriceLevel {
    /// Price
    pub price: String,
    /// Size/Quantity
    pub size: String,
}

/// Partial order book response  
#[derive(Debug, Clone, Deserialize)]
pub struct PartOrderBookResponse {
    /// Server timestamp
    pub time: i64,
    /// Sequence number
    pub sequence: String,
    /// Bid orders (buy orders)
    pub bids: Vec<[String; 2]>, // [price, size]
    /// Ask orders (sell orders)
    pub asks: Vec<[String; 2]>, // [price, size]
}

/// Full order book response
#[derive(Debug, Clone, Deserialize)]
pub struct FullOrderBookResponse {
    /// Server timestamp
    pub time: i64,
    /// Sequence number
    pub sequence: String,
    /// Bid orders (buy orders)
    pub bids: Vec<[String; 2]>, // [price, size]
    /// Ask orders (sell orders)
    pub asks: Vec<[String; 2]>, // [price, size]
}

impl RestClient {
    /// Get partial order book (20 or 100 levels)
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::public::{RestClient, GetPartOrderBookRequest, OrderBookLevel};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_default();
    ///     let request = GetPartOrderBookRequest {
    ///         symbol: "BTC-USDT".to_string(),
    ///         level: OrderBookLevel::Twenty,
    ///     };
    ///     let (response, _headers) = client.get_part_order_book(request).await?;
    ///     println!("Order book time: {}", response.time);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_part_order_book(
        &self,
        request: GetPartOrderBookRequest,
    ) -> Result<(PartOrderBookResponse, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol);

        let level_str = match request.level {
            OrderBookLevel::Twenty => "20",
            OrderBookLevel::OneHundred => "100",
        };

        let endpoint = format!("/api/v1/market/orderbook/level2_{}", level_str);

        let (response, headers): (RestResponse<PartOrderBookResponse>, ResponseHeaders) =
            self.get(&endpoint, Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Get full order book (all levels)
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::public::{RestClient, GetFullOrderBookRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_default();
    ///     let request = GetFullOrderBookRequest {
    ///         symbol: "BTC-USDT".to_string(),
    ///     };
    ///     let (response, _headers) = client.get_full_order_book(request).await?;
    ///     println!("Order book sequence: {}", response.sequence);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_full_order_book(
        &self,
        request: GetFullOrderBookRequest,
    ) -> Result<(FullOrderBookResponse, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol);

        let (response, headers): (RestResponse<FullOrderBookResponse>, ResponseHeaders) =
            self.get("/api/v1/market/orderbook/level2", Some(params)).await?;

        Ok((response.data, headers))
    }
}
