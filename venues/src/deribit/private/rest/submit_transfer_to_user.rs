use super::client::RestClient;
use crate::deribit::{EndpointType, RestResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Request parameters for submit transfer to user
#[derive(Debug, Clone, Serialize)]
pub struct SubmitTransferToUserRequest {
    /// The currency symbol (BTC, ETH, USDC, USDT, EURR)
    pub currency: String,
    /// Amount of funds to be transferred
    pub amount: f64,
    /// Destination wallet's address taken from address book
    pub destination: String,
}

/// Response data for a transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferData {
    /// Amount of funds in given currency
    pub amount: f64,
    /// The timestamp (milliseconds since the Unix epoch)
    pub created_timestamp: i64,
    /// Currency, i.e "BTC", "ETH", "USDC"
    pub currency: String,
    /// Transfer direction
    pub direction: String,
    /// Id of transfer
    pub id: i64,
    /// For transfer from/to subaccount returns this subaccount name, for transfer to other account returns address, for transfer from other account returns that accounts username
    pub other_side: String,
    /// Transfer state
    pub state: String,
    /// Type of transfer: user - sent to user, subaccount - sent to subaccount
    #[serde(rename = "type")]
    pub transfer_type: String,
    /// The timestamp (milliseconds since the Unix epoch)
    pub updated_timestamp: i64,
}

/// Response for submit transfer to user endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitTransferToUserResponse {
    /// The id that was sent in the request
    pub id: i64,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Transfer result data
    pub result: TransferData,
}

impl RestClient {
    /// Transfer funds to another user
    ///
    /// Transfer funds to another user using their wallet address from address book.
    ///
    /// See: <https://docs.deribit.com/v2/#private-submit_transfer_to_user>
    ///
    /// Rate limit: 500 credits per request (non-matching engine)
    /// Scope: wallet:read_write and mainaccount
    ///
    /// # Arguments
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    /// * `amount` - Amount of funds to be transferred
    /// * `destination` - Destination wallet's address taken from address book
    ///
    /// # Returns
    /// Transfer result with complete transfer information
    pub async fn submit_transfer_to_user(
        &self,
        currency: &str,
        amount: f64,
        destination: &str,
    ) -> RestResult<SubmitTransferToUserResponse> {
        // Check rate limits before making the request
        self.rate_limiter.check_limits(EndpointType::NonMatchingEngine).await?;

        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let request_id = 1;

        // Create request parameters
        let params = json!({
            "currency": currency,
            "amount": amount,
            "destination": destination
        });

        // Create the full request data
        let request_data = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "private/submit_transfer_to_user",
            "params": params
        });

        // Sign the request
        let request_data_str = serde_json::to_string(&request_data)?;
        let signature = self.sign_request(&request_data_str, nonce, request_id)?;

        // Create the final request with authentication
        let authenticated_request = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "private/submit_transfer_to_user",
            "params": params,
            "sig": signature,
            "nonce": nonce,
            "api_key": self.api_key.expose_secret()
        });

        // Make the request
        let response = self
            .client
            .post(format!("{}/api/v2/private/submit_transfer_to_user", self.base_url))
            .json(&authenticated_request)
            .send()
            .await?;

        // Record the request for rate limiting
        self.rate_limiter.record_request(EndpointType::NonMatchingEngine).await;

        // Parse the response
        let result: SubmitTransferToUserResponse = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::AccountTier;
    use rest::secrets::ExposableSecret;
    use serde_json::{json, Value};

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
    fn test_request_parameters_serialization() {
        let request = SubmitTransferToUserRequest {
            currency: "BTC".to_string(),
            amount: 0.001,
            destination: "wallet_address_123".to_string(),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("amount").unwrap(), 0.001);
        assert_eq!(json_value.get("destination").unwrap(), "wallet_address_123");
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "amount": 0.001,
                "created_timestamp": 1640995200000i64,
                "currency": "BTC",
                "direction": "payment",
                "id": 12345,
                "other_side": "wallet_address_123",
                "state": "confirmed",
                "type": "user",
                "updated_timestamp": 1640995210000i64
            }
        });

        let response: SubmitTransferToUserResponse = serde_json::from_value(response_json).unwrap();
        
        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.amount, 0.001);
        assert_eq!(response.result.currency, "BTC");
        assert_eq!(response.result.direction, "payment");
        assert_eq!(response.result.state, "confirmed");
        assert_eq!(response.result.transfer_type, "user");
    }

    #[tokio::test]
    async fn test_submit_transfer_to_user_method_exists() {
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
        let _ = RestClient::submit_transfer_to_user;
        
        // Verify the client exists
        let _ = &rest_client;
        
        println!("submit_transfer_to_user method is accessible and properly typed");
    }
}