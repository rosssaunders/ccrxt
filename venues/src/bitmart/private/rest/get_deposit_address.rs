use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;

/// Request parameters for getting deposit address
#[derive(Debug, Serialize)]
pub struct GetDepositAddressRequest {
    /// Token symbol, e.g., 'BTC'
    pub currency: String,
}

/// Response for deposit address endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositAddressResponse {
    /// Token symbol, e.g., 'BTC'
    pub currency: String,
    /// Token chain
    pub chain: String,
    /// Deposit address
    pub address: String,
    /// Tag (tag/payment_id/memo); If some currencies need to withdraw currency, it will return data. If not, it will return empty string
    pub address_memo: String,
}

impl RestClient {
    /// Get deposit address
    ///
    /// Gets the deposit address for a specific currency
    ///
    /// Note: This interface is not available for sub-account
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/funding_account.md
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Deposit address information
    pub async fn get_deposit_address(&self, request: GetDepositAddressRequest) -> RestResult<GetDepositAddressResponse> {
        self.send_request(
            "/account/v1/deposit/address",
            reqwest::Method::GET,
            Some(&request),
            EndpointType::FundingAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_deposit_address_request() {
        let request = GetDepositAddressRequest {
            currency: "USDT-TRC20".to_string(),
        };
        assert_eq!(request.currency, "USDT-TRC20");
    }

    #[test]
    fn test_get_deposit_address_request_serialization() {
        let request = GetDepositAddressRequest {
            currency: "BTC".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("BTC"));
        assert!(serialized.contains("currency"));
    }

    #[test]
    fn test_get_deposit_address_response_structure() {
        let response = GetDepositAddressResponse {
            currency: "USDT-TRC20".to_string(),
            chain: "USDT-TRC20".to_string(),
            address: "TGR3ghy2b5VLbyAYrmiE15jasR6aPHTvC5".to_string(),
            address_memo: "".to_string(),
        };

        assert_eq!(response.currency, "USDT-TRC20");
        assert_eq!(response.chain, "USDT-TRC20");
        assert_eq!(response.address, "TGR3ghy2b5VLbyAYrmiE15jasR6aPHTvC5");
        assert_eq!(response.address_memo, "");
    }

    #[test]
    fn test_get_deposit_address_response_with_memo() {
        let response = GetDepositAddressResponse {
            currency: "XRP".to_string(),
            chain: "XRP".to_string(),
            address: "rDNa96Q2ZubFz3m4GG6BxXmxyuA3tWBJ4y".to_string(),
            address_memo: "123456789".to_string(),
        };

        assert_eq!(response.currency, "XRP");
        assert_eq!(response.chain, "XRP");
        assert_eq!(response.address, "rDNa96Q2ZubFz3m4GG6BxXmxyuA3tWBJ4y");
        assert_eq!(response.address_memo, "123456789");
    }

    #[test]
    fn test_deposit_address_serialization_roundtrip() {
        let response = GetDepositAddressResponse {
            currency: "USDT-ERC20".to_string(),
            chain: "USDT-ERC20".to_string(),
            address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            address_memo: "".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetDepositAddressResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.currency, deserialized.currency);
        assert_eq!(response.chain, deserialized.chain);
        assert_eq!(response.address, deserialized.address);
        assert_eq!(response.address_memo, deserialized.address_memo);
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "currency": "USDT-TRC20",
            "chain": "USDT-TRC20",
            "address": "TGR3ghy2b5VLbyAYrmiE15jasR6aPHTvC5",
            "address_memo": ""
        }"#;

        let response: GetDepositAddressResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.currency, "USDT-TRC20");
        assert_eq!(response.chain, "USDT-TRC20");
        assert_eq!(response.address, "TGR3ghy2b5VLbyAYrmiE15jasR6aPHTvC5");
        assert_eq!(response.address_memo, "");
    }
}