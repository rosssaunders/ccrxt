use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Parameters for get order history request
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderHistoryRequest {
    /// e.g. BTCUSD-PERP. Omit for 'all'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    /// Start time in Unix time format (inclusive). Default: end_time - 1 day. Nanosecond is recommended for accurate pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// End time in Unix time format (exclusive). Default: current system timestamp. Nanosecond is recommended for accurate pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The maximum number of orders to be retrieved before the end_time. Default: 100. Max: 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Order history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderHistoryEntry {
    /// Account ID
    pub account_id: String,
    /// Order ID
    pub order_id: String,
    /// Client Order ID
    pub client_oid: String,
    /// Order type: MARKET, LIMIT, STOP_LOSS, STOP_LIMIT, TAKE_PROFIT, TAKE_PROFIT_LIMIT
    pub order_type: String,
    /// Time in force: GOOD_TILL_CANCEL, IMMEDIATE_OR_CANCEL, FILL_OR_KILL
    pub time_in_force: String,
    /// Side: BUY or SELL
    pub side: String,
    /// Execution instructions: POST_ONLY, LIQUIDATION
    pub exec_inst: Vec<String>,
    /// Quantity specified in the order
    pub quantity: String,
    /// Limit price specified in the order
    pub limit_price: String,
    /// Order value
    pub order_value: String,
    /// User's maker fee rate
    pub maker_fee_rate: String,
    /// User's taker fee rate
    pub taker_fee_rate: String,
    /// Average price
    pub avg_price: String,
    /// Cumulative executed quantity
    pub cumulative_quantity: String,
    /// Cumulative executed value
    pub cumulative_value: String,
    /// Cumulative executed fee
    pub cumulative_fee: String,
    /// Order status: REJECTED, CANCELED, FILLED, EXPIRED
    pub status: String,
    /// Updated user
    pub update_user_id: String,
    /// Order creation date
    pub order_date: String,
    /// Order creation timestamp
    pub create_time: u64,
    /// Order creation timestamp (nanosecond)
    pub create_time_ns: String,
    /// Order update timestamp
    pub update_time: u64,
    /// Instrument name e.g. BTCUSD-PERP
    pub instrument_name: String,
    /// Currency used for the fees
    pub fee_instrument_name: String,
    /// Reason code (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<u32>,
}

/// Response for get order history endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderHistoryResponse {
    /// Array of order history data
    pub data: Vec<OrderHistoryEntry>,
}

impl RestClient {
    /// Get order history
    ///
    /// Gets the order history for a particular instrument.
    /// Users should use user.order to keep track of real-time order updates,
    /// and private/get-order-history should primarily be used for recovery;
    /// typically when the websocket is disconnected.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `params` - Request parameters including optional instrument_name, start_time, end_time, and limit
    ///
    /// # Returns
    /// Order history information
    pub async fn get_order_history(
        &self,
        params: GetOrderHistoryRequest,
    ) -> RestResult<GetOrderHistoryResponse> {
        self.send_signed_request("private/get-order-history", params)
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
    fn test_order_history_entry_structure() {
        let entry_json = json!({
            "account_id": "52e7c00f-1324-5a6z-bfgt-de445bde21a5",
            "order_id": "18342311",
            "client_oid": "1613571154795",
            "order_type": "LIMIT",
            "time_in_force": "GOOD_TILL_CANCEL",
            "side": "BUY",
            "exec_inst": [],
            "quantity": "0.0001",
            "limit_price": "51000.0",
            "order_value": "3.900100",
            "maker_fee_rate": "0.000250",
            "taker_fee_rate": "0.000400",
            "avg_price": "0.0",
            "cumulative_quantity": "0.0000",
            "cumulative_value": "0.000000",
            "cumulative_fee": "0.000000",
            "status": "CANCELED",
            "update_user_id": "fd797356-55db-48c2-a44d-157aabf702e8",
            "order_date": "2021-02-17",
            "instrument_name": "BTCUSD-PERP",
            "fee_instrument_name": "USD",
            "create_time": 1610905028000_u64,
            "create_time_ns": "1610905028000123456",
            "update_time": 1613571320251_u64
        });

        let entry: OrderHistoryEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.account_id, "52e7c00f-1324-5a6z-bfgt-de445bde21a5");
        assert_eq!(entry.order_id, "18342311");
        assert_eq!(entry.order_type, "LIMIT");
        assert_eq!(entry.side, "BUY");
        assert_eq!(entry.status, "CANCELED");
        assert_eq!(entry.instrument_name, "BTCUSD-PERP");
    }

    #[test]
    fn test_order_history_request_serialization() {
        let request = GetOrderHistoryRequest {
            instrument_name: Some("BTCUSD-PERP".to_string()),
            start_time: Some("1610905028000081486".to_string()),
            end_time: Some("1613570791058211357".to_string()),
            limit: Some(20),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert_eq!(json_value.get("start_time").unwrap(), "1610905028000081486");
        assert_eq!(json_value.get("end_time").unwrap(), "1613570791058211357");
        assert_eq!(json_value.get("limit").unwrap(), 20);
    }

    #[test]
    fn test_order_history_request_optional_fields() {
        let request = GetOrderHistoryRequest {
            instrument_name: None,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_order_history_request_partial_fields() {
        let request = GetOrderHistoryRequest {
            instrument_name: Some("BTCUSD-PERP".to_string()),
            start_time: None,
            end_time: None,
            limit: Some(50),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert_eq!(json_value.get("limit").unwrap(), 50);
        assert!(!json_value.as_object().unwrap().contains_key("start_time"));
        assert!(!json_value.as_object().unwrap().contains_key("end_time"));
    }
}
