use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Endpoint path for the create-withdrawal API
const CREATE_WITHDRAWAL_ENDPOINT: &str = "private/create-withdrawal";

/// Request parameters for create withdrawal
#[derive(Debug, Clone, Serialize)]
pub struct CreateWithdrawalRequest {
    /// Optional Client withdrawal ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_wid: Option<String>,

    /// Currency symbol e.g. BTC, CRO
    pub currency: String,

    /// Amount to withdraw
    pub amount: String,

    /// Withdrawal address
    pub address: String,

    /// Secondary address identifier for coins like XRP, XLM etc. Also known as memo or tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<String>,

    /// Network ID - must be whitelisted first
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

/// Response for create withdrawal endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWithdrawalResponse {
    /// Newly created withdrawal ID
    pub id: u64,

    /// Currency symbol e.g. BTC, CRO  
    pub symbol: String,

    /// Withdrawal amount
    pub amount: f64,

    /// Withdrawal fee
    pub fee: f64,

    /// Withdrawal address with Address Tag (if any)
    pub address: String,

    /// Optional Client withdrawal ID if provided in request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_wid: Option<String>,

    /// Creation timestamp
    pub create_time: u64,

    /// Network ID if specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

impl RestClient {
    /// Create a withdrawal request
    ///
    /// Creates a withdrawal request. Withdrawal setting must be enabled for your API Key.
    /// If you do not see the option when viewing your API Key, this feature is not yet available for you.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// # Arguments
    /// * `request` - Parameters for creating a withdrawal
    ///
    /// # Returns
    /// Withdrawal creation result with newly created withdrawal details
    pub async fn create_withdrawal(
        &self,
        request: CreateWithdrawalRequest,
    ) -> RestResult<CreateWithdrawalResponse> {
        self.send_signed_request(CREATE_WITHDRAWAL_ENDPOINT, request)
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
    fn test_create_withdrawal_request_structure() {
        let request = CreateWithdrawalRequest {
            client_wid: Some("my_withdrawal_002".to_string()),
            currency: "BTC".to_string(),
            amount: "1".to_string(),
            address: "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBf".to_string(),
            address_tag: Some("".to_string()),
            network_id: None,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("amount").unwrap(), "1");
        assert_eq!(
            json_value.get("address").unwrap(),
            "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBf"
        );
        assert_eq!(json_value.get("client_wid").unwrap(), "my_withdrawal_002");
    }

    #[test]
    fn test_create_withdrawal_request_minimal() {
        let request = CreateWithdrawalRequest {
            client_wid: None,
            currency: "CRO".to_string(),
            amount: "100".to_string(),
            address: "address123".to_string(),
            address_tag: None,
            network_id: None,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "CRO");
        assert_eq!(json_value.get("amount").unwrap(), "100");
        assert_eq!(json_value.get("address").unwrap(), "address123");
        assert!(!json_value.as_object().unwrap().contains_key("client_wid"));
        assert!(!json_value.as_object().unwrap().contains_key("address_tag"));
        assert!(!json_value.as_object().unwrap().contains_key("network_id"));
    }

    #[test]
    fn test_create_withdrawal_response_structure() {
        let response_json = json!({
            "id": 2220,
            "amount": 1.0,
            "fee": 0.0004,
            "symbol": "BTC",
            "address": "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBf",
            "client_wid": "my_withdrawal_002",
            "create_time": 1607063412000_u64,
            "network_id": null
        });

        let response: CreateWithdrawalResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 2220);
        assert_eq!(response.symbol, "BTC");
        assert_eq!(response.amount, 1.0);
        assert_eq!(response.fee, 0.0004);
        assert_eq!(response.address, "2NBqqD5GRJ8wHy1PYyCXTe9ke5226FhavBf");
        assert_eq!(response.client_wid, Some("my_withdrawal_002".to_string()));
        assert_eq!(response.create_time, 1607063412000);
        assert_eq!(response.network_id, None);
    }

    #[test]
    fn test_create_withdrawal_response_without_optional_fields() {
        let response_json = json!({
            "id": 2221,
            "amount": 0.5,
            "fee": 0.0002,
            "symbol": "ETH",
            "address": "0x1234567890abcdef",
            "create_time": 1607063412001_u64
        });

        let response: CreateWithdrawalResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, 2221);
        assert_eq!(response.symbol, "ETH");
        assert_eq!(response.amount, 0.5);
        assert_eq!(response.fee, 0.0002);
        assert_eq!(response.address, "0x1234567890abcdef");
        assert_eq!(response.client_wid, None);
        assert_eq!(response.create_time, 1607063412001);
        assert_eq!(response.network_id, None);
    }
}
