use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

const TRADES_ENDPOINT: &str = "private/get-trades";
/// Parameters for get trades request
#[derive(Debug, Clone, Serialize)]
pub struct GetTradesRequest {
    /// e.g. BTCUSD-PERP. Omit for 'all'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    /// Start time in Unix time format (inclusive). Default: end_time - 1 day. Nanosecond is recommended for accurate pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// End time in Unix time format (exclusive). Default: current system timestamp. Nanosecond is recommended for accurate pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The maximum number of trades to be retrieved before the end_time. Default: 100. Max: 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Trade entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeEntry {
    /// Account ID
    pub account_id: String,
    /// Event date
    pub event_date: String,
    /// Journal type would be TRADING
    pub journal_type: String,
    /// Trade quantity
    pub traded_quantity: String,
    /// Trade price
    pub traded_price: String,
    /// Trade fees, the negative sign means a deduction on balance
    pub fees: String,
    /// Order ID
    pub order_id: String,
    /// Trade ID
    pub trade_id: String,
    /// Trade match ID
    pub trade_match_id: String,
    /// Client Order ID
    pub client_oid: String,
    /// MAKER or TAKER or empty
    pub taker_side: String,
    /// BUY or SELL
    pub side: String,
    /// e.g. BTCUSD-PERP
    pub instrument_name: String,
    /// e.g. USD
    pub fee_instrument_name: String,
    /// Create timestamp in milliseconds
    pub create_time: u64,
    /// Create timestamp in nanoseconds
    pub create_time_ns: String,
    /// Trade transaction time in nanoseconds
    pub transact_time_ns: String,
    /// Number of orders matched for this trade execution (Optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_count: Option<String>,
    /// Order entry index of corresponding price level was matched (Optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_index: Option<String>,
}

/// Response for get trades endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTradesResponse {
    /// Array of trade data
    pub data: Vec<TradeEntry>,
}

impl RestClient {
    /// Get trades
    ///
    /// Gets all executed trades for a particular instrument.
    /// Users should use user.trade to keep track of real-time trades,
    /// and private/get-trades should primarily be used for recovery;
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
    /// Trade history information
    pub async fn get_trades(&self, params: GetTradesRequest) -> RestResult<GetTradesResponse> {
        self.send_signed_request(TRADES_ENDPOINT, params).await
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
    fn test_trade_entry_structure() {
        let entry_json = json!({
            "account_id": "52e7c00f-1324-5a6z-bfgt-de445bde21a5",
            "event_date": "2021-02-17",
            "journal_type": "TRADING",
            "traded_quantity": "0.0500",
            "traded_price": "51278.5",
            "fees": "-1.025570",
            "order_id": "19708564",
            "trade_id": "38554669",
            "trade_match_id": "76423",
            "client_oid": "7665b001-2753-4d17-b266-61ecb755922d",
            "taker_side": "MAKER",
            "side": "BUY",
            "instrument_name": "BTCUSD-PERP",
            "fee_instrument_name": "USD",
            "create_time": 1613570791060_u64,
            "create_time_ns": "1613570791060827635",
            "transact_time_ns": "1613570791060827635",
            "match_count": "1",
            "match_index": "0"
        });

        let entry: TradeEntry = serde_json::from_value(entry_json).unwrap();
        assert_eq!(entry.account_id, "52e7c00f-1324-5a6z-bfgt-de445bde21a5");
        assert_eq!(entry.trade_id, "38554669");
        assert_eq!(entry.journal_type, "TRADING");
        assert_eq!(entry.side, "BUY");
        assert_eq!(entry.taker_side, "MAKER");
        assert_eq!(entry.instrument_name, "BTCUSD-PERP");
    }

    #[test]
    fn test_trades_request_serialization() {
        let request = GetTradesRequest {
            instrument_name: Some("BTCUSD-PERP".to_string()),
            start_time: Some("1619089031996081486".to_string()),
            end_time: Some("1619200052124211357".to_string()),
            limit: Some(20),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert_eq!(json_value.get("start_time").unwrap(), "1619089031996081486");
        assert_eq!(json_value.get("end_time").unwrap(), "1619200052124211357");
        assert_eq!(json_value.get("limit").unwrap(), 20);
    }

    #[test]
    fn test_trades_request_optional_fields() {
        let request = GetTradesRequest {
            instrument_name: None,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value, json!({}));
    }

    #[test]
    fn test_trades_request_partial_fields() {
        let request = GetTradesRequest {
            instrument_name: Some("BTCUSD-PERP".to_string()),
            start_time: None,
            end_time: Some("1619200052124211357".to_string()),
            limit: None,
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert_eq!(json_value.get("end_time").unwrap(), "1619200052124211357");
        assert!(!json_value.as_object().unwrap().contains_key("start_time"));
        assert!(!json_value.as_object().unwrap().contains_key("limit"));
    }
}
