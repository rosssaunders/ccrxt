use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::{ApiResult, RestResult};

const USER_BALANCE_HISTORY_ENDPOINT: &str = "private/user-balance-history";
/// Balance history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceHistoryEntry {
    /// timestamp
    pub t: u64,
    /// total cash balance
    pub c: String,
}

/// Request parameters for user balance history
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetUserBalanceHistoryRequest {
    /// Optional timeframe filter (1H, 4H, 12H, 1D, 1W, 1M, 3M, 6M, 1Y)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,

    /// Optional end time filter in Unix time format (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Optional maximum number of entries to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Result data for get user balance history endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserBalanceHistoryResult {
    /// Instrument name (typically "USD")
    pub instrument_name: String,
    /// Array of balance history entries
    pub data: Vec<BalanceHistoryEntry>,
}

/// Response wrapper for endpoint
pub type GetUserBalanceHistoryResponse = ApiResult<GetUserBalanceHistoryResult>;

impl RestClient {
    /// Get user balance history
    ///
    /// Returns the user's balance history with optional timeframe filtering (H1/D1).
    /// This call may temporarily have discrepancies with that shown on the GUI.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#private-user-balance-history)
    ///
    /// Rate limit: No rate limit
    ///
    /// # Arguments
    /// * `request` - GetUserBalanceHistoryRequest containing optional parameters:
    ///   * `timeframe` - H1 means every hour, D1 means every day. Omit for 'D1'
    ///   * `end_time` - Can be millisecond or nanosecond. Exclusive. If not provided, will be current time.
    ///   * `limit` - If timeframe is D1, max limit will be 30 (days). If timeframe is H1, max limit will be 120 (hours).
    ///
    /// # Returns
    /// User balance history information
    pub async fn get_user_balance_history(
        &self,
        request: GetUserBalanceHistoryRequest,
    ) -> RestResult<GetUserBalanceHistoryResponse> {
        self.send_signed_request(USER_BALANCE_HISTORY_ENDPOINT, request)
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
            "code": 0,
            "id": 1,
            "result": {
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
            }
        });

        let history: GetUserBalanceHistoryResponse = serde_json::from_value(history_json).unwrap();
        assert_eq!(history.result.instrument_name, "USD");
        assert_eq!(history.result.data.len(), 2);
        assert_eq!(history.result.data.first().unwrap().t, 1629478800000_u64);
        assert_eq!(history.result.data.first().unwrap().c, "811.621851");
        assert_eq!(history.result.data.get(1).unwrap().t, 1629565200000_u64);
        assert_eq!(history.result.data.get(1).unwrap().c, "900.123456");
    }

    #[test]
    fn test_balance_history_request_serialization() {
        let request = GetUserBalanceHistoryRequest {
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
        let request = GetUserBalanceHistoryRequest {
            timeframe: None,
            end_time: None,
            limit: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_balance_history_request_partial_fields() {
        let request = GetUserBalanceHistoryRequest {
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
