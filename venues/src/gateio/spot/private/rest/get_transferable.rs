use serde::{Deserialize, Serialize};

use super::RestClient;

const MARGIN_TRANSFERABLE_ENDPOINT: &str = "/margin/transferable";

/// Request parameters for transferable amount
#[derive(Debug, Clone, Serialize)]
pub struct TransferableRequest {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,
}

/// Transferable amount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferableAmount {
    /// Currency
    pub currency: String,

    /// Available amount for transfer
    pub amount: String,
}

impl RestClient {
    /// Get transferable amount
    ///
    /// This endpoint returns the amount that can be transferred for a specific
    /// currency and currency pair in margin trading.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-transferable-amount>
    pub async fn get_transferable(
        &self,
        params: TransferableRequest,
    ) -> crate::gateio::spot::Result<TransferableAmount> {
        self.get_with_query(MARGIN_TRANSFERABLE_ENDPOINT, &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transferable_request_serialization() {
        let request = TransferableRequest {
            currency: "BTC".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_transferable_request_different_pairs() {
        let pairs = vec![
            ("BTC", "BTC_USDT"),
            ("ETH", "ETH_USDT"),
            ("BNB", "BNB_USDT"),
            ("SOL", "SOL_USDC"),
            ("ETH", "ETH_BTC"),
            ("USDC", "USDC_USDT"),
        ];

        for (currency, currency_pair) in pairs {
            let request = TransferableRequest {
                currency: currency.to_string(),
                currency_pair: currency_pair.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], currency);
            assert_eq!(json["currency_pair"], currency_pair);
        }
    }

    #[test]
    fn test_transferable_amount_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "amount": "0.5"
        }"#;

        let transferable: TransferableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(transferable.currency, "BTC");
        assert_eq!(transferable.amount, "0.5");
    }

    #[test]
    fn test_transferable_amount_different_values() {
        let amounts = vec![
            ("BTC", "0.5"),
            ("ETH", "10.0"),
            ("USDT", "50000.0"),
            ("USDC", "25000.0"),
            ("BNB", "100.0"),
            ("SOL", "500.0"),
        ];

        for (currency, amount) in amounts {
            let json = format!(
                r#"{{
                "currency": "{}",
                "amount": "{}"
            }}"#,
                currency, amount
            );

            let transferable: TransferableAmount = serde_json::from_str(&json).unwrap();
            assert_eq!(transferable.currency, currency);
            assert_eq!(transferable.amount, amount);
        }
    }

    #[test]
    fn test_transferable_request_realistic_margin_transfer_scenario() {
        // Scenario: Check transferable BTC for margin trading
        let request = TransferableRequest {
            currency: "BTC".to_string(),
            currency_pair: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_transferable_amount_realistic_available_balance_scenario() {
        let json = r#"{
            "currency": "BTC",
            "amount": "0.75"
        }"#;

        let transferable: TransferableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(transferable.currency, "BTC");
        assert_eq!(transferable.amount, "0.75");

        // Verify amount is a valid decimal
        let amount: f64 = transferable.amount.parse().unwrap();
        assert!(amount > 0.0);
    }

    #[test]
    fn test_transferable_amount_zero_balance() {
        let json = r#"{
            "currency": "ETH",
            "amount": "0"
        }"#;

        let transferable: TransferableAmount = serde_json::from_str(json).unwrap();
        assert_eq!(transferable.currency, "ETH");
        assert_eq!(transferable.amount, "0");
    }

    #[test]
    fn test_transferable_request_endpoint_validation() {
        let request = TransferableRequest {
            currency: "BTC".to_string(),
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
    fn test_transferable_amount_round_trip() {
        let original = TransferableAmount {
            currency: "BTC".to_string(),
            amount: "0.5".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TransferableAmount = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.amount, original.amount);
    }
}