use serde::{Deserialize, Serialize};

use super::RestClient;

const OPTIONS_ACCOUNT_BOOK_ENDPOINT: &str = "/options/account_book";

/// Options account book entry
#[derive(Debug, Clone, Deserialize)]
pub struct OptionsAccountBookEntry {
    /// Entry ID
    pub id: String,

    /// Time of the entry
    pub time: f64,

    /// Change amount
    pub change: String,

    /// Balance after the change
    pub balance: String,

    /// Entry type
    #[serde(rename = "type")]
    pub entry_type: String,

    /// Text description
    pub text: String,
}

/// Request to retrieve options account book
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsAccountBookRequest {
    /// Maximum number of record items to be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// You can set this to the last result ID to retrieve the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Start timestamp (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End timestamp (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Filter by entry type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub entry_type: Option<String>,
}

impl RestClient {
    /// Get options account book
    ///
    /// This endpoint returns the options account book with balance change records.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The account book request parameters
    ///
    /// # Returns
    /// List of account book entries
    pub async fn get_options_account_book(
        &self,
        request: OptionsAccountBookRequest,
    ) -> crate::gateio::options::RestResult<Vec<OptionsAccountBookEntry>> {
        self.get_with_query(OPTIONS_ACCOUNT_BOOK_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_account_book_request_serialization_default() {
        let request = OptionsAccountBookRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_options_account_book_request_serialization_with_limit() {
        let request = OptionsAccountBookRequest {
            limit: Some(100),
            offset: None,
            from: None,
            to: None,
            entry_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "limit=100");
    }

    #[test]
    fn test_options_account_book_request_serialization_with_pagination() {
        let request = OptionsAccountBookRequest {
            limit: Some(50),
            offset: Some(100),
            from: None,
            to: None,
            entry_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("offset=100"));
    }

    #[test]
    fn test_options_account_book_request_serialization_with_time_range() {
        let request = OptionsAccountBookRequest {
            limit: None,
            offset: None,
            from: Some(1640995200),
            to: Some(1641081600),
            entry_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_options_account_book_request_serialization_with_entry_type() {
        let request = OptionsAccountBookRequest {
            limit: None,
            offset: None,
            from: None,
            to: None,
            entry_type: Some("trade".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "type=trade");
    }

    #[test]
    fn test_options_account_book_request_serialization_all_fields() {
        let request = OptionsAccountBookRequest {
            limit: Some(25),
            offset: Some(50),
            from: Some(1640995200),
            to: Some(1641081600),
            entry_type: Some("fee".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=25"));
        assert!(serialized.contains("offset=50"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
        assert!(serialized.contains("type=fee"));
    }

    #[test]
    fn test_options_account_book_request_serialization_negative_values() {
        let request = OptionsAccountBookRequest {
            limit: Some(-1),
            offset: Some(-10),
            from: Some(-1640995200),
            to: Some(-1641081600),
            entry_type: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=-1"));
        assert!(serialized.contains("offset=-10"));
        assert!(serialized.contains("from=-1640995200"));
        assert!(serialized.contains("to=-1641081600"));
    }

    #[test]
    fn test_options_account_book_entry_deserialization() {
        let json = r#"{
            "id": "12345",
            "time": 1640995200.123,
            "change": "100.5",
            "balance": "1500.75",
            "type": "trade",
            "text": "BTC-20211231-50000-C buy 1"
        }"#;

        let entry: OptionsAccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id, "12345");
        assert_eq!(entry.time, 1640995200.123);
        assert_eq!(entry.change, "100.5");
        assert_eq!(entry.balance, "1500.75");
        assert_eq!(entry.entry_type, "trade");
        assert_eq!(entry.text, "BTC-20211231-50000-C buy 1");
    }

    #[test]
    fn test_options_account_book_entry_deserialization_negative_change() {
        let json = r#"{
            "id": "67890",
            "time": 1640995300.456,
            "change": "-50.25",
            "balance": "1450.50",
            "type": "fee",
            "text": "Trading fee for BTC-20211231-45000-P"
        }"#;

        let entry: OptionsAccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id, "67890");
        assert_eq!(entry.time, 1640995300.456);
        assert_eq!(entry.change, "-50.25");
        assert_eq!(entry.balance, "1450.50");
        assert_eq!(entry.entry_type, "fee");
        assert_eq!(entry.text, "Trading fee for BTC-20211231-45000-P");
    }

    #[test]
    fn test_options_account_book_entry_deserialization_zero_change() {
        let json = r#"{
            "id": "11111",
            "time": 1640995400.0,
            "change": "0",
            "balance": "1450.50",
            "type": "settlement",
            "text": "Option settlement"
        }"#;

        let entry: OptionsAccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id, "11111");
        assert_eq!(entry.time, 1640995400.0);
        assert_eq!(entry.change, "0");
        assert_eq!(entry.balance, "1450.50");
        assert_eq!(entry.entry_type, "settlement");
        assert_eq!(entry.text, "Option settlement");
    }

    #[test]
    fn test_options_account_book_entry_deserialization_large_values() {
        let json = r#"{
            "id": "999999999",
            "time": 1640995500.999999,
            "change": "10000.123456789",
            "balance": "50000.987654321",
            "type": "deposit",
            "text": "Large deposit transaction"
        }"#;

        let entry: OptionsAccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id, "999999999");
        assert_eq!(entry.time, 1640995500.999999);
        assert_eq!(entry.change, "10000.123456789");
        assert_eq!(entry.balance, "50000.987654321");
        assert_eq!(entry.entry_type, "deposit");
        assert_eq!(entry.text, "Large deposit transaction");
    }

    #[test]
    fn test_options_account_book_entry_array_deserialization() {
        let json = r#"[
            {
                "id": "1",
                "time": 1640995200.0,
                "change": "100.0",
                "balance": "1000.0",
                "type": "trade",
                "text": "Buy option"
            },
            {
                "id": "2",
                "time": 1640995300.0,
                "change": "-10.0",
                "balance": "990.0",
                "type": "fee",
                "text": "Trading fee"
            }
        ]"#;

        let entries: Vec<OptionsAccountBookEntry> = serde_json::from_str(json).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id, "1");
        assert_eq!(entries[0].change, "100.0");
        assert_eq!(entries[1].id, "2");
        assert_eq!(entries[1].change, "-10.0");
    }

    #[test]
    fn test_options_account_book_entry_empty_array_deserialization() {
        let json = r#"[]"#;
        let entries: Vec<OptionsAccountBookEntry> = serde_json::from_str(json).unwrap();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_options_account_book_entry_different_types() {
        let types = vec![
            "trade",
            "fee",
            "deposit",
            "withdrawal",
            "settlement",
            "transfer",
        ];

        for entry_type in types {
            let json = format!(
                r#"{{
                "id": "test",
                "time": 1640995200.0,
                "change": "100.0",
                "balance": "1000.0",
                "type": "{}",
                "text": "Test entry"
            }}"#,
                entry_type
            );

            let entry: OptionsAccountBookEntry = serde_json::from_str(&json).unwrap();
            assert_eq!(entry.entry_type, entry_type);
        }
    }

    #[test]
    fn test_options_account_book_entry_special_characters() {
        let json = r#"{
            "id": "test-123_abc",
            "time": 1640995200.0,
            "change": "100.0",
            "balance": "1000.0",
            "type": "trade",
            "text": "Special chars: BTC-20211231-50000-C @$#%"
        }"#;

        let entry: OptionsAccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id, "test-123_abc");
        assert_eq!(entry.text, "Special chars: BTC-20211231-50000-C @$#%");
    }

    #[test]
    fn test_options_account_book_request_serialization_different_entry_types() {
        let types = vec!["trade", "fee", "deposit", "withdrawal", "settlement"];

        for entry_type in types {
            let request = OptionsAccountBookRequest {
                limit: None,
                offset: None,
                from: None,
                to: None,
                entry_type: Some(entry_type.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("type={}", entry_type));
        }
    }
}
