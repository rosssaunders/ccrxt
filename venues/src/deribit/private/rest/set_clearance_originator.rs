use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const SET_CLEARANCE_ORIGINATOR_ENDPOINT: &str = "private/set_clearance_originator";

/// Deposit ID parameters for set clearance originator request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositId {
    /// The currency symbol (BTC, ETH, USDC, USDT, EURR)
    pub currency: String,

    /// Id of a (sub)account
    pub user_id: i64,

    /// Address in currency format
    pub address: String,

    /// Transaction id in proper format for the currency
    pub tx_hash: String,
}

/// Originator information for set clearance originator request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Originator {
    /// If the user is the originator of the deposit
    pub is_personal: bool,

    /// Company name of the originator if the originator is a legal entity
    pub company_name: String,

    /// First name if the user is the originator of the deposit
    pub first_name: String,

    /// Last name of the originator if the originator is a person
    pub last_name: String,

    /// Geographical address of the originator
    pub address: String,
}

/// Request parameters for set clearance originator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetClearanceOriginatorRequest {
    /// Id of the deposit
    pub deposit_id: DepositId,

    /// Information about the originator of the deposit
    pub originator: Originator,
}

/// Response data for set clearance originator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetClearanceOriginatorResult {
    /// Address in proper format for currency
    pub address: String,

    /// Amount of funds in given currency
    pub amount: f64,

    /// Clearance state
    pub clearance_state: String,

    /// Currency, i.e "BTC", "ETH", "USDC"
    pub currency: String,

    /// Note
    pub note: String,

    /// The timestamp (milliseconds since the Unix epoch)
    pub received_timestamp: i64,

    /// Transaction id in proper format for currency, null if id is not available
    pub refund_transaction_id: Option<String>,

    /// Address in proper format for currency
    pub source_address: String,

    /// Deposit state
    pub state: String,

    /// Transaction id in proper format for currency, null if id is not available
    pub transaction_id: Option<String>,

    /// The timestamp (milliseconds since the Unix epoch)
    pub updated_timestamp: i64,
}

/// Response for set clearance originator endpoint
pub type SetClearanceOriginatorResponse = JsonRpcResult<SetClearanceOriginatorResult>;

impl RestClient {
    /// Sets the clearance originator for a deposit.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-set_clearance_originator)
    ///
    /// # Arguments
    /// * `params` - Parameters for the request (deposit_id, originator)
    ///
    /// # Returns
    /// Clearance originator response
    pub async fn set_clearance_originator(
        &self,
        params: SetClearanceOriginatorRequest,
    ) -> RestResult<SetClearanceOriginatorResponse> {
        self.send_signed_request(
            SET_CLEARANCE_ORIGINATOR_ENDPOINT,
            &params,
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rest::secrets::ExposableSecret;
    /// REST API endpoint constant
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::AccountTier;
    use crate::deribit::private::rest::credentials::Credentials;
    use rest::secrets::SecretString;

    #[test]
    fn test_deposit_id_serialization() {
        let deposit_id = DepositId {
            currency: "BTC".to_string(),
            user_id: 12345,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            tx_hash: "1a2b3c4d5e6f7890abcdef1234567890abcdef1234567890abcdef1234567890".to_string(),
        };

        let json_str = serde_json::to_string(&deposit_id).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("user_id").unwrap(), 12345);
        assert_eq!(
            json_value.get("address").unwrap(),
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(
            json_value.get("tx_hash").unwrap(),
            "1a2b3c4d5e6f7890abcdef1234567890abcdef1234567890abcdef1234567890"
        );
    }

    #[test]
    fn test_originator_serialization() {
        let originator = Originator {
            is_personal: true,
            company_name: "".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            address: "123 Main St, New York, NY 10001".to_string(),
        };

        let json_str = serde_json::to_string(&originator).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("is_personal").unwrap(), true);
        assert_eq!(json_value.get("company_name").unwrap(), "");
        assert_eq!(json_value.get("first_name").unwrap(), "John");
        assert_eq!(json_value.get("last_name").unwrap(), "Doe");
        assert_eq!(
            json_value.get("address").unwrap(),
            "123 Main St, New York, NY 10001"
        );
    }

    #[test]
    fn test_request_parameters_serialization() {
        let deposit_id = DepositId {
            currency: "ETH".to_string(),
            user_id: 98765,
            address: "0x742d35Cc6634C0532925a3b8D2D1e6e4E7B5b7E4".to_string(),
            tx_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
                .to_string(),
        };

        let originator = Originator {
            is_personal: false,
            company_name: "Acme Corp".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            address: "456 Business Ave, San Francisco, CA 94105".to_string(),
        };

        let request = SetClearanceOriginatorRequest {
            deposit_id,
            originator,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert!(json_value.get("deposit_id").is_some());
        assert!(json_value.get("originator").is_some());

        let deposit_id_value = json_value.get("deposit_id").unwrap();
        assert_eq!(deposit_id_value.get("currency").unwrap(), "ETH");
        assert_eq!(deposit_id_value.get("user_id").unwrap(), 98765);

        let originator_value = json_value.get("originator").unwrap();
        assert_eq!(originator_value.get("is_personal").unwrap(), false);
        assert_eq!(originator_value.get("company_name").unwrap(), "Acme Corp");
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                "amount": 0.001,
                "clearance_state": "success",
                "currency": "BTC",
                "note": "Deposit cleared successfully",
                "received_timestamp": 1640995200000i64,
                "refund_transaction_id": null,
                "source_address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                "state": "completed",
                "transaction_id": "1a2b3c4d5e6f7890abcdef1234567890abcdef1234567890abcdef1234567890",
                "updated_timestamp": 1640995210000i64
            }
        });

        let response: SetClearanceOriginatorResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(
            response.result.address,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(response.result.amount, 0.001);
        assert_eq!(response.result.clearance_state, "success");
        assert_eq!(response.result.currency, "BTC");
        assert_eq!(response.result.note, "Deposit cleared successfully");
        assert_eq!(response.result.received_timestamp, 1640995200000);
        assert_eq!(response.result.refund_transaction_id, None);
        assert_eq!(
            response.result.source_address,
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
        );
        assert_eq!(response.result.state, "completed");
        assert_eq!(
            response.result.transaction_id,
            Some("1a2b3c4d5e6f7890abcdef1234567890abcdef1234567890abcdef1234567890".to_string())
        );
        assert_eq!(response.result.updated_timestamp, 1640995210000);
    }

    #[tokio::test]
    async fn test_set_clearance_originator_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::set_clearance_originator;

        // Verify the client exists
        let _ = &rest_client;

        println!("set_clearance_originator method is accessible and properly typed");
    }
}
