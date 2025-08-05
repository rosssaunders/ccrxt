use serde::{Deserialize, Serialize};

use super::RestClient;

const MARGIN_BORROWABLE_ENDPOINT: &str = "/margin/borrowable";

/// Request parameters for borrowable amount
#[derive(Debug, Clone, Serialize)]
pub struct BorrowableRequest {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,
}

/// Borrowable amount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowableAmount {
    /// Currency
    pub currency: String,

    /// Amount available for borrowing
    pub amount: String,
}

impl RestClient {
    /// Get borrowable amount
    ///
    /// This endpoint returns the amount that can be borrowed for a specific
    /// currency and currency pair in margin trading.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-borrowable-amount>
    pub async fn get_borrowable(
        &self,
        params: BorrowableRequest,
    ) -> crate::gateio::spot::RestResult<BorrowableAmount> {
        self.get_with_query(MARGIN_BORROWABLE_ENDPOINT, &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrowable_request_serialization() {
        let request = BorrowableRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_borrowable_request_different_scenarios() {
        let scenarios = vec![
            ("USDT", "BTC_USDT", "Borrow USDT for BTC pair"),
            ("BTC", "BTC_USDT", "Borrow BTC for BTC pair"),
            ("ETH", "ETH_USDT", "Borrow ETH for ETH pair"),
            ("USDT", "ETH_USDT", "Borrow USDT for ETH pair"),
            ("USDC", "SOL_USDC", "Borrow USDC for SOL pair"),
            ("BTC", "ETH_BTC", "Borrow BTC for ETH/BTC pair"),
        ];

        for (currency, currency_pair, _description) in scenarios {
            let request = BorrowableRequest {
                currency: currency.to_string(),
                currency_pair: currency_pair.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], currency);
            assert_eq!(json["currency_pair"], currency_pair);
        }
    }

    #[test]
    fn test_borrowable_amount_deserialization() {
        let json = r#"{
            "currency": "USDT",
            "amount": "10000.0"
        }"#;

        let borrowable: BorrowableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(borrowable.currency, "USDT");
        assert_eq!(borrowable.amount, "10000.0");
    }

    #[test]
    fn test_borrowable_amount_different_limits() {
        let limits = vec![
            ("BTC", "0.1"),
            ("ETH", "5.0"),
            ("USDT", "100000.0"),
            ("USDC", "75000.0"),
            ("BNB", "500.0"),
            ("SOL", "2000.0"),
        ];

        for (currency, amount) in limits {
            let json = format!(
                r#"{{
                "currency": "{}",
                "amount": "{}"
            }}"#,
                currency, amount
            );

            let borrowable: BorrowableAmount = serde_json::from_str(&json).unwrap();
            assert_eq!(borrowable.currency, currency);
            assert_eq!(borrowable.amount, amount);
        }
    }

    #[test]
    fn test_borrowable_request_realistic_leverage_scenario() {
        // Scenario: Check borrowable USDT for leveraged BTC position
        let request = BorrowableRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_borrowable_amount_realistic_margin_limit_scenario() {
        let json = r#"{
            "currency": "USDT",
            "amount": "150000.0"
        }"#;

        let borrowable: BorrowableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(borrowable.currency, "USDT");
        assert_eq!(borrowable.amount, "150000.0");

        // Verify high borrowing limit for USDT
        let amount: f64 = borrowable.amount.parse().unwrap();
        assert!(amount >= 100000.0); // High limit for stablecoin
    }

    #[test]
    fn test_borrowable_amount_maximum_precision() {
        let json = r#"{
            "currency": "BTC",
            "amount": "0.00000001"
        }"#;

        let borrowable: BorrowableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(borrowable.currency, "BTC");
        assert_eq!(borrowable.amount, "0.00000001");
    }

    #[test]
    fn test_borrowable_request_endpoint_validation() {
        let request = BorrowableRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("currency"));
        assert!(json.as_object().unwrap().contains_key("currency_pair"));

        // Verify required fields are strings
        assert!(json["currency"].is_string());
        assert!(json["currency_pair"].is_string());
    }

    #[test]
    fn test_borrowable_amount_round_trip() {
        let original = BorrowableAmount {
            currency: "USDT".to_string(),
            amount: "10000.0".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: BorrowableAmount = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.amount, original.amount);
    }
}