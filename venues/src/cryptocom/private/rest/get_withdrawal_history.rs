use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Request parameters for get withdrawal history
#[derive(Debug, Clone, Serialize)]
pub struct GetWithdrawalHistoryRequest {
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
    /// Withdrawal status filter (optional)
    /// "0" - Pending, "1" - Processing, "2" - Rejected, "3" - Payment In-progress,
    /// "4" - Payment Failed, "5" - Completed, "6" - Cancelled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Withdrawal history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalHistoryEntry {
    /// Withdrawal ID
    pub id: String,
    /// Currency symbol e.g. BTC, CRO
    pub currency: String,
    /// Withdrawal amount
    pub amount: f64,
    /// Withdrawal fee
    pub fee: f64,
    /// Withdrawal address with Address Tag (if any)
    pub address: String,
    /// Creation timestamp
    pub create_time: u64,
    /// Update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<u64>,
    /// Withdrawal status
    /// "0" - Pending, "1" - Processing, "2" - Rejected, "3" - Payment In-progress,
    /// "4" - Payment Failed, "5" - Completed, "6" - Cancelled
    pub status: String,
    /// Optional Client withdrawal ID if provided in request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_wid: Option<String>,
    /// Transaction hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    /// Network for the transaction - please see get-currency-networks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

/// Response for get withdrawal history endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWithdrawalHistoryResponse {
    /// Array of withdrawal history entries
    pub withdrawal_list: Vec<WithdrawalHistoryEntry>,
}

impl RestClient {
    /// Fetch withdrawal history
    ///
    /// Fetches withdrawal history. Withdrawal setting must be enabled for your API Key.
    /// If you do not see the option when viewing your API Keys, this feature is not yet available for you.
    /// Note: It works for master account only, not for sub-accounts.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/v1/private/get-withdrawal-history>
    ///
    /// # Arguments
    /// * `currency` - Optional currency filter e.g. BTC, CRO
    /// * `start_ts` - Optional start timestamp (default is 90 days from current timestamp)
    /// * `end_ts` - Optional end timestamp (default is current timestamp)
    /// * `page_size` - Optional page size (default: 20, max: 200)
    /// * `page` - Optional page number, 0-based
    /// * `status` - Optional status filter ("0"-"6")
    ///
    /// # Returns
    /// List of withdrawal history entries matching the criteria
    #[allow(clippy::indexing_slicing)] // Safe: adding optional keys to JSON object
    pub async fn get_withdrawal_history(
        &self,
        currency: Option<&str>,
        start_ts: Option<u64>,
        end_ts: Option<u64>,
        page_size: Option<u32>,
        page: Option<u32>,
        status: Option<&str>,
    ) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;

        let mut params = json!({});

        if let Some(c) = currency {
            params["currency"] = Value::String(c.to_string());
        }
        if let Some(st) = start_ts {
            params["start_ts"] = Value::Number(st.into());
        }
        if let Some(et) = end_ts {
            params["end_ts"] = Value::Number(et.into());
        }
        if let Some(ps) = page_size {
            params["page_size"] = Value::Number(ps.into());
        }
        if let Some(p) = page {
            params["page"] = Value::Number(p.into());
        }
        if let Some(s) = status {
            params["status"] = Value::String(s.to_string());
        }

        let signature = self.sign_request("private/get-withdrawal-history", id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": "private/get-withdrawal-history",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(format!(
                "{}/v1/private/get-withdrawal-history",
                self.base_url
            ))
            .json(&request_body)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;
    use serde_json::json;

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
    fn test_get_withdrawal_history_request_structure() {
        let request = GetWithdrawalHistoryRequest {
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
    fn test_get_withdrawal_history_request_minimal() {
        let request = GetWithdrawalHistoryRequest {
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
    fn test_withdrawal_history_entry_structure() {
        let entry_json = json!({
            "currency": "XRP",
            "client_wid": "my_withdrawal_002",
            "fee": 1.0,
            "create_time": 1607063412000_u64,
            "id": "2220",
            "update_time": 1607063460000_u64,
            "amount": 100.0,
            "address": "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBf?1234567890",
            "status": "1",
            "txid": "",
            "network_id": null
        });

        let entry: WithdrawalHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.currency, "XRP");
        assert_eq!(entry.client_wid, Some("my_withdrawal_002".to_string()));
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
        assert_eq!(entry.txid, Some("".to_string()));
        assert_eq!(entry.network_id, None);
    }

    #[test]
    fn test_withdrawal_history_entry_minimal() {
        let entry_json = json!({
            "currency": "BTC",
            "fee": 0.0005,
            "create_time": 1607063412000_u64,
            "id": "2221",
            "amount": 0.1,
            "address": "bc1qxyz123",
            "status": "5"
        });

        let entry: WithdrawalHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.currency, "BTC");
        assert_eq!(entry.client_wid, None);
        assert_eq!(entry.fee, 0.0005);
        assert_eq!(entry.create_time, 1607063412000);
        assert_eq!(entry.id, "2221");
        assert_eq!(entry.update_time, None);
        assert_eq!(entry.amount, 0.1);
        assert_eq!(entry.address, "bc1qxyz123");
        assert_eq!(entry.status, "5");
        assert_eq!(entry.txid, None);
        assert_eq!(entry.network_id, None);
    }

    #[test]
    fn test_get_withdrawal_history_response_structure() {
        let response_json = json!({
            "withdrawal_list": [
                {
                    "currency": "XRP",
                    "client_wid": "my_withdrawal_002",
                    "fee": 1.0,
                    "create_time": 1607063412000_u64,
                    "id": "2220",
                    "update_time": 1607063460000_u64,
                    "amount": 100.0,
                    "address": "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBf?1234567890",
                    "status": "1",
                    "txid": "",
                    "network_id": null
                }
            ]
        });

        let response: GetWithdrawalHistoryResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.withdrawal_list.len(), 1);

        let entry = &response.withdrawal_list.first().unwrap();
        assert_eq!(entry.currency, "XRP");
        assert_eq!(entry.status, "1");
        assert_eq!(entry.amount, 100.0);
        assert_eq!(entry.client_wid, Some("my_withdrawal_002".to_string()));
    }

    #[test]
    fn test_withdrawal_status_meanings() {
        // Test different status values
        let statuses = vec![
            ("0", "Pending"),
            ("1", "Processing"),
            ("2", "Rejected"),
            ("3", "Payment In-progress"),
            ("4", "Payment Failed"),
            ("5", "Completed"),
            ("6", "Cancelled"),
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

            let entry: WithdrawalHistoryEntry = serde_json::from_value(entry_json).unwrap();
            assert_eq!(entry.status, status_code);
        }
    }

    #[test]
    fn test_withdrawal_with_transaction_hash() {
        let entry_json = json!({
            "currency": "ETH",
            "fee": 0.01,
            "create_time": 1607063412000_u64,
            "id": "3333",
            "amount": 1.5,
            "address": "0x1234567890abcdef",
            "status": "5",
            "txid": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
            "network_id": "ETH"
        });

        let entry: WithdrawalHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.currency, "ETH");
        assert_eq!(entry.status, "5");
        assert_eq!(
            entry.txid,
            Some("0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890".to_string())
        );
        assert_eq!(entry.network_id, Some("ETH".to_string()));
    }
}
