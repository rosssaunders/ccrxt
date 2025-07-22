use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for getting account book
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetAccountBookRequest {
    /// Retrieve data of the specified currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Start timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Number of records per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Type of record
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub record_type: Option<String>,
}

/// Account book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBookEntry {
    /// Entry ID
    pub id: String,
    /// Unix timestamp
    pub time: i64,
    /// Currency
    pub currency: String,
    /// Change amount (positive for income, negative for expenditure)
    pub change: String,
    /// Balance after change
    pub balance: String,
    /// Entry type
    #[serde(rename = "type")]
    pub entry_type: String,
    /// Additional text
    pub text: Option<String>,
}

impl RestClient {
    /// Get account book
    ///
    /// This endpoint returns the account balance change history.
    /// You can filter by currency, time range, and record type.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#query-account-book>
    pub async fn get_account_book(
        &self,
        request: GetAccountBookRequest,
    ) -> crate::gateio::spot::Result<Vec<AccountBookEntry>> {
        self.get_with_query("/spot/account_book", &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_book_request_default() {
        let request = GetAccountBookRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty()); // No fields should be serialized when None
    }

    #[test]
    fn test_get_account_book_request_with_currency() {
        let request = GetAccountBookRequest {
            currency: Some("BTC".to_string()),
            from: None,
            to: None,
            page: None,
            limit: None,
            record_type: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
        assert!(!obj.contains_key("page"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("type"));
    }

    #[test]
    fn test_get_account_book_request_with_time_range() {
        let request = GetAccountBookRequest {
            currency: None,
            from: Some(1640995200),
            to: Some(1640995800),
            page: None,
            limit: None,
            record_type: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_get_account_book_request_with_pagination() {
        let request = GetAccountBookRequest {
            currency: None,
            from: None,
            to: None,
            page: Some(2),
            limit: Some(50),
            record_type: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["page"], 2);
        assert_eq!(json["limit"], 50);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_get_account_book_request_with_record_type() {
        let request = GetAccountBookRequest {
            currency: None,
            from: None,
            to: None,
            page: None,
            limit: None,
            record_type: Some("trade".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["type"], "trade");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_get_account_book_request_full() {
        let request = GetAccountBookRequest {
            currency: Some("USDT".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            page: Some(1),
            limit: Some(100),
            record_type: Some("deposit".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 100);
        assert_eq!(json["type"], "deposit");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 6);
    }

    #[test]
    fn test_get_account_book_request_different_record_types() {
        let types = vec![
            "trade", "deposit", "withdraw", "fee", "refund", 
            "bonus", "transfer", "liquidation", "margin", "interest"
        ];

        for record_type in types {
            let request = GetAccountBookRequest {
                currency: None,
                from: None,
                to: None,
                page: None,
                limit: None,
                record_type: Some(record_type.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["type"], record_type);
        }
    }

    #[test]
    fn test_get_account_book_request_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "USDC", "BNB", "SOL", "ADA", "DOT"];

        for currency in currencies {
            let request = GetAccountBookRequest {
                currency: Some(currency.to_string()),
                from: None,
                to: None,
                page: None,
                limit: None,
                record_type: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], currency);
        }
    }

    #[test]
    fn test_get_account_book_request_pagination_limits() {
        let test_cases = vec![(1, 10), (2, 25), (5, 50), (10, 100)];

        for (page, limit) in test_cases {
            let request = GetAccountBookRequest {
                currency: None,
                from: None,
                to: None,
                page: Some(page),
                limit: Some(limit),
                record_type: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["page"], page);
            assert_eq!(json["limit"], limit);
        }
    }

    #[test]
    fn test_account_book_entry_deserialization() {
        let json = r#"{
            "id": "12345678",
            "time": 1640995200,
            "currency": "BTC",
            "change": "0.001",
            "balance": "1.501",
            "type": "trade",
            "text": "Buy BTC with USDT"
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id, "12345678");
        assert_eq!(entry.time, 1640995200);
        assert_eq!(entry.currency, "BTC");
        assert_eq!(entry.change, "0.001");
        assert_eq!(entry.balance, "1.501");
        assert_eq!(entry.entry_type, "trade");
        assert_eq!(entry.text.as_ref().unwrap(), "Buy BTC with USDT");
    }

    #[test]
    fn test_account_book_entry_without_text() {
        let json = r#"{
            "id": "87654321",
            "time": 1640995300,
            "currency": "USDT",
            "change": "-30.5",
            "balance": "1969.5",
            "type": "trade",
            "text": null
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id, "87654321");
        assert_eq!(entry.time, 1640995300);
        assert_eq!(entry.currency, "USDT");
        assert_eq!(entry.change, "-30.5");
        assert_eq!(entry.balance, "1969.5");
        assert_eq!(entry.entry_type, "trade");
        assert!(entry.text.is_none());
    }

    #[test]
    fn test_account_book_entry_different_types() {
        let types = vec![
            ("trade", "0.1", "Trading profit"),
            ("deposit", "100.0", "Bank deposit"),
            ("withdraw", "-50.0", "Withdrawal to bank"),
            ("fee", "-0.05", "Trading fee"),
            ("refund", "5.0", "Fee refund"),
            ("bonus", "10.0", "Promotion bonus"),
            ("transfer", "25.0", "Internal transfer"),
            ("liquidation", "-2.5", "Position liquidated"),
            ("margin", "15.0", "Margin adjustment"),
            ("interest", "-0.5", "Margin interest"),
        ];

        for (entry_type, change, text) in types {
            let json = format!(r#"{{
                "id": "12345678",
                "time": 1640995200,
                "currency": "USDT",
                "change": "{}",
                "balance": "1000.0",
                "type": "{}",
                "text": "{}"
            }}"#, change, entry_type, text);

            let entry: AccountBookEntry = serde_json::from_str(&json).unwrap();
            assert_eq!(entry.entry_type, entry_type);
            assert_eq!(entry.change, change);
            assert_eq!(entry.text.as_ref().unwrap(), text);
        }
    }

    #[test]
    fn test_account_book_entry_positive_negative_changes() {
        let changes = vec![
            ("0.1", "Positive change"),
            ("-0.05", "Negative change"),
            ("1000.123456", "Large positive"),
            ("-999.987654", "Large negative"),
            ("0", "No change"),
            ("0.00000001", "Tiny positive"),
            ("-0.00000001", "Tiny negative"),
        ];

        for (change, description) in changes {
            let json = format!(r#"{{
                "id": "12345678",
                "time": 1640995200,
                "currency": "BTC",
                "change": "{}",
                "balance": "1.0",
                "type": "trade",
                "text": "{}"
            }}"#, change, description);

            let entry: AccountBookEntry = serde_json::from_str(&json).unwrap();
            assert_eq!(entry.change, change);
            assert_eq!(entry.text.as_ref().unwrap(), description);
        }
    }

    #[test]
    fn test_account_book_entry_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "USDC", "BNB", "SOL"];

        for currency in currencies {
            let json = format!(r#"{{
                "id": "12345678",
                "time": 1640995200,
                "currency": "{}",
                "change": "1.0",
                "balance": "10.0",
                "type": "trade",
                "text": "Test transaction"
            }}"#, currency);

            let entry: AccountBookEntry = serde_json::from_str(&json).unwrap();
            assert_eq!(entry.currency, currency);
        }
    }

    #[test]
    fn test_account_book_entry_realistic_trade_scenario() {
        let json = r#"{
            "id": "trade_001",
            "time": 1640995200,
            "currency": "BTC",
            "change": "0.00123456",
            "balance": "0.12345678",
            "type": "trade",
            "text": "Buy BTC at 30000 USDT"
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.entry_type, "trade");
        assert_eq!(entry.change, "0.00123456");
        assert_eq!(entry.balance, "0.12345678");
        assert_eq!(entry.text.as_ref().unwrap(), "Buy BTC at 30000 USDT");

        // Verify change is positive (buy)
        let change: f64 = entry.change.parse().unwrap();
        assert!(change > 0.0);
    }

    #[test]
    fn test_account_book_entry_realistic_deposit_scenario() {
        let json = r#"{
            "id": "deposit_001",
            "time": 1640995300,
            "currency": "USDT",
            "change": "1000.0",
            "balance": "5000.0",
            "type": "deposit",
            "text": "Bank wire transfer"
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.entry_type, "deposit");
        assert_eq!(entry.change, "1000.0");
        assert_eq!(entry.balance, "5000.0");
        assert_eq!(entry.text.as_ref().unwrap(), "Bank wire transfer");

        // Verify deposit increases balance
        let change: f64 = entry.change.parse().unwrap();
        let balance: f64 = entry.balance.parse().unwrap();
        assert!(change > 0.0);
        assert!(balance >= change);
    }

    #[test]
    fn test_account_book_entry_realistic_withdrawal_scenario() {
        let json = r#"{
            "id": "withdraw_001",
            "time": 1640995400,
            "currency": "ETH",
            "change": "-2.5",
            "balance": "7.5",
            "type": "withdraw",
            "text": "Withdrawal to external wallet"
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.entry_type, "withdraw");
        assert_eq!(entry.change, "-2.5");
        assert_eq!(entry.balance, "7.5");
        assert_eq!(entry.text.as_ref().unwrap(), "Withdrawal to external wallet");

        // Verify withdrawal decreases balance
        let change: f64 = entry.change.parse().unwrap();
        assert!(change < 0.0);
    }

    #[test]
    fn test_account_book_entry_realistic_fee_scenario() {
        let json = r#"{
            "id": "fee_001",
            "time": 1640995500,
            "currency": "BNB",
            "change": "-0.001",
            "balance": "99.999",
            "type": "fee",
            "text": "Trading fee for BTC/USDT order"
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.entry_type, "fee");
        assert_eq!(entry.change, "-0.001");
        assert_eq!(entry.balance, "99.999");
        assert_eq!(entry.text.as_ref().unwrap(), "Trading fee for BTC/USDT order");

        // Verify fee is negative
        let change: f64 = entry.change.parse().unwrap();
        assert!(change < 0.0);
    }

    #[test]
    fn test_account_book_entry_realistic_transfer_scenario() {
        let json = r#"{
            "id": "transfer_001",
            "time": 1640995600,
            "currency": "USDT",
            "change": "500.0",
            "balance": "1500.0",
            "type": "transfer",
            "text": "Transfer from margin to spot account"
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.entry_type, "transfer");
        assert_eq!(entry.change, "500.0");
        assert_eq!(entry.balance, "1500.0");
        assert_eq!(entry.text.as_ref().unwrap(), "Transfer from margin to spot account");
    }

    #[test]
    fn test_account_book_entry_realistic_bonus_scenario() {
        let json = r#"{
            "id": "bonus_001",
            "time": 1640995700,
            "currency": "GT",
            "change": "50.0",
            "balance": "150.0",
            "type": "bonus",
            "text": "New user registration bonus"
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.entry_type, "bonus");
        assert_eq!(entry.change, "50.0");
        assert_eq!(entry.balance, "150.0");
        assert_eq!(entry.text.as_ref().unwrap(), "New user registration bonus");

        // Verify bonus is positive
        let change: f64 = entry.change.parse().unwrap();
        assert!(change > 0.0);
    }

    #[test]
    fn test_account_book_entry_realistic_liquidation_scenario() {
        let json = r#"{
            "id": "liquidation_001",
            "time": 1640995800,
            "currency": "BTC",
            "change": "-0.05",
            "balance": "0.45",
            "type": "liquidation",
            "text": "Margin position liquidated due to insufficient margin"
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.entry_type, "liquidation");
        assert_eq!(entry.change, "-0.05");
        assert_eq!(entry.balance, "0.45");
        assert_eq!(entry.text.as_ref().unwrap(), "Margin position liquidated due to insufficient margin");

        // Verify liquidation decreases balance
        let change: f64 = entry.change.parse().unwrap();
        assert!(change < 0.0);
    }

    #[test]
    fn test_account_book_entry_realistic_interest_scenario() {
        let json = r#"{
            "id": "interest_001",
            "time": 1640995900,
            "currency": "USDT",
            "change": "-1.25",
            "balance": "998.75",
            "type": "interest",
            "text": "Daily margin interest charge"
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.entry_type, "interest");
        assert_eq!(entry.change, "-1.25");
        assert_eq!(entry.balance, "998.75");
        assert_eq!(entry.text.as_ref().unwrap(), "Daily margin interest charge");

        // Verify interest charge is negative
        let change: f64 = entry.change.parse().unwrap();
        assert!(change < 0.0);
    }

    #[test]
    fn test_get_account_book_request_realistic_all_transactions_scenario() {
        // Scenario: Get all account changes for the past week
        let request = GetAccountBookRequest {
            currency: None,
            from: Some(1640995200),
            to: Some(1641600000),
            page: Some(1),
            limit: Some(100),
            record_type: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1641600000);
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 100);

        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("currency"));
        assert!(!obj.contains_key("type"));
    }

    #[test]
    fn test_get_account_book_request_realistic_btc_trades_scenario() {
        // Scenario: Get BTC trading history only
        let request = GetAccountBookRequest {
            currency: Some("BTC".to_string()),
            from: None,
            to: None,
            page: Some(1),
            limit: Some(50),
            record_type: Some("trade".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["type"], "trade");
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_get_account_book_request_realistic_deposit_history_scenario() {
        // Scenario: Get deposit history for tax reporting
        let request = GetAccountBookRequest {
            currency: None,
            from: Some(1609459200), // Start of year
            to: Some(1640995200),   // End of year
            page: Some(1),
            limit: Some(100),
            record_type: Some("deposit".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["from"], 1609459200);
        assert_eq!(json["to"], 1640995200);
        assert_eq!(json["type"], "deposit");
        assert_eq!(json["limit"], 100);
    }

    #[test]
    fn test_get_account_book_request_realistic_fee_analysis_scenario() {
        // Scenario: Analyze trading fees for cost optimization
        let request = GetAccountBookRequest {
            currency: None,
            from: Some(1640908800), // Last 24 hours
            to: Some(1640995200),
            page: Some(1),
            limit: Some(50),
            record_type: Some("fee".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["from"], 1640908800);
        assert_eq!(json["to"], 1640995200);
        assert_eq!(json["type"], "fee");
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_account_book_entry_high_precision_amounts() {
        let json = r#"{
            "id": "precision_001",
            "time": 1640995200,
            "currency": "BTC",
            "change": "0.12345678",
            "balance": "1.87654321",
            "type": "trade",
            "text": "High precision transaction"
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.change, "0.12345678");
        assert_eq!(entry.balance, "1.87654321");
    }

    #[test]
    fn test_account_book_entry_large_amounts() {
        let json = r#"{
            "id": "large_001",
            "time": 1640995200,
            "currency": "USDT",
            "change": "1000000.123456",
            "balance": "5000000.987654",
            "type": "deposit",
            "text": "Large institutional deposit"
        }"#;

        let entry: AccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.change, "1000000.123456");
        assert_eq!(entry.balance, "5000000.987654");
    }

    #[test]
    fn test_get_account_book_request_clone() {
        let original = GetAccountBookRequest {
            currency: Some("BTC".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            page: Some(1),
            limit: Some(50),
            record_type: Some("trade".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.from, original.from);
        assert_eq!(cloned.to, original.to);
        assert_eq!(cloned.page, original.page);
        assert_eq!(cloned.limit, original.limit);
        assert_eq!(cloned.record_type, original.record_type);
    }

    #[test]
    fn test_account_book_entry_clone() {
        let original = AccountBookEntry {
            id: "12345678".to_string(),
            time: 1640995200,
            currency: "BTC".to_string(),
            change: "0.001".to_string(),
            balance: "1.501".to_string(),
            entry_type: "trade".to_string(),
            text: Some("Buy BTC with USDT".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.time, original.time);
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.change, original.change);
        assert_eq!(cloned.balance, original.balance);
        assert_eq!(cloned.entry_type, original.entry_type);
        assert_eq!(cloned.text, original.text);
    }

    #[test]
    fn test_get_account_book_request_debug() {
        let request = GetAccountBookRequest {
            currency: Some("BTC".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            page: Some(1),
            limit: Some(50),
            record_type: Some("trade".to_string()),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("GetAccountBookRequest"));
        assert!(debug_str.contains("BTC"));
        assert!(debug_str.contains("trade"));
    }

    #[test]
    fn test_account_book_entry_debug() {
        let entry = AccountBookEntry {
            id: "12345678".to_string(),
            time: 1640995200,
            currency: "BTC".to_string(),
            change: "0.001".to_string(),
            balance: "1.501".to_string(),
            entry_type: "trade".to_string(),
            text: Some("Buy BTC with USDT".to_string()),
        };

        let debug_str = format!("{:?}", entry);
        assert!(debug_str.contains("AccountBookEntry"));
        assert!(debug_str.contains("12345678"));
        assert!(debug_str.contains("BTC"));
    }

    #[test]
    fn test_get_account_book_request_serialization() {
        let request = GetAccountBookRequest {
            currency: Some("BTC".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            page: Some(1),
            limit: Some(50),
            record_type: Some("trade".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 50);
        assert_eq!(json["type"], "trade");
    }

    #[test]
    fn test_account_book_entry_serialization() {
        let entry = AccountBookEntry {
            id: "12345678".to_string(),
            time: 1640995200,
            currency: "BTC".to_string(),
            change: "0.001".to_string(),
            balance: "1.501".to_string(),
            entry_type: "trade".to_string(),
            text: Some("Buy BTC with USDT".to_string()),
        };

        let json = serde_json::to_value(&entry).unwrap();
        assert_eq!(json["id"], "12345678");
        assert_eq!(json["time"], 1640995200);
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["change"], "0.001");
        assert_eq!(json["balance"], "1.501");
        assert_eq!(json["type"], "trade");
        assert_eq!(json["text"], "Buy BTC with USDT");
    }

    #[test]
    fn test_get_account_book_request_optional_fields_behavior() {
        // Test with all fields
        let full_request = GetAccountBookRequest {
            currency: Some("BTC".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            page: Some(1),
            limit: Some(50),
            record_type: Some("trade".to_string()),
        };

        // Test with no fields
        let empty_request = GetAccountBookRequest {
            currency: None,
            from: None,
            to: None,
            page: None,
            limit: None,
            record_type: None,
        };

        let json_full = serde_json::to_value(&full_request).unwrap();
        let json_empty = serde_json::to_value(&empty_request).unwrap();

        // Full request should have all fields
        let obj_full = json_full.as_object().unwrap();
        assert_eq!(obj_full.len(), 6);
        assert!(obj_full.contains_key("currency"));
        assert!(obj_full.contains_key("from"));
        assert!(obj_full.contains_key("to"));
        assert!(obj_full.contains_key("page"));
        assert!(obj_full.contains_key("limit"));
        assert!(obj_full.contains_key("type"));

        // Empty request should have no fields
        let obj_empty = json_empty.as_object().unwrap();
        assert_eq!(obj_empty.len(), 0);
    }

    #[test]
    fn test_account_book_entry_round_trip() {
        let original = AccountBookEntry {
            id: "12345678".to_string(),
            time: 1640995200,
            currency: "BTC".to_string(),
            change: "0.001".to_string(),
            balance: "1.501".to_string(),
            entry_type: "trade".to_string(),
            text: Some("Buy BTC with USDT".to_string()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: AccountBookEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.time, original.time);
        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.change, original.change);
        assert_eq!(deserialized.balance, original.balance);
        assert_eq!(deserialized.entry_type, original.entry_type);
        assert_eq!(deserialized.text, original.text);
    }

    #[test]
    fn test_get_account_book_request_endpoint_validation() {
        let request = GetAccountBookRequest {
            currency: Some("BTC".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            page: Some(1),
            limit: Some(50),
            record_type: Some("trade".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        let _obj = json.as_object().unwrap();

        // Verify field types
        assert!(json["currency"].is_string());
        assert!(json["from"].is_number());
        assert!(json["to"].is_number());
        assert!(json["page"].is_number());
        assert!(json["limit"].is_number());
        assert!(json["type"].is_string());
    }

    #[test]
    fn test_account_book_entry_balance_consistency() {
        // Test that balance changes are mathematically consistent
        let entries = vec![
            ("100.0", "1000.0"),   // Initial
            ("50.0", "1050.0"),    // Deposit
            ("-25.0", "1025.0"),   // Trade
            ("-1.0", "1024.0"),    // Fee
            ("500.0", "1524.0"),   // Large deposit
        ];

        let mut previous_balance = 1000.0;
        for (change, expected_balance) in entries {
            let json = format!(r#"{{
                "id": "test",
                "time": 1640995200,
                "currency": "USDT",
                "change": "{}",
                "balance": "{}",
                "type": "trade",
                "text": "Test"
            }}"#, change, expected_balance);

            let entry: AccountBookEntry = serde_json::from_str(&json).unwrap();
            let change_val: f64 = entry.change.parse().unwrap();
            let balance_val: f64 = entry.balance.parse().unwrap();
            
            if change != "100.0" { // Skip first entry
                let expected = previous_balance + change_val;
                assert!((balance_val - expected).abs() < 0.001, 
                    "Balance inconsistency: {} + {} should equal {}, got {}", 
                    previous_balance, change_val, expected, balance_val);
            }
            
            previous_balance = balance_val;
        }
    }
}
