use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Position close order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClosePositionType {
    Limit,
    Market,
}

/// Request parameters for closing a position
#[derive(Debug, Clone, Serialize)]
pub struct ClosePositionRequest {
    /// Instrument name e.g. BTCUSD-PERP
    pub instrument_name: String,
    /// Order type: LIMIT or MARKET
    #[serde(rename = "type")]
    pub order_type: ClosePositionType,
    /// Price (required for LIMIT orders only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
}

/// Response for closing a position
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct ClosePositionResponse {
    /// Order ID
    pub order_id: String,
    /// Client Order ID
    pub client_oid: String,
}

impl RestClient {
    /// Closes position for a particular instrument/pair (asynchronous)
    ///
    /// This call is asynchronous, so the response is simply a confirmation of the request.
    /// The user.order subscription can be used to check when the order is successfully created.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The close position parameters
    ///
    /// # Returns
    /// Order ID and client order ID
    pub async fn close_position(
        &self,
        request: ClosePositionRequest,
    ) -> RestResult<ClosePositionResponse> {
        self.send_signed_request("private/close-position", request)
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
    fn test_close_position_request_limit_order() {
        let request = ClosePositionRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
            order_type: ClosePositionType::Limit,
            price: Some("30000.0".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert_eq!(serialized.get("type").unwrap(), "LIMIT");
        assert_eq!(serialized.get("price").unwrap(), "30000.0");
    }

    #[test]
    fn test_close_position_request_market_order() {
        let request = ClosePositionRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
            order_type: ClosePositionType::Market,
            price: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTCUSD-PERP");
        assert_eq!(serialized.get("type").unwrap(), "MARKET");
        assert!(!serialized.as_object().unwrap().contains_key("price"));
    }

    #[test]
    fn test_close_position_type_serialization() {
        assert_eq!(
            serde_json::to_value(ClosePositionType::Limit).unwrap(),
            "LIMIT"
        );
        assert_eq!(
            serde_json::to_value(ClosePositionType::Market).unwrap(),
            "MARKET"
        );
    }

    #[test]
    fn test_close_position_response_structure() {
        let response_json = json!({
            "order_id": "15744",
            "client_oid": "1684d6e4-2c55-64e1-52c3-3aa9febc3a23"
        });

        let response: ClosePositionResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.order_id, "15744");
        assert_eq!(response.client_oid, "1684d6e4-2c55-64e1-52c3-3aa9febc3a23");
    }
}
