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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<f64>,

    /// Last update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<f64>,

    /// Ask orders (selling)
    pub asks: Vec<OptionsOrderBookEntry>,

    /// Bid orders (buying)
    pub bids: Vec<OptionsOrderBookEntry>,
}

impl RestClient {
    /// Options order book
    ///
    /// Retrieves order book for a specific options contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#options-order-book>
    pub async fn get_options_order_book(
        &self,
        params: OptionsOrderBookRequest,
    ) -> crate::gateio::options::RestResult<OptionsOrderBook> {
        self.get_with_query("/options/order_book", Some(&params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_order_book_request_minimal() {
        let request = OptionsOrderBookRequest {
            contract: "BTC_USDT-20240315-50000-C".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "BTC_USDT-20240315-50000-C");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert!(!obj.contains_key("interval"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("with_id"));
    }

    #[test]
    fn test_options_order_book_request_full() {
        let request = OptionsOrderBookRequest {
            contract: "ETH_USDT-20240315-3000-P".to_string(),
            interval: Some("0.1".to_string()),
            limit: Some(50),
            with_id: Some(true),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "ETH_USDT-20240315-3000-P");
        assert_eq!(json["interval"], "0.1");
        assert_eq!(json["limit"], 50);
        assert_eq!(json["with_id"], true);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 4);
    }

    #[test]
    fn test_options_order_book_request_with_interval() {
        let request = OptionsOrderBookRequest {
            contract: "BTC_USDT-20240315-50000-C".to_string(),
            interval: Some("0.01".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["interval"], "0.01");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_options_order_book_request_with_limit() {
        let request = OptionsOrderBookRequest {
            contract: "BTC_USDT-20240315-50000-C".to_string(),
            limit: Some(25),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["limit"], 25);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_options_order_book_entry_deserialization() {
        let json = r#"{
            "p": "1250.5",
            "s": 100
        }"#;

        let entry: OptionsOrderBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.p, "1250.5");
        assert_eq!(entry.s, 100);
    }

    #[test]
    fn test_options_order_book_deserialization() {
        let json = r#"{
            "id": 12345,
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [
                {"p": "1250.5", "s": 100},
                {"p": "1251.0", "s": 50}
            ],
            "bids": [
                {"p": "1249.5", "s": 75},
                {"p": "1249.0", "s": 25}
            ]
        }"#;

        let order_book: OptionsOrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.id, Some(12345));
        assert_eq!(order_book.current, Some(1640995200.123));
        assert_eq!(order_book.update, Some(1640995200.456));
        assert_eq!(order_book.asks.len(), 2);
        assert_eq!(order_book.bids.len(), 2);

        assert_eq!(order_book.asks[0].p, "1250.5");
        assert_eq!(order_book.asks[0].s, 100);
        assert_eq!(order_book.bids[0].p, "1249.5");
        assert_eq!(order_book.bids[0].s, 75);
    }

    #[test]
    fn test_options_order_book_minimal_deserialization() {
        let json = r#"{
            "asks": [],
            "bids": []
        }"#;

        let order_book: OptionsOrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.id, None);
        assert_eq!(order_book.current, None);
        assert_eq!(order_book.update, None);
        assert_eq!(order_book.asks.len(), 0);
        assert_eq!(order_book.bids.len(), 0);
    }

    #[test]
    fn test_options_order_book_round_trip() {
        let original = OptionsOrderBook {
            id: Some(54321),
            current: Some(1640995300.789),
            update: Some(1640995300.890),
            asks: vec![
                OptionsOrderBookEntry {
                    p: "1260.25".to_string(),
                    s: 200,
                },
                OptionsOrderBookEntry {
                    p: "1261.0".to_string(),
                    s: 150,
                },
            ],
            bids: vec![
                OptionsOrderBookEntry {
                    p: "1259.75".to_string(),
                    s: 180,
                },
                OptionsOrderBookEntry {
                    p: "1259.0".to_string(),
                    s: 120,
                },
            ],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: OptionsOrderBook = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.current, original.current);
        assert_eq!(deserialized.update, original.update);
        assert_eq!(deserialized.asks.len(), original.asks.len());
        assert_eq!(deserialized.bids.len(), original.bids.len());

        for (i, (orig, deser)) in original.asks.iter().zip(deserialized.asks.iter()).enumerate() {
            assert_eq!(deser.p, orig.p, "Ask {} price mismatch", i);
            assert_eq!(deser.s, orig.s, "Ask {} size mismatch", i);
        }

        for (i, (orig, deser)) in original.bids.iter().zip(deserialized.bids.iter()).enumerate() {
            assert_eq!(deser.p, orig.p, "Bid {} price mismatch", i);
            assert_eq!(deser.s, orig.s, "Bid {} size mismatch", i);
        }
    }

    #[test]
    fn test_options_order_book_different_contracts() {
        let call_request = OptionsOrderBookRequest {
            contract: "BTC_USDT-20240315-50000-C".to_string(),
            ..Default::default()
        };
        let put_request = OptionsOrderBookRequest {
            contract: "BTC_USDT-20240315-50000-P".to_string(),
            ..Default::default()
        };

        let call_json = serde_json::to_value(&call_request).unwrap();
        let put_json = serde_json::to_value(&put_request).unwrap();

        assert_eq!(call_json["contract"], "BTC_USDT-20240315-50000-C");
        assert_eq!(put_json["contract"], "BTC_USDT-20240315-50000-P");
    }

    #[test]
    fn test_options_order_book_limit_bounds() {
        let min_limit = OptionsOrderBookRequest {
            contract: "BTC_USDT-20240315-50000-C".to_string(),
            limit: Some(1),
            ..Default::default()
        };
        let max_limit = OptionsOrderBookRequest {
            contract: "BTC_USDT-20240315-50000-C".to_string(),
            limit: Some(100),
            ..Default::default()
        };

        let min_json = serde_json::to_value(&min_limit).unwrap();
        let max_json = serde_json::to_value(&max_limit).unwrap();

        assert_eq!(min_json["limit"], 1);
        assert_eq!(max_json["limit"], 100);
    }

    #[test]
    fn test_options_order_book_entry_price_formats() {
        let entries = vec![
            ("0.001", 1000),
            ("1250.5", 100),
            ("50000.0", 10),
            ("0.1", 5000),
        ];

        for (price, size) in entries {
            let entry = OptionsOrderBookEntry {
                p: price.to_string(),
                s: size,
            };

            let json = serde_json::to_string(&entry).unwrap();
            let deserialized: OptionsOrderBookEntry = serde_json::from_str(&json).unwrap();

            assert_eq!(deserialized.p, price);
            assert_eq!(deserialized.s, size);
            
            // Verify price can be parsed as a number
            assert!(price.parse::<f64>().is_ok());
        }
    }
}
