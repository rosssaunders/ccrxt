use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};

/// Side enum for RFQ requests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    /// Buy side
    Buy,
    /// Sell side
    Sell,
}

/// Request parameters for send RFQ
#[derive(Debug, Clone, Serialize)]
pub struct SendRfqRequest {
    /// Instrument name
    pub instrument_name: String,
    /// Amount (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Side - buy or sell (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
}

/// Response for send RFQ endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendRfqResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Result of method execution. "ok" in case of success
    pub result: String,
}

impl RestClient {
    /// Send RFQ on a given instrument
    ///
    /// Sends Request for Quote (RFQ) on the specified instrument.
    /// This endpoint requires trade:read_write scope.
    ///
    /// See: <https://docs.deribit.com/v2/#private-send_rfq>
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: trade:read_write
    ///
    /// # Arguments
    /// * `instrument_name` - The instrument name
    /// * `amount` - Optional amount
    /// * `side` - Optional side (buy or sell)
    ///
    /// # Returns
    /// Result with "ok" string in case of success
    pub async fn send_rfq(&self, instrument_name: &str, amount: Option<f64>, side: Option<Side>) -> RestResult<SendRfqResponse> {
        let request = SendRfqRequest {
            instrument_name: instrument_name.to_string(),
            amount,
            side,
        };
        self.send_signed_request(
            "private/send_rfq",
            &request,
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::AccountTier;
    use rest::secrets::ExposableSecret;
    use serde_json::{Value, json};

    // Test secret implementation
    #[derive(Clone)]
    struct PlainTextSecret {
        secret: String,
    }

    impl PlainTextSecret {
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    #[test]
    fn test_side_serialization() {
        let buy_side = Side::Buy;
        let sell_side = Side::Sell;

        let buy_json = serde_json::to_string(&buy_side).unwrap();
        let sell_json = serde_json::to_string(&sell_side).unwrap();

        assert_eq!(buy_json, "\"buy\"");
        assert_eq!(sell_json, "\"sell\"");
    }

    #[test]
    fn test_side_deserialization() {
        let buy_side: Side = serde_json::from_str("\"buy\"").unwrap();
        let sell_side: Side = serde_json::from_str("\"sell\"").unwrap();

        matches!(buy_side, Side::Buy);
        matches!(sell_side, Side::Sell);
    }

    #[test]
    fn test_request_parameters_serialization_minimal() {
        let request = SendRfqRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
            amount: None,
            side: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert!(json_value.get("amount").is_none());
        assert!(json_value.get("side").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_full() {
        let request = SendRfqRequest {
            instrument_name: "ETHUSD-PERP".to_string(),
            amount: Some(1.5),
            side: Some(Side::Buy),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("instrument_name").unwrap(), "ETHUSD-PERP");
        assert_eq!(json_value.get("amount").unwrap(), 1.5);
        assert_eq!(json_value.get("side").unwrap(), "buy");
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": "ok"
        });

        let response: SendRfqResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, "ok");
    }

    #[tokio::test]
    async fn test_send_rfq_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let api_key = Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            rate_limiter,
            client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::send_rfq;

        // Verify the client exists
        let _ = &rest_client;

        println!("send_rfq method is accessible and properly typed");
    }
}
