use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::delivery::RestResult;

const DELIVERY_ACCOUNT_BOOK_ENDPOINT: &str = "/delivery/{}/account_book";

/// Request parameters for delivery account book
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryAccountBookRequest {
    /// Settlement currency
    pub settle: String,

    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Start time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Account book type filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Delivery account book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryAccountBookEntry {
    /// Entry ID
    pub id: i64,

    /// Change time
    pub time: f64,

    /// Currency
    pub currency: String,

    /// Change amount
    pub change: String,

    /// Balance after change
    pub balance: String,

    /// Change type
    #[serde(rename = "type")]
    pub entry_type: String,

    /// Change text
    pub text: String,
}

impl RestClient {
    /// Query delivery account book
    ///
    /// Retrieves detailed account transaction history for delivery trading.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The account book request parameters
    ///
    /// # Returns
    /// List of account book entries
    pub async fn get_delivery_account_book(
        &self,
        params: DeliveryAccountBookRequest,
    ) -> RestResult<Vec<DeliveryAccountBookEntry>> {
        let endpoint = DELIVERY_ACCOUNT_BOOK_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_account_book_endpoint() {
        assert_eq!(DELIVERY_ACCOUNT_BOOK_ENDPOINT, "/delivery/{}/account_book");
    }

    #[test]
    fn test_delivery_account_book_request_minimal() {
        let request = DeliveryAccountBookRequest {
            settle: "BTC".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "BTC");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("offset"));
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
        assert!(!obj.contains_key("type_"));
    }

    #[test]
    fn test_delivery_account_book_request_full() {
        let request = DeliveryAccountBookRequest {
            settle: "USDT".to_string(),
            limit: Some(50),
            offset: Some(10),
            from: Some(1640995200),
            to: Some(1640995800),
            type_: Some("trade".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["limit"], 50);
        assert_eq!(json["offset"], 10);
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["type_"], "trade");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 6);
    }

    #[test]
    fn test_delivery_account_book_request_with_time_range() {
        let request = DeliveryAccountBookRequest {
            settle: "BTC".to_string(),
            from: Some(1640995200),
            to: Some(1640995800),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 3);
    }

    #[test]
    fn test_delivery_account_book_request_with_pagination() {
        let request = DeliveryAccountBookRequest {
            settle: "USDT".to_string(),
            limit: Some(100),
            offset: Some(25),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["limit"], 100);
        assert_eq!(json["offset"], 25);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 3);
    }

    #[test]
    fn test_delivery_account_book_entry_deserialization() {
        let json = r#"{
            "id": 12345678,
            "time": 1640995200.123,
            "currency": "BTC",
            "change": "0.001",
            "balance": "1.501",
            "type": "trade",
            "text": "Buy BTC delivery contract"
        }"#;

        let entry: DeliveryAccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id, 12345678);
        assert_eq!(entry.time, 1640995200.123);
        assert_eq!(entry.currency, "BTC");
        assert_eq!(entry.change, "0.001");
        assert_eq!(entry.balance, "1.501");
        assert_eq!(entry.entry_type, "trade");
        assert_eq!(entry.text, "Buy BTC delivery contract");
    }

    #[test]
    fn test_delivery_account_book_entry_round_trip() {
        let original = DeliveryAccountBookEntry {
            id: 87654321,
            time: 1640995300.456,
            currency: "USDT".to_string(),
            change: "-50.25".to_string(),
            balance: "1450.75".to_string(),
            entry_type: "settle".to_string(),
            text: "Contract settlement".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: DeliveryAccountBookEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.time, original.time);
        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.change, original.change);
        assert_eq!(deserialized.balance, original.balance);
        assert_eq!(deserialized.entry_type, original.entry_type);
        assert_eq!(deserialized.text, original.text);
    }

    #[test]
    fn test_delivery_account_book_different_settlement_currencies() {
        let btc_request = DeliveryAccountBookRequest {
            settle: "BTC".to_string(),
            ..Default::default()
        };
        let usdt_request = DeliveryAccountBookRequest {
            settle: "USDT".to_string(),
            ..Default::default()
        };

        let btc_json = serde_json::to_value(&btc_request).unwrap();
        let usdt_json = serde_json::to_value(&usdt_request).unwrap();

        assert_eq!(btc_json["settle"], "BTC");
        assert_eq!(usdt_json["settle"], "USDT");
    }

    #[test]
    fn test_delivery_account_book_limit_validation() {
        let min_limit = DeliveryAccountBookRequest {
            settle: "BTC".to_string(),
            limit: Some(1),
            ..Default::default()
        };
        let max_limit = DeliveryAccountBookRequest {
            settle: "BTC".to_string(),
            limit: Some(1000),
            ..Default::default()
        };

        let min_json = serde_json::to_value(&min_limit).unwrap();
        let max_json = serde_json::to_value(&max_limit).unwrap();

        assert_eq!(min_json["limit"], 1);
        assert_eq!(max_json["limit"], 1000);
    }
}
