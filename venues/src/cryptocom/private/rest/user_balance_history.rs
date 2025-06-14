use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Balance history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceHistoryEntry {
    /// timestamp
    pub t: u64,
    /// total cash balance
    pub c: String,
}

/// Response for user balance history endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBalanceHistoryResponse {
    /// instrument name of the balance e.g. USD
    pub instrument_name: String,
    /// Array of balance history data
    pub data: Vec<BalanceHistoryEntry>,
}

/// Parameters for user balance history request
#[derive(Debug, Clone, Serialize)]
pub struct UserBalanceHistoryRequest {
    /// H1 means every hour, D1 means every day. Omit for 'D1'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    /// Can be millisecond or nanosecond. Exclusive. If not provided, will be current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// If timeframe is D1, max limit will be 30 (days). If timeframe is H1, max limit will be 120 (hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl RestClient {
    /// Get user balance history
    ///
    /// Returns the user's balance history with optional timeframe filtering (H1/D1).
    /// This call may temporarily have discrepancies with that shown on the GUI.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html#private-user-balance-history>
    ///
    /// Rate limit: No rate limit
    ///
    /// # Arguments
    /// * `timeframe` - H1 means every hour, D1 means every day. Omit for 'D1'
    /// * `end_time` - Can be millisecond or nanosecond. Exclusive. If not provided, will be current time.
    /// * `limit` - If timeframe is D1, max limit will be 30 (days). If timeframe is H1, max limit will be 120 (hours).
    ///
    /// # Returns
    /// User balance history information
    pub async fn get_user_balance_history(
        &self,
        timeframe: Option<String>,
        end_time: Option<u64>,
        limit: Option<i32>,
    ) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;

        let mut params = json!({});
        if let Some(tf) = timeframe {
            params["timeframe"] = Value::String(tf);
        }
        if let Some(et) = end_time {
            params["end_time"] = Value::Number(et.into());
        }
        if let Some(l) = limit {
            params["limit"] = Value::Number(l.into());
        }

        let signature = self.sign_request("private/user-balance-history", id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": "private/user-balance-history",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(&format!(
                "{}/v1/private/user-balance-history",
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
    fn test_balance_history_entry_structure() {
        let entry_json = json!({
            "t": 1629478800000_u64,
            "c": "811.621851"
        });

        let entry: BalanceHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.t, 1629478800000_u64);
        assert_eq!(entry.c, "811.621851");
    }

    #[test]
    fn test_balance_history_response_structure() {
        let history_json = json!({
            "instrument_name": "USD",
            "data": [
                {
                    "t": 1629478800000_u64,
                    "c": "811.621851"
                },
                {
                    "t": 1629565200000_u64,
                    "c": "900.123456"
                }
            ]
        });

        let history: UserBalanceHistoryResponse = serde_json::from_value(history_json).unwrap();
        assert_eq!(history.instrument_name, "USD");
        assert_eq!(history.data.len(), 2);
        assert_eq!(history.data[0].t, 1629478800000_u64);
        assert_eq!(history.data[0].c, "811.621851");
        assert_eq!(history.data[1].t, 1629565200000_u64);
        assert_eq!(history.data[1].c, "900.123456");
    }

    #[test]
    fn test_balance_history_request_serialization() {
        let request = UserBalanceHistoryRequest {
            timeframe: Some("H1".to_string()),
            end_time: Some(1629478800000_u64),
            limit: Some(10),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("timeframe").unwrap(), "H1");
        assert_eq!(json_value.get("end_time").unwrap(), 1629478800000_u64);
        assert_eq!(json_value.get("limit").unwrap(), 10);
    }

    #[test]
    fn test_balance_history_request_optional_fields() {
        let request = UserBalanceHistoryRequest {
            timeframe: None,
            end_time: None,
            limit: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_balance_history_request_partial_fields() {
        let request = UserBalanceHistoryRequest {
            timeframe: Some("D1".to_string()),
            end_time: None,
            limit: Some(30),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("timeframe").unwrap(), "D1");
        assert_eq!(json_value.get("limit").unwrap(), 30);
        assert!(!json_value.as_object().unwrap().contains_key("end_time"));
    }
}
