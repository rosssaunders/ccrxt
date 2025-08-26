use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

/// Request parameters for retrieving margin account transaction history.
///
/// Used to query detailed margin account ledger entries including trading activity,
/// loan operations, interest charges, and balance transfers with comprehensive
/// filtering and pagination support for financial record keeping.
#[derive(Debug, Clone, Serialize, Default)]
pub struct MarginAccountBookRequest {
    /// Currency pair filter for trading-specific transactions (e.g., "BTC_USDT", "ETH_USDT").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Currency filter for asset-specific balance changes (e.g., "BTC", "ETH", "USDT").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Start time filter as Unix timestamp in seconds for transaction history range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time filter as Unix timestamp in seconds for transaction history range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Page number for pagination through large transaction datasets (default: 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records per page (default: 10, maximum: 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Comprehensive margin account transaction record with balance change details.
///
/// Represents individual entries in the margin account ledger including trading
/// activity, loan operations, interest charges, and fund transfers with complete
/// transaction context and resulting balance information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginAccountBookEntry {
    /// Unique transaction record identifier assigned by the exchange system.
    pub id: String,

    /// Transaction timestamp as Unix timestamp in seconds.
    pub time: i64,

    /// Currency code affected by this transaction (e.g., "BTC", "ETH", "USDT").
    pub currency: String,

    /// Balance change amount as string to preserve precision (positive for credits, negative for debits).
    pub change: String,

    /// Account balance after this transaction as string to preserve precision.
    pub balance: String,

    /// Transaction type classification (e.g., "trade", "loan", "repay", "interest", "transfer").
    #[serde(rename = "type")]
    pub change_type: String,

    /// Account type where the transaction occurred (typically "margin").
    pub account: String,

    /// Additional transaction details as JSON object (optional, varies by transaction type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
}

impl RestClient {
    /// Retrieve margin account transaction history
    ///
    /// Retrieves detailed margin account ledger entries showing all balance changes
    /// including trading activity, loan operations, repayments, interest charges,
    /// and fund transfers. Essential for margin trading reconciliation and audit trails.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#margin-account-book)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - Account book request parameters including optional currency filters, time range, and pagination
    ///
    /// # Returns
    /// Vector of margin account book entries containing complete transaction history with balance details
    pub async fn spot_get_margin_account_book(
        &self,
        params: MarginAccountBookRequest,
    ) -> RestResult<Vec<MarginAccountBookEntry>> {
        self.get_with_query("/margin/account_book", &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic margin account book request serialization with default parameters.
    #[test]
    fn test_margin_account_book_request_default() {
        let request = MarginAccountBookRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty()); // No fields should be serialized when None
    }

    #[test]
    fn test_margin_account_book_request_full() {
        let request = MarginAccountBookRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            currency: Some("BTC".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            page: Some(1),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_margin_account_book_request_partial() {
        let request = MarginAccountBookRequest {
            currency_pair: Some("ETH_USDT".to_string()),
            currency: None,
            from: Some(1640995200),
            to: None,
            page: None,
            limit: Some(100),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "ETH_USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["limit"], 100);

        // Fields that are None should be omitted
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("currency"));
        assert!(!obj.contains_key("to"));
        assert!(!obj.contains_key("page"));
    }

    #[test]
    fn test_margin_account_book_entry_deserialization() {
        let json = r#"{
            "id": "123456789",
            "time": 1640995200,
            "currency": "BTC",
            "change": "-0.0001",
            "balance": "0.9999",
            "type": "interest",
            "account": "margin",
            "detail": null
        }"#;

        let entry: MarginAccountBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.id, "123456789");
        assert_eq!(entry.time, 1640995200);
        assert_eq!(entry.currency, "BTC");
        assert_eq!(entry.change, "-0.0001");
        assert_eq!(entry.balance, "0.9999");
        assert_eq!(entry.change_type, "interest");
        assert_eq!(entry.account, "margin");
        assert!(entry.detail.is_none());
    }

    #[test]
    fn test_margin_account_book_entry_types() {
        let test_cases = vec![
            ("trade", "Trading activity"),
            ("loan", "Loan taken"),
            ("repay", "Loan repayment"),
            ("interest", "Interest charge"),
            ("transfer_in", "Transfer in"),
            ("transfer_out", "Transfer out"),
        ];

        for (change_type, _description) in test_cases {
            let json = format!(
                r#"{{
                    "id": "123",
                    "time": 1640995200,
                    "currency": "BTC",
                    "change": "0.1",
                    "balance": "1.0",
                    "type": "{}",
                    "account": "margin",
                    "detail": null
                }}"#,
                change_type
            );

            let entry: MarginAccountBookEntry = serde_json::from_str(&json).unwrap();
            assert_eq!(entry.change_type, change_type);
        }
    }

    #[test]
    fn test_margin_account_book_request_realistic_trade_history_scenario() {
        // Scenario: Get trade history for last week
        let from_time = 1640995200; // 1 week ago
        let to_time = 1641600000; // now

        let request = MarginAccountBookRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            currency: None,
            from: Some(from_time),
            to: Some(to_time),
            page: Some(1),
            limit: Some(100),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["from"], from_time);
        assert_eq!(json["to"], to_time);
        assert_eq!(json["limit"], 100);
    }

    #[test]
    fn test_margin_account_book_request_realistic_interest_tracking_scenario() {
        // Scenario: Track interest charges for specific currency
        let request = MarginAccountBookRequest {
            currency_pair: None,
            currency: Some("BTC".to_string()),
            from: Some(1640995200),
            to: None,
            page: Some(1),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_margin_account_book_request_clone() {
        let original = MarginAccountBookRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            currency: Some("BTC".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            page: Some(1),
            limit: Some(50),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.from, original.from);
        assert_eq!(cloned.to, original.to);
        assert_eq!(cloned.page, original.page);
        assert_eq!(cloned.limit, original.limit);
    }

    #[test]
    fn test_margin_account_book_entry_clone() {
        let original = MarginAccountBookEntry {
            id: "123456789".to_string(),
            time: 1640995200,
            currency: "BTC".to_string(),
            change: "-0.0001".to_string(),
            balance: "0.9999".to_string(),
            change_type: "interest".to_string(),
            account: "margin".to_string(),
            detail: None,
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.time, original.time);
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.change, original.change);
        assert_eq!(cloned.balance, original.balance);
        assert_eq!(cloned.change_type, original.change_type);
        assert_eq!(cloned.account, original.account);
        assert_eq!(cloned.detail, original.detail);
    }

    #[test]
    fn test_margin_account_book_request_pagination() {
        let pages = vec![1, 2, 5, 10];
        let limits = vec![10, 50, 100];

        for page in pages {
            for limit in limits.clone() {
                let request = MarginAccountBookRequest {
                    currency_pair: None,
                    currency: None,
                    from: None,
                    to: None,
                    page: Some(page),
                    limit: Some(limit),
                };

                let json = serde_json::to_value(&request).unwrap();
                assert_eq!(json["page"], page);
                assert_eq!(json["limit"], limit);
            }
        }
    }

    #[test]
    fn test_margin_account_book_request_time_ranges() {
        let test_cases = vec![
            (Some(1640995200), Some(1640995800), "6 hour range"),
            (Some(1640995200), None, "From timestamp only"),
            (None, Some(1640995800), "To timestamp only"),
            (None, None, "No time filter"),
        ];

        for (from, to, description) in test_cases {
            let request = MarginAccountBookRequest {
                currency_pair: None,
                currency: None,
                from,
                to,
                page: None,
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            let obj = json.as_object().unwrap();

            if from.is_some() {
                assert!(obj.contains_key("from"), "Failed for case: {}", description);
            } else {
                assert!(
                    !obj.contains_key("from"),
                    "Failed for case: {}",
                    description
                );
            }

            if to.is_some() {
                assert!(obj.contains_key("to"), "Failed for case: {}", description);
            } else {
                assert!(!obj.contains_key("to"), "Failed for case: {}", description);
            }
        }
    }
}
