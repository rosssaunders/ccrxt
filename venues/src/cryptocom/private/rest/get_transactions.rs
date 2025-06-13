use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::cryptocom::RestResult;
use super::client::RestClient;

/// Parameters for get transactions request
#[derive(Debug, Clone, Serialize)]
pub struct GetTransactionsRequest {
    /// e.g. BTCUSD-PERP. Omit for 'all'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    /// Refer to the journal_type in Response Attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_type: Option<String>,
    /// Start time in Unix time format (inclusive). Default: end_time - 1 day. Nanosecond is recommended for accurate pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// End time in Unix time format (exclusive). Default: current system timestamp. Nanosecond is recommended for accurate pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The maximum number of transactions to be retrieved before the end_time. Default: 100. Max: 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Transaction entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEntry {
    /// Account ID
    pub account_id: String,
    /// Event date
    pub event_date: String,
    /// Journal type: TRADING, TRADE_FEE, ONCHAIN_WITHDRAW, ONCHAIN_DEPOSIT, etc.
    pub journal_type: String,
    /// Journal ID
    pub journal_id: String,
    /// Transaction quantity
    pub transaction_qty: String,
    /// Transaction cost
    pub transaction_cost: String,
    /// Realized PNL
    pub realized_pnl: String,
    /// Order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Trade ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    /// Trade match ID applicable to trades only. Non-trade related transactions will have zero or null value.
    pub trade_match_id: String,
    /// Event timestamp in milliseconds
    pub event_timestamp_ms: u64,
    /// Event timestamp in nanoseconds
    pub event_timestamp_ns: String,
    /// Client Order ID (can be empty)
    pub client_oid: String,
    /// MAKER or TAKER or empty
    pub taker_side: String,
    /// BUY or SELL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// e.g. BTCUSD-PERP
    pub instrument_name: String,
}

/// Response for get transactions endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionsResponse {
    /// Array of transaction data
    pub data: Vec<TransactionEntry>,
}

impl RestClient {
    /// Get transactions
    ///
    /// Fetches recent transactions.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html#private-get-transactions>
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `instrument_name` - e.g. BTCUSD-PERP. Omit for 'all'
    /// * `journal_type` - Refer to the journal_type in Response Attributes
    /// * `start_time` - Start time in Unix time format (inclusive). Default: end_time - 1 day. Nanosecond is recommended for accurate pagination
    /// * `end_time` - End time in Unix time format (exclusive). Default: current system timestamp. Nanosecond is recommended for accurate pagination
    /// * `limit` - The maximum number of transactions to be retrieved before the end_time. Default: 100. Max: 100.
    ///
    /// # Returns
    /// Transaction history information
    pub async fn get_transactions(
        &self,
        instrument_name: Option<String>,
        journal_type: Option<String>,
        start_time: Option<String>,
        end_time: Option<String>,
        limit: Option<i32>
    ) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        
        let mut params = json!({});
        if let Some(in_name) = instrument_name {
            params["instrument_name"] = Value::String(in_name);
        }
        if let Some(jt) = journal_type {
            params["journal_type"] = Value::String(jt);
        }
        if let Some(st) = start_time {
            params["start_time"] = Value::String(st);
        }
        if let Some(et) = end_time {
            params["end_time"] = Value::String(et);
        }
        if let Some(l) = limit {
            params["limit"] = Value::Number(l.into());
        }
        
        let signature = self.sign_request("private/get-transactions", id, &params, nonce)?;
        
        let request_body = json!({
            "id": id,
            "method": "private/get-transactions",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self.client
            .post(&format!("{}/v1/private/get-transactions", self.base_url))
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
    struct PlainTextSecret {
        secret: String,
    }
    
    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }
    
    impl PlainTextSecret {
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_transaction_entry_structure_trading() {
        let entry_json = json!({
            "account_id": "88888888-8888-8888-8888-000000000123",
            "event_date": "2021-02-18",
            "journal_type": "TRADING",
            "journal_id": "187078",
            "transaction_qty": "-0.0005",
            "transaction_cost": "-24.500000",
            "realized_pnl": "-0.006125",
            "order_id": "72062",
            "trade_id": "71497",
            "trade_match_id": "8625",
            "event_timestamp_ms": 1613640752166_u64,
            "event_timestamp_ns": "1613640752166234567",
            "client_oid": "6ac2421d-5078-4ef6-a9d5-9680602ce123",
            "taker_side": "MAKER",
            "side": "SELL",
            "instrument_name": "BTCUSD-PERP"
        });

        let entry: TransactionEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.account_id, "88888888-8888-8888-8888-000000000123");
        assert_eq!(entry.journal_type, "TRADING");
        assert_eq!(entry.journal_id, "187078");
        assert_eq!(entry.transaction_qty, "-0.0005");
        assert_eq!(entry.side, Some("SELL".to_string()));
        assert_eq!(entry.instrument_name, "BTCUSD-PERP");
    }

    #[test]
    fn test_transaction_entry_structure_session_settle() {
        let entry_json = json!({
            "account_id": "88888888-8888-8888-8888-000000000123",
            "event_date": "2021-02-18",
            "journal_type": "SESSION_SETTLE",
            "journal_id": "186959",
            "transaction_qty": "0",
            "transaction_cost": "0.000000",
            "realized_pnl": "-0.007800",
            "trade_match_id": "0",
            "event_timestamp_ms": 1613638800001_u64,
            "event_timestamp_ns": "1613638800001124563",
            "client_oid": "",
            "taker_side": "",
            "instrument_name": "BTCUSD-PERP"
        });

        let entry: TransactionEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.account_id, "88888888-8888-8888-8888-000000000123");
        assert_eq!(entry.journal_type, "SESSION_SETTLE");
        assert_eq!(entry.journal_id, "186959");
        assert_eq!(entry.transaction_qty, "0");
        assert_eq!(entry.side, None);
        assert_eq!(entry.instrument_name, "BTCUSD-PERP");
    }

    #[test]
    fn test_transactions_request_serialization() {
        let request = GetTransactionsRequest {
            instrument_name: Some("BTCUSD-PERP".to_string()),
            journal_type: Some("TRADING".to_string()),
            start_time: Some("1619089031996081486".to_string()),
            end_time: Some("1619200052124211357".to_string()),
            limit: Some(20),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value["instrument_name"], "BTCUSD-PERP");
        assert_eq!(json_value["journal_type"], "TRADING");
        assert_eq!(json_value["start_time"], "1619089031996081486");
        assert_eq!(json_value["end_time"], "1619200052124211357");
        assert_eq!(json_value["limit"], 20);
    }

    #[test]
    fn test_transactions_request_optional_fields() {
        let request = GetTransactionsRequest {
            instrument_name: None,
            journal_type: None,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_transactions_request_partial_fields() {
        let request = GetTransactionsRequest {
            instrument_name: Some("BTCUSD-PERP".to_string()),
            journal_type: None,
            start_time: None,
            end_time: None,
            limit: Some(50),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value["instrument_name"], "BTCUSD-PERP");
        assert_eq!(json_value["limit"], 50);
        assert!(!json_value.as_object().unwrap().contains_key("journal_type"));
        assert!(!json_value.as_object().unwrap().contains_key("start_time"));
        assert!(!json_value.as_object().unwrap().contains_key("end_time"));
    }
}