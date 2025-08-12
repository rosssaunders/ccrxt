use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::coinbaseexchange::{RestResult, enums::OrderSide};

/// Endpoint URL path for getting product trades
const ENDPOINT_PATH: &str = "products/{}/trades";

/// Request to get product trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetProductTradesRequest {
    /// Limit on number of results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Used for pagination. Sets start cursor to before id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Used for pagination. Sets end cursor to after id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Trade information
#[derive(Debug, Clone, Deserialize)]
pub struct Trade {
    /// Trade ID
    pub trade_id: u64,

    /// Side of the trade (indicates maker order side)
    /// buy = down-tick (maker was buy order)
    /// sell = up-tick (maker was sell order)
    pub side: OrderSide,

    /// Size of the trade
    pub size: String,

    /// Price of the trade
    pub price: String,

    /// Time of the trade
    pub time: DateTime<Utc>,
}

/// Pagination information
#[derive(Debug, Clone)]
pub struct PaginationInfo {
    /// Before cursor for newer data
    pub before: Option<String>,
    /// After cursor for older data
    pub after: Option<String>,
}

/// Response from getting product trades
pub type GetProductTradesResponse = Vec<Trade>;

impl RestClient {
    /// Get product trades
    ///
    /// Gets a list of the latest trades for a product.
    /// The side of a trade indicates the maker order side. The maker order is the order that was open on the order book.
    /// A buy side indicates a down-tick because the maker was a buy order and their order was removed.
    /// A sell side indicates an up-tick.
    ///
    /// [API Documentation](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getproducttrades)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `product_id` - The product ID (e.g., "BTC-USD")
    /// * `request` - The product trades request parameters
    ///
    /// # Returns
    /// A result containing the product trades and pagination info or an error
    pub async fn get_product_trades(
        &self,
        product_id: &str,
        request: &GetProductTradesRequest,
    ) -> RestResult<(GetProductTradesResponse, Option<PaginationInfo>)> {
        let endpoint = ENDPOINT_PATH.replace("{}", product_id);
        let (data, headers) = self
            .send_get_request_with_headers(&endpoint, Some(request))
            .await?;

        // Extract pagination headers
        let before = headers.get("CB-BEFORE").cloned();
        let after = headers.get("CB-AFTER").cloned();

        let pagination = if before.is_some() || after.is_some() {
            Some(PaginationInfo { before, after })
        } else {
            None
        };

        Ok((data, pagination))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_product_trades_request_serialization() {
        let request = GetProductTradesRequest {
            limit: Some(50),
            before: Some("123".to_string()),
            after: Some("456".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("before=123"));
        assert!(serialized.contains("after=456"));
    }

    #[test]
    fn test_get_product_trades_request_default() {
        let request = GetProductTradesRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.is_empty());
    }

    #[test]
    fn test_trade_deserialization() {
        let json = r#"{
            "trade_id": 12345,
            "side": "buy",
            "size": "0.1",
            "price": "28999.50",
            "time": "2021-01-01T00:00:00.000Z"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.trade_id, 12345);
        assert_eq!(trade.side, OrderSide::Buy);
        assert_eq!(trade.size, "0.1");
        assert_eq!(trade.price, "28999.50");
    }

    #[test]
    fn test_get_product_trades_response_deserialization() {
        let json = r#"[
            {
                "trade_id": 12345,
                "side": "buy",
                "size": "0.1",
                "price": "28999.50",
                "time": "2021-01-01T00:00:00.000Z"
            },
            {
                "trade_id": 12346,
                "side": "sell",
                "size": "0.2",
                "price": "29000.00",
                "time": "2021-01-01T00:01:00.000Z"
            }
        ]"#;

        let trades: GetProductTradesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].trade_id, 12345);
        assert_eq!(trades[1].trade_id, 12346);
        assert_eq!(trades[0].side, OrderSide::Buy);
        assert_eq!(trades[1].side, OrderSide::Sell);
    }
}
