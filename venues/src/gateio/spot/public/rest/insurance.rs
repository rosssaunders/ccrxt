use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for insurance fund history
#[derive(Debug, Clone, Serialize, Default)]
pub struct InsuranceHistoryRequest {
    /// Currency to query (e.g., "BTC", "USDT")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Page number, starting from 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Insurance fund history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceRecord {
    /// Timestamp
    pub t: i64,

    /// Currency
    pub currency: String,

    /// Amount
    pub amount: String,

    /// Type of record (liquidation, fee, etc.)
    #[serde(rename = "type")]
    pub record_type: String,
}

impl RestClient {
    /// Get insurance fund history
    ///
    /// This endpoint returns the history of the insurance fund, showing
    /// liquidation and fee contributions over time.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#spot-insurance-balance-history>
    pub async fn get_insurance_history(
        &self,
        params: InsuranceHistoryRequest,
    ) -> crate::gateio::spot::Result<Vec<InsuranceRecord>> {
        self.get_with_query("/spot/insurance_history", Some(&params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insurance_history_request_minimal_serialization() {
        let request = InsuranceHistoryRequest {
            currency: None,
            page: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_insurance_history_request_with_currency() {
        let request = InsuranceHistoryRequest {
            currency: Some("BTC".to_string()),
            page: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency=BTC");
    }

    #[test]
    fn test_insurance_history_request_with_pagination() {
        let request = InsuranceHistoryRequest {
            currency: None,
            page: Some(2),
            limit: Some(50),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("page=2"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_insurance_history_request_full_parameters() {
        let request = InsuranceHistoryRequest {
            currency: Some("USDT".to_string()),
            page: Some(1),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency=USDT"));
        assert!(serialized.contains("page=1"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_insurance_history_request_different_currencies() {
        let currencies = vec!["BTC", "USDT", "ETH", "BNB", "SOL", "ADA"];
        
        for currency in currencies {
            let request = InsuranceHistoryRequest {
                currency: Some(currency.to_string()),
                page: None,
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency={}", currency));
        }
    }

    #[test]
    fn test_insurance_history_request_page_ranges() {
        let pages = vec![1, 10, 100, 1000];
        
        for page in pages {
            let request = InsuranceHistoryRequest {
                currency: None,
                page: Some(page),
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("page={}", page));
        }
    }

    #[test]
    fn test_insurance_history_request_limit_ranges() {
        let limits = vec![1, 10, 50, 100, 500, 1000];
        
        for limit in limits {
            let request = InsuranceHistoryRequest {
                currency: None,
                page: None,
                limit: Some(limit),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("limit={}", limit));
        }
    }

    #[test]
    fn test_insurance_history_request_negative_values() {
        let request = InsuranceHistoryRequest {
            currency: None,
            page: Some(-1),
            limit: Some(-100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("page=-1"));
        assert!(serialized.contains("limit=-100"));
    }

    #[test]
    fn test_insurance_history_request_max_values() {
        let request = InsuranceHistoryRequest {
            currency: None,
            page: Some(i32::MAX),
            limit: Some(i32::MAX),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains(&format!("page={}", i32::MAX)));
        assert!(serialized.contains(&format!("limit={}", i32::MAX)));
    }

    #[test]
    fn test_insurance_history_request_default() {
        let request = InsuranceHistoryRequest::default();
        assert_eq!(request.currency, None);
        assert_eq!(request.page, None);
        assert_eq!(request.limit, None);
    }

    #[test]
    fn test_insurance_record_deserialization() {
        let json = r#"{
            "t": 1640995200,
            "currency": "BTC",
            "amount": "0.12345678",
            "type": "liquidation"
        }"#;

        let record: InsuranceRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.t, 1640995200);
        assert_eq!(record.currency, "BTC");
        assert_eq!(record.amount, "0.12345678");
        assert_eq!(record.record_type, "liquidation");
    }

    #[test]
    fn test_insurance_record_fee_type() {
        let json = r#"{
            "t": 1640995300,
            "currency": "USDT",
            "amount": "1000.50",
            "type": "fee"
        }"#;

        let record: InsuranceRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.t, 1640995300);
        assert_eq!(record.currency, "USDT");
        assert_eq!(record.amount, "1000.50");
        assert_eq!(record.record_type, "fee");
    }

    #[test]
    fn test_insurance_record_large_amounts() {
        let json = r#"{
            "t": 1640995400,
            "currency": "BTC",
            "amount": "999999.99999999",
            "type": "liquidation"
        }"#;

        let record: InsuranceRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.amount, "999999.99999999");
    }

    #[test]
    fn test_insurance_record_small_amounts() {
        let json = r#"{
            "t": 1640995500,
            "currency": "ETH",
            "amount": "0.00000001",
            "type": "fee"
        }"#;

        let record: InsuranceRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.amount, "0.00000001");
    }

    #[test]
    fn test_insurance_record_zero_amount() {
        let json = r#"{
            "t": 1640995600,
            "currency": "USDT",
            "amount": "0",
            "type": "fee"
        }"#;

        let record: InsuranceRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.amount, "0");
    }

    #[test]
    fn test_insurance_record_negative_timestamp() {
        let json = r#"{
            "t": -1640995200,
            "currency": "BTC",
            "amount": "1.0",
            "type": "liquidation"
        }"#;

        let record: InsuranceRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.t, -1640995200);
    }

    #[test]
    fn test_insurance_record_max_timestamp() {
        let json = format!(r#"{{
            "t": {},
            "currency": "BTC",
            "amount": "1.0",
            "type": "liquidation"
        }}"#, i64::MAX);

        let record: InsuranceRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(record.t, i64::MAX);
    }

    #[test]
    fn test_insurance_record_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "USDC", "BNB", "SOL", "ADA", "DOT"];
        
        for currency in currencies {
            let json = format!(r#"{{
                "t": 1640995200,
                "currency": "{}",
                "amount": "100.0",
                "type": "fee"
            }}"#, currency);

            let record: InsuranceRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.currency, currency);
        }
    }

    #[test]
    fn test_insurance_record_different_types() {
        let types = vec!["liquidation", "fee", "transfer", "adjustment", "other"];
        
        for record_type in types {
            let json = format!(r#"{{
                "t": 1640995200,
                "currency": "USDT",
                "amount": "100.0",
                "type": "{}"
            }}"#, record_type);

            let record: InsuranceRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.record_type, record_type);
        }
    }

    #[test]
    fn test_insurance_record_array_deserialization() {
        let json = r#"[
            {
                "t": 1640995200,
                "currency": "BTC",
                "amount": "0.5",
                "type": "liquidation"
            },
            {
                "t": 1640995300,
                "currency": "ETH",
                "amount": "10.0",
                "type": "fee"
            },
            {
                "t": 1640995400,
                "currency": "USDT",
                "amount": "1000.0",
                "type": "liquidation"
            }
        ]"#;

        let records: Vec<InsuranceRecord> = serde_json::from_str(json).unwrap();
        assert_eq!(records.len(), 3);
        
        assert_eq!(records[0].t, 1640995200);
        assert_eq!(records[0].currency, "BTC");
        assert_eq!(records[0].amount, "0.5");
        assert_eq!(records[0].record_type, "liquidation");
        
        assert_eq!(records[1].t, 1640995300);
        assert_eq!(records[1].currency, "ETH");
        assert_eq!(records[1].amount, "10.0");
        assert_eq!(records[1].record_type, "fee");
        
        assert_eq!(records[2].t, 1640995400);
        assert_eq!(records[2].currency, "USDT");
        assert_eq!(records[2].amount, "1000.0");
        assert_eq!(records[2].record_type, "liquidation");
    }

    #[test]
    fn test_insurance_record_empty_array_deserialization() {
        let json = r#"[]"#;
        let records: Vec<InsuranceRecord> = serde_json::from_str(json).unwrap();
        assert_eq!(records.len(), 0);
    }

    #[test]
    fn test_insurance_record_serialization() {
        let record = InsuranceRecord {
            t: 1640995200,
            currency: "BTC".to_string(),
            amount: "0.12345678".to_string(),
            record_type: "liquidation".to_string(),
        };

        let json = serde_json::to_value(&record).unwrap();
        assert_eq!(json["t"], 1640995200);
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["amount"], "0.12345678");
        assert_eq!(json["type"], "liquidation");
    }

    #[test]
    fn test_insurance_record_round_trip() {
        let original = InsuranceRecord {
            t: 1640995200,
            currency: "ETH".to_string(),
            amount: "123.456789".to_string(),
            record_type: "fee".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: InsuranceRecord = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.t, original.t);
        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.amount, original.amount);
        assert_eq!(deserialized.record_type, original.record_type);
    }

    #[test]
    fn test_insurance_record_realistic_liquidation() {
        let json = r#"{
            "t": 1640995200,
            "currency": "BTC",
            "amount": "0.25",
            "type": "liquidation"
        }"#;

        let record: InsuranceRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.currency, "BTC");
        assert_eq!(record.amount, "0.25");
        assert_eq!(record.record_type, "liquidation");
        
        // Liquidations typically involve larger amounts
        let amount: f64 = record.amount.parse().unwrap();
        assert!(amount > 0.0);
    }

    #[test]
    fn test_insurance_record_realistic_fee_contribution() {
        let json = r#"{
            "t": 1640995300,
            "currency": "USDT",
            "amount": "50000.0",
            "type": "fee"
        }"#;

        let record: InsuranceRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.currency, "USDT");
        assert_eq!(record.amount, "50000.0");
        assert_eq!(record.record_type, "fee");
        
        // Fee contributions are typically in stablecoins and larger amounts
        let amount: f64 = record.amount.parse().unwrap();
        assert!(amount > 1000.0);
    }

    #[test]
    fn test_insurance_record_time_sequence() {
        let json = r#"[
            {
                "t": 1640995200,
                "currency": "BTC",
                "amount": "0.1",
                "type": "fee"
            },
            {
                "t": 1640995300,
                "currency": "BTC",
                "amount": "0.2",
                "type": "liquidation"
            },
            {
                "t": 1640995400,
                "currency": "BTC",
                "amount": "0.15",
                "type": "fee"
            }
        ]"#;

        let records: Vec<InsuranceRecord> = serde_json::from_str(json).unwrap();
        
        // Verify chronological order
        for i in 1..records.len() {
            assert!(records[i].t > records[i-1].t);
        }
        
        // Verify mix of record types
        let liquidation_count = records.iter().filter(|r| r.record_type == "liquidation").count();
        let fee_count = records.iter().filter(|r| r.record_type == "fee").count();
        assert_eq!(liquidation_count, 1);
        assert_eq!(fee_count, 2);
    }

    #[test]
    fn test_insurance_record_clone() {
        let original = InsuranceRecord {
            t: 1640995200,
            currency: "BTC".to_string(),
            amount: "0.12345678".to_string(),
            record_type: "liquidation".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.t, original.t);
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.amount, original.amount);
        assert_eq!(cloned.record_type, original.record_type);
    }

    #[test]
    fn test_insurance_record_debug() {
        let record = InsuranceRecord {
            t: 1640995200,
            currency: "BTC".to_string(),
            amount: "0.12345678".to_string(),
            record_type: "liquidation".to_string(),
        };

        let debug_str = format!("{:?}", record);
        assert!(debug_str.contains("InsuranceRecord"));
        assert!(debug_str.contains("1640995200"));
        assert!(debug_str.contains("BTC"));
        assert!(debug_str.contains("0.12345678"));
        assert!(debug_str.contains("liquidation"));
    }
}
