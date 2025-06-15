use super::client::RestClient;
use crate::deribit::{RestResult, EndpointType};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Supported currencies for subaccount transfers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Currency {
    #[serde(rename = "BTC")]
    BTC,
    #[serde(rename = "ETH")]
    ETH,
    #[serde(rename = "USDC")]
    USDC,
    #[serde(rename = "USDT")]
    USDT,
    #[serde(rename = "EURR")]
    EURR,
}

impl Currency {
    /// Convert currency enum to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::BTC => "BTC",
            Currency::ETH => "ETH",
            Currency::USDC => "USDC",
            Currency::USDT => "USDT",
            Currency::EURR => "EURR",
        }
    }
}

/// Request parameters for submit transfer to subaccount
#[derive(Debug, Clone, Serialize)]
pub struct SubmitTransferToSubaccountRequest {
    /// The currency symbol
    pub currency: Currency,
    /// Amount of funds to be transferred
    pub amount: f64,
    /// Id of destination subaccount
    pub destination: i32,
}

/// Transfer direction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TransferDirection {
    Deposit,
    Withdrawal,
}

/// Transfer state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TransferState {
    Prepared,
    Confirmed,
    Cancelled,
    WaitingForAdmin,
    InsufficientFunds,
    WithdrawalLimit,
}

/// Transfer type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TransferType {
    User,
    Subaccount,
}

/// Transfer result data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    /// Amount of funds in given currency
    pub amount: f64,
    /// The timestamp (milliseconds since the Unix epoch)
    pub created_timestamp: i64,
    /// Currency, i.e "BTC", "ETH", "USDC"
    pub currency: String,
    /// Transfer direction
    pub direction: TransferDirection,
    /// Id of transfer
    pub id: i32,
    /// For transfer from/to subaccount returns this subaccount name, for transfer to other account returns address, for transfer from other account returns that accounts username.
    pub other_side: String,
    /// Transfer state
    pub state: TransferState,
    /// Type of transfer: user - sent to user, subaccount - sent to subaccount
    #[serde(rename = "type")]
    pub transfer_type: TransferType,
    /// The timestamp (milliseconds since the Unix epoch)
    pub updated_timestamp: i64,
}

/// Response for submit transfer to subaccount endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitTransferToSubaccountResponse {
    /// The id that was sent in the request
    pub id: Option<i32>,
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// Transfer result data
    pub result: TransferResult,
}

impl RestClient {
    /// Transfer funds to subaccount
    ///
    /// Transfer funds to subaccount.
    ///
    /// See: Deribit API documentation for private/submit_transfer_to_subaccount
    ///
    /// Scope: wallets:read_write
    ///
    /// # Arguments
    /// * `currency` - The currency symbol (BTC, ETH, USDC, USDT, EURR)
    /// * `amount` - Amount of funds to be transferred
    /// * `destination` - Id of destination subaccount
    ///
    /// # Returns
    /// Transfer result with details about the submitted transfer
    pub async fn submit_transfer_to_subaccount(
        &self,
        currency: Currency,
        amount: f64,
        destination: i32,
    ) -> RestResult<TransferResult> {
        // Check rate limits before making the request
        self.rate_limiter.check_limits(EndpointType::from_path("private/submit_transfer_to_subaccount")).await?;

        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let request_id = 1;

        // Create request parameters
        let params = json!({
            "currency": currency.as_str(),
            "amount": amount,
            "destination": destination
        });

        // Create the full request data
        let request_data = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "private/submit_transfer_to_subaccount",
            "params": params
        });

        // Sign the request
        let request_data_str = serde_json::to_string(&request_data)?;
        let signature = self.sign_request(&request_data_str, nonce, request_id)?;

        // Create the final request with authentication
        let authenticated_request = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "private/submit_transfer_to_subaccount",
            "params": params,
            "sig": signature,
            "nonce": nonce,
            "api_key": self.api_key.expose_secret()
        });

        // Make the request
        let response = self
            .client
            .post(format!("{}/api/v2/private/submit_transfer_to_subaccount", self.base_url))
            .json(&authenticated_request)
            .send()
            .await?;

        // Record the request for rate limiting
        self.rate_limiter.record_request(EndpointType::from_path("private/submit_transfer_to_subaccount")).await;

        // Parse the response
        let result: SubmitTransferToSubaccountResponse = response.json().await?;
        Ok(result.result)
    }

    /// Transfer funds to subaccount with string parameters
    ///
    /// Convenience method that accepts string parameters for currency
    ///
    /// # Arguments
    /// * `currency` - The currency symbol as string
    /// * `amount` - Amount of funds to be transferred
    /// * `destination` - Id of destination subaccount
    ///
    /// # Returns
    /// Transfer result with details about the submitted transfer
    pub async fn submit_transfer_to_subaccount_str(
        &self,
        currency: &str,
        amount: f64,
        destination: i32,
    ) -> RestResult<TransferResult> {
        let currency_enum = match currency.to_uppercase().as_str() {
            "BTC" => Currency::BTC,
            "ETH" => Currency::ETH,
            "USDC" => Currency::USDC,
            "USDT" => Currency::USDT,
            "EURR" => Currency::EURR,
            _ => return Err(crate::deribit::Errors::Error(format!("Unsupported currency: {}", currency))),
        };

        self.submit_transfer_to_subaccount(currency_enum, amount, destination).await
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
    fn test_currency_serialization() {
        assert_eq!(serde_json::to_string(&Currency::BTC).unwrap(), "\"BTC\"");
        assert_eq!(serde_json::to_string(&Currency::ETH).unwrap(), "\"ETH\"");
        assert_eq!(serde_json::to_string(&Currency::USDC).unwrap(), "\"USDC\"");
        assert_eq!(serde_json::to_string(&Currency::USDT).unwrap(), "\"USDT\"");
        assert_eq!(serde_json::to_string(&Currency::EURR).unwrap(), "\"EURR\"");
    }

    #[test]
    fn test_currency_deserialization() {
        let btc: Currency = serde_json::from_str("\"BTC\"").unwrap();
        assert_eq!(btc, Currency::BTC);

        let eth: Currency = serde_json::from_str("\"ETH\"").unwrap();
        assert_eq!(eth, Currency::ETH);

        let usdc: Currency = serde_json::from_str("\"USDC\"").unwrap();
        assert_eq!(usdc, Currency::USDC);

        let usdt: Currency = serde_json::from_str("\"USDT\"").unwrap();
        assert_eq!(usdt, Currency::USDT);

        let eurr: Currency = serde_json::from_str("\"EURR\"").unwrap();
        assert_eq!(eurr, Currency::EURR);
    }

    #[test]
    fn test_currency_as_str() {
        assert_eq!(Currency::BTC.as_str(), "BTC");
        assert_eq!(Currency::ETH.as_str(), "ETH");
        assert_eq!(Currency::USDC.as_str(), "USDC");
        assert_eq!(Currency::USDT.as_str(), "USDT");
        assert_eq!(Currency::EURR.as_str(), "EURR");
    }

    #[test]
    fn test_submit_transfer_request_serialization() {
        let request = SubmitTransferToSubaccountRequest {
            currency: Currency::BTC,
            amount: 0.5,
            destination: 12345,
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("amount").unwrap(), 0.5);
        assert_eq!(json_value.get("destination").unwrap(), 12345);
    }

    #[test]
    fn test_transfer_result_deserialization() {
        let result_json = json!({
            "amount": 100.0,
            "created_timestamp": 1638360000000_i64,
            "currency": "BTC",
            "direction": "deposit",
            "id": 123,
            "other_side": "subaccount_1",
            "state": "confirmed",
            "type": "subaccount",
            "updated_timestamp": 1638360000000_i64
        });

        let result: TransferResult = serde_json::from_value(result_json).unwrap();
        assert_eq!(result.amount, 100.0);
        assert_eq!(result.currency, "BTC");
        assert_eq!(result.direction, TransferDirection::Deposit);
        assert_eq!(result.id, 123);
        assert_eq!(result.other_side, "subaccount_1");
        assert_eq!(result.state, TransferState::Confirmed);
        assert_eq!(result.transfer_type, TransferType::Subaccount);
    }

    #[test]
    fn test_different_currencies() {
        let btc_request = SubmitTransferToSubaccountRequest {
            currency: Currency::BTC,
            amount: 0.001,
            destination: 100,
        };

        let json_value = serde_json::to_value(&btc_request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("amount").unwrap(), 0.001);

        let eth_request = SubmitTransferToSubaccountRequest {
            currency: Currency::ETH,
            amount: 1.5,
            destination: 200,
        };

        let json_value = serde_json::to_value(&eth_request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "ETH");
        assert_eq!(json_value.get("amount").unwrap(), 1.5);
        assert_eq!(json_value.get("destination").unwrap(), 200);
    }

    #[test]
    fn test_transfer_states() {
        let states = vec![
            (TransferState::Prepared, "prepared"),
            (TransferState::Confirmed, "confirmed"),
            (TransferState::Cancelled, "cancelled"),
            (TransferState::WaitingForAdmin, "waiting_for_admin"),
            (TransferState::InsufficientFunds, "insufficient_funds"),
            (TransferState::WithdrawalLimit, "withdrawal_limit"),
        ];

        for (state, expected_str) in states {
            let serialized = serde_json::to_string(&state).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: TransferState = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, state);
        }
    }

    #[test]
    fn test_complete_response_deserialization() {
        let response_json = json!({
            "id": 42,
            "jsonrpc": "2.0",
            "result": {
                "amount": 250.0,
                "created_timestamp": 1638360000000_i64,
                "currency": "USDC",
                "direction": "withdrawal",
                "id": 456,
                "other_side": "test_subaccount",
                "state": "prepared",
                "type": "subaccount",
                "updated_timestamp": 1638360000000_i64
            }
        });

        let response: SubmitTransferToSubaccountResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, Some(42));
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.amount, 250.0);
        assert_eq!(response.result.currency, "USDC");
        assert_eq!(response.result.direction, TransferDirection::Withdrawal);
        assert_eq!(response.result.state, TransferState::Prepared);
        assert_eq!(response.result.transfer_type, TransferType::Subaccount);
    }
}