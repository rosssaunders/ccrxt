use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const GET_SPREAD_BOOKS_ENDPOINT: &str = "/api/v5/sprd/books";

/// Request parameters for getting spread order book
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpreadBooksRequest {
    /// Spread ID, e.g. BTC-USDT_BTC-USDT-SWAP
    #[serde(rename = "sprdId")]
    pub sprd_id: String,

    /// Order book depth per side. Maximum value is 400. Default value is 5.
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
}

/// Response data for getting spread order book
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadBooksData {
    /// Order book on sell side
    /// Each entry is an array: ["price", "quantity", "order_count"]
    /// - "price" is the depth price
    /// - "quantity" is the quantity at the price (Unit: szCcy)
    /// - "order_count" is the number of orders at the price
    #[serde(rename = "asks")]
    pub asks: Vec<Vec<String>>,

    /// Order book on buy side
    /// Each entry is an array: ["price", "quantity", "order_count"]
    /// - "price" is the depth price
    /// - "quantity" is the quantity at the price (Unit: szCcy)
    /// - "order_count" is the number of orders at the price
    #[serde(rename = "bids")]
    pub bids: Vec<Vec<String>>,

    /// Order book generation time, Unix timestamp format in milliseconds
    #[serde(rename = "ts")]
    pub ts: String,
}

impl RestClient {
    /// Get spread order book
    /// Retrieve the order book of the spread
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-get-order-book-public)
    pub async fn get_spread_books(
        &self,
        request: GetSpreadBooksRequest,
    ) -> RestResult<SpreadBooksData> {
        self.send_get_request(
            GET_SPREAD_BOOKS_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_spread_books_request_with_depth() {
        let request = GetSpreadBooksRequest {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            sz: Some("10".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetSpreadBooksRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_spread_books_request_default_depth() {
        let request = GetSpreadBooksRequest {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            sz: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("sprdId"));
        assert!(!serialized.contains("sz"));
    }

    #[test]
    fn test_spread_books_data_deserialization() {
        let json_response = r#"{
            "asks": [
                ["50010", "2.5", "3"],
                ["50020", "1.2", "2"],
                ["50030", "0.8", "1"]
            ],
            "bids": [
                ["49990", "1.8", "2"],
                ["49980", "3.2", "4"],
                ["49970", "0.5", "1"]
            ],
            "ts": "1597026383085"
        }"#;

        let books: SpreadBooksData = serde_json::from_str(json_response).unwrap();
        assert_eq!(books.asks.len(), 3);
        assert_eq!(books.bids.len(), 3);
        assert_eq!(books.asks[0][0], "50010");
        assert_eq!(books.asks[0][1], "2.5");
        assert_eq!(books.asks[0][2], "3");
        assert_eq!(books.bids[0][0], "49990");
        assert_eq!(books.bids[0][1], "1.8");
        assert_eq!(books.bids[0][2], "2");
        assert_eq!(books.ts, "1597026383085");
    }

    #[test]
    fn test_spread_books_data_serialization() {
        let books = SpreadBooksData {
            asks: vec![
                vec!["50010".to_string(), "2.5".to_string(), "3".to_string()],
                vec!["50020".to_string(), "1.2".to_string(), "2".to_string()],
            ],
            bids: vec![
                vec!["49990".to_string(), "1.8".to_string(), "2".to_string()],
                vec!["49980".to_string(), "3.2".to_string(), "4".to_string()],
            ],
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_string(&books).unwrap();
        let deserialized: SpreadBooksData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(books, deserialized);
    }

    #[test]
    fn test_empty_order_book() {
        let json_response = r#"{
            "asks": [],
            "bids": [],
            "ts": "1597026383085"
        }"#;

        let books: SpreadBooksData = serde_json::from_str(json_response).unwrap();
        assert_eq!(books.asks.len(), 0);
        assert_eq!(books.bids.len(), 0);
        assert_eq!(books.ts, "1597026383085");
    }

    #[test]
    fn test_order_book_entry_format() {
        // Test that order book entries have exactly 3 elements: [price, quantity, order_count]
        let json_response = r#"{
            "asks": [["50010", "2.5", "3"]],
            "bids": [["49990", "1.8", "2"]],
            "ts": "1597026383085"
        }"#;

        let books: SpreadBooksData = serde_json::from_str(json_response).unwrap();
        assert_eq!(books.asks[0].len(), 3);
        assert_eq!(books.bids[0].len(), 3);

        // Price should be the first element
        let ask_price = &books.asks[0][0];
        let bid_price = &books.bids[0][0];
        assert_eq!(ask_price, "50010");
        assert_eq!(bid_price, "49990");

        // Quantity should be the second element
        let ask_qty = &books.asks[0][1];
        let bid_qty = &books.bids[0][1];
        assert_eq!(ask_qty, "2.5");
        assert_eq!(bid_qty, "1.8");

        // Order count should be the third element
        let ask_count = &books.asks[0][2];
        let bid_count = &books.bids[0][2];
        assert_eq!(ask_count, "3");
        assert_eq!(bid_count, "2");
    }

    #[test]
    fn test_depth_parameter_values() {
        let valid_depths = vec!["1", "5", "10", "20", "50", "100", "200", "400"];

        for depth in valid_depths {
            let request = GetSpreadBooksRequest {
                sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
                sz: Some(depth.to_string()),
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"sz\":\"{}\"", depth)));
        }
    }
}
