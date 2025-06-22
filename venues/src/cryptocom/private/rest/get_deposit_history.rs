use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Request parameters for get deposit history
#[derive(Debug, Clone, Serialize)]
pub struct GetDepositHistoryRequest {
    /// Currency symbol e.g. BTC, CRO (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Start timestamp (optional, default is 90 days from current timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<u64>,
    /// End timestamp (optional, default is current timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<u64>,
    /// Page size (optional, default: 20, max: 200)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// Page number, 0-based (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Deposit status filter (optional)
    /// "0" - Not Arrived, "1" - Arrived, "2" - Failed, "3" - Pending
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Deposit history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositHistoryEntry {
    /// Deposit ID
    pub id: String,
    /// Currency symbol e.g. BTC, CRO
    pub currency: String,
    /// Deposit amount
    pub amount: f64,
    /// Deposit fee
    pub fee: f64,
    /// Deposit address with Address Tag (if any)
    pub address: String,
    /// Creation timestamp
    pub create_time: u64,
    /// Update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<u64>,
    /// Deposit status
    /// "0" - Not Arrived, "1" - Arrived, "2" - Failed, "3" - Pending
    pub status: String,
}

/// Response for get deposit history endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositHistoryResponse {
    /// Array of deposit history entries
    pub deposit_list: Vec<DepositHistoryEntry>,
}

impl RestClient {
    /// Fetch deposit history
    ///
    /// Fetches deposit history. Withdrawal setting must be enabled for your API Key.
    /// If you do not see the option when viewing your API Keys, this feature is not yet available for you.
    /// Note: It works for master account only, not for sub-accounts.
    ///
    /// See: <>
    ///
    /// # Arguments
    /// * `request` - Parameters for retrieving deposit history
    ///
    /// # Returns
    /// List of deposit history entries matching the criteria
    pub async fn get_deposit_history(&self, request: GetDepositHistoryRequest) -> RestResult<GetDepositHistoryResponse> {
        self.send_signed_request("private/get-deposit-history", request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    use super::*;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        #[allow(dead_code)]
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_get_deposit_history_request_structure() {
        let request = GetDepositHistoryRequest {
            currency: Some("XRP".to_string()),
            start_ts: Some(1587846300000),
            end_ts: Some(1587846358253),
            page_size: Some(2),
            page: Some(0),
            status: Some("1".to_string()),
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "XRP");
        assert_eq!(json_value.get("start_ts").unwrap(), 1587846300000_u64);
        assert_eq!(json_value.get("end_ts").unwrap(), 1587846358253_u64);
        assert_eq!(json_value.get("page_size").unwrap(), 2);
        assert_eq!(json_value.get("page").unwrap(), 0);
        assert_eq!(json_value.get("status").unwrap(), "1");
    }

    #[test]
    fn test_get_deposit_history_request_minimal() {
        let request = GetDepositHistoryRequest {
            currency: None,
            start_ts: None,
            end_ts: None,
            page_size: None,
            page: None,
            status: None,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        let obj = json_value.as_object().unwrap();
        assert!(!obj.contains_key("currency"));
        assert!(!obj.contains_key("start_ts"));
        assert!(!obj.contains_key("end_ts"));
        assert!(!obj.contains_key("page_size"));
        assert!(!obj.contains_key("page"));
        assert!(!obj.contains_key("status"));
    }

    #[test]
    fn test_deposit_history_entry_structure() {
        let entry_json = json!({
            "currency": "XRP",
            "fee": 1.0,
            "create_time": 1607063412000_u64,
            "id": "2220",
            "update_time": 1607063460000_u64,
            "amount": 100.0,
            "address": "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBf?1234567890",
            "status": "1"
        });

        let entry: DepositHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.currency, "XRP");
        assert_eq!(entry.fee, 1.0);
        assert_eq!(entry.create_time, 1607063412000);
        assert_eq!(entry.id, "2220");
        assert_eq!(entry.update_time, Some(1607063460000));
        assert_eq!(entry.amount, 100.0);
        assert_eq!(
            entry.address,
            "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBf?1234567890"
        );
        assert_eq!(entry.status, "1");
    }

    #[test]
    fn test_deposit_history_entry_without_update_time() {
        let entry_json = json!({
            "currency": "BTC",
            "fee": 0.0005,
            "create_time": 1607063412000_u64,
            "id": "2221",
            "amount": 0.1,
            "address": "bc1qxyz123",
            "status": "3"
        });

        let entry: DepositHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.currency, "BTC");
        assert_eq!(entry.fee, 0.0005);
        assert_eq!(entry.create_time, 1607063412000);
        assert_eq!(entry.id, "2221");
        assert_eq!(entry.update_time, None);
        assert_eq!(entry.amount, 0.1);
        assert_eq!(entry.address, "bc1qxyz123");
        assert_eq!(entry.status, "3");
    }

    #[test]
    fn test_get_deposit_history_response_structure() {
        let response_json = json!({
            "deposit_list": [
                {
                    "currency": "XRP",
                    "fee": 1.0,
                    "create_time": 1607063412000_u64,
                    "id": "2220",
                    "update_time": 1607063460000_u64,
                    "amount": 100.0,
                    "address": "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBf?1234567890",
                    "status": "1"
                }
            ]
        });

        let response: GetDepositHistoryResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.deposit_list.len(), 1);

        let entry = &response.deposit_list.first().unwrap();
        assert_eq!(entry.currency, "XRP");
        assert_eq!(entry.status, "1");
        assert_eq!(entry.amount, 100.0);
    }

    #[test]
    fn test_deposit_status_meanings() {
        // Test different status values
        let statuses = vec![
            ("0", "Not Arrived"),
            ("1", "Arrived"),
            ("2", "Failed"),
            ("3", "Pending"),
        ];

        for (status_code, _description) in statuses {
            let entry_json = json!({
                "currency": "BTC",
                "fee": 0.0,
                "create_time": 1607063412000_u64,
                "id": "test",
                "amount": 1.0,
                "address": "test_address",
                "status": status_code
            });

            let entry: DepositHistoryEntry = serde_json::from_value(entry_json).unwrap();
            assert_eq!(entry.status, status_code);
        }
    }
}
