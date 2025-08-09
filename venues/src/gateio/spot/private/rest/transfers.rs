use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::spot::RestResult;

const TRANSFERS_ENDPOINT: &str = "/wallet/transfers";

/// Request parameters for creating a transfer between trading accounts.
#[derive(Debug, Clone, Serialize)]
pub struct CreateTransferRequest {
    /// Currency symbol to transfer (e.g., "USDT", "BTC", "ETH").
    pub currency: String,

    /// Source account type (spot, margin, futures, delivery, cross_margin, options).
    pub from: String,

    /// Destination account type (spot, margin, futures, delivery, cross_margin, options).
    pub to: String,

    /// Transfer amount in the specified currency.
    pub amount: String,

    /// Currency pair for margin transfers (required for margin account transfers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Settle currency for futures/delivery transfers (required for futures/delivery).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle: Option<String>,
}

/// Transfer record representing a completed or pending transfer operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecord {
    /// Currency symbol that was transferred.
    pub currency: String,

    /// Source account type (spot, margin, futures, delivery, cross_margin, options).
    pub from: String,

    /// Destination account type (spot, margin, futures, delivery, cross_margin, options).
    pub to: String,

    /// Transfer amount in the specified currency.
    pub amount: String,

    /// Currency pair associated with margin transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Settle currency for futures/delivery transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle: Option<String>,
}

impl RestClient {
    /// Transfer between trading accounts
    ///
    /// Transfer funds between different account types (spot, margin, futures,
    /// delivery, cross_margin, options) within the same user account.
    ///
    /// [docs]: https://www.gate.com/docs/developers/apiv4/#transfer-between-trading-accounts
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The transfer request parameters with source, destination, and amount
    ///
    /// # Returns
    /// Transfer record confirming the completed or pending transfer operation
    pub async fn create_transfer(
        &self,
        request: CreateTransferRequest,
    ) -> RestResult<TransferRecord> {
        self.post(TRANSFERS_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transfer_request() {
        let request = CreateTransferRequest {
            currency: "USDT".to_string(),
            from: "spot".to_string(),
            to: "margin".to_string(),
            amount: "1000.0".to_string(),
            currency_pair: Some("BTC_USDT".to_string()),
            settle: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["from"], "spot");
        assert_eq!(json["to"], "margin");
        assert_eq!(json["amount"], "1000.0");
        assert_eq!(json["currency_pair"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("settle"));
    }

    #[test]
    fn test_create_transfer_request_futures() {
        let request = CreateTransferRequest {
            currency: "USDT".to_string(),
            from: "spot".to_string(),
            to: "futures".to_string(),
            amount: "500.0".to_string(),
            currency_pair: None,
            settle: Some("USDT".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["from"], "spot");
        assert_eq!(json["to"], "futures");
        assert_eq!(json["amount"], "500.0");
        assert_eq!(json["settle"], "USDT");

        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("currency_pair"));
    }

    #[test]
    fn test_transfer_record_deserialization() {
        let json = r#"{
            "currency": "USDT",
            "from": "spot",
            "to": "margin",
            "amount": "1000.0",
            "currency_pair": "BTC_USDT",
            "settle": null
        }"#;

        let record: TransferRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.currency, "USDT");
        assert_eq!(record.from, "spot");
        assert_eq!(record.to, "margin");
        assert_eq!(record.amount, "1000.0");
        assert_eq!(record.currency_pair.as_ref().unwrap(), "BTC_USDT");
        assert!(record.settle.is_none());
    }

    #[test]
    fn test_different_account_types() {
        let account_types = vec![
            "spot",
            "margin",
            "futures",
            "delivery",
            "cross_margin",
            "options",
        ];

        for from_account in &account_types {
            for to_account in &account_types {
                if from_account != to_account {
                    let request = CreateTransferRequest {
                        currency: "USDT".to_string(),
                        from: from_account.to_string(),
                        to: to_account.to_string(),
                        amount: "100.0".to_string(),
                        currency_pair: None,
                        settle: None,
                    };

                    let json = serde_json::to_value(&request).unwrap();
                    assert_eq!(json["from"], *from_account);
                    assert_eq!(json["to"], *to_account);
                }
            }
        }
    }

    #[test]
    fn test_realistic_transfer_scenarios() {
        // Spot to Margin transfer
        let spot_to_margin = CreateTransferRequest {
            currency: "USDT".to_string(),
            from: "spot".to_string(),
            to: "margin".to_string(),
            amount: "5000.0".to_string(),
            currency_pair: Some("BTC_USDT".to_string()),
            settle: None,
        };

        // Spot to Futures transfer
        let spot_to_futures = CreateTransferRequest {
            currency: "USDT".to_string(),
            from: "spot".to_string(),
            to: "futures".to_string(),
            amount: "2000.0".to_string(),
            currency_pair: None,
            settle: Some("USDT".to_string()),
        };

        // Margin to Spot transfer
        let margin_to_spot = CreateTransferRequest {
            currency: "BTC".to_string(),
            from: "margin".to_string(),
            to: "spot".to_string(),
            amount: "0.1".to_string(),
            currency_pair: Some("BTC_USDT".to_string()),
            settle: None,
        };

        let transfers = vec![spot_to_margin, spot_to_futures, margin_to_spot];

        for transfer in transfers {
            let json = serde_json::to_value(&transfer).unwrap();
            assert!(json["currency"].is_string());
            assert!(json["from"].is_string());
            assert!(json["to"].is_string());
            assert!(json["amount"].is_string());
        }
    }

    #[test]
    fn test_clone_behavior() {
        let transfer = CreateTransferRequest {
            currency: "USDT".to_string(),
            from: "spot".to_string(),
            to: "margin".to_string(),
            amount: "1000.0".to_string(),
            currency_pair: Some("BTC_USDT".to_string()),
            settle: None,
        };
        let cloned_transfer = transfer.clone();
        assert_eq!(cloned_transfer.currency, transfer.currency);
        assert_eq!(cloned_transfer.from, transfer.from);
        assert_eq!(cloned_transfer.to, transfer.to);
    }

    /// Test that optional fields with None values are properly excluded from serialization.
    #[test]
    fn test_optional_field_serialization() {
        let request = CreateTransferRequest {
            currency: "BTC".to_string(),
            from: "spot".to_string(),
            to: "futures".to_string(),
            amount: "0.5".to_string(),
            currency_pair: None,
            settle: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("currency_pair"));
        assert!(!obj.contains_key("settle"));
        assert_eq!(obj.len(), 4); // Only non-None fields
    }
}
