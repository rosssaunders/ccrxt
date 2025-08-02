use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const WITHDRAW_ENDPOINT: &str = "/account/v1/withdraw/apply";

/// Request parameters for withdraw
#[derive(Debug, Serialize, Default)]
pub struct WithdrawRequest {
    /// Token symbol, e.g., 'BTC'
    pub currency: String,
    /// The amount of currency to withdraw
    pub amount: String,
    /// Remark (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

    // Parameters for withdraw to the blockchain
    /// Withdraw address (only the address added on the official website is supported)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Address tag (tag Or payment_id Or memo)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_memo: Option<String>,

    // Parameters for withdraw to BitMart account
    /// Account type
    /// - `1` = CID
    /// - `2` = Email
    /// - `3` = Phone
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<i32>,
    /// Account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Phone area code, required when account type is phone, e.g.: 61
    #[serde(rename = "areaCode", skip_serializing_if = "Option::is_none")]
    pub area_code: Option<String>,
}

/// Response for withdraw endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawResponse {
    /// Withdraw ID
    pub withdraw_id: String,
}

impl RestClient {
    /// Withdraw
    ///
    /// Creates a withdraw request from spot account to an external address
    ///
    /// The API can only make withdrawal to verified addresses, and verified addresses can be set by WEB/APP.
    ///
    /// Note: This interface is not available for sub-account
    ///
    /// [docs]: https://developer-pro.bitmart.com/en/spot/#withdraw-signed
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Withdraw response with withdraw ID
    pub async fn withdraw(&self, request: WithdrawRequest) -> RestResult<WithdrawResponse> {
        self.send_post_signed_request(WITHDRAW_ENDPOINT, request,
            EndpointType::FundingAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_withdraw_request() {
        let request = WithdrawRequest {
            currency: "USDT-TRC20".to_string(),
            amount: "100.000".to_string(),
            destination: Some("To Digital Address".to_string()),
            address: Some("0x1EE6FA5A380360**********".to_string()),
            address_memo: Some("".to_string()),
            account_type: None,
            value: None,
            area_code: None,
        };

        assert_eq!(request.currency, "USDT-TRC20");
        assert_eq!(request.amount, "100.000");
        assert_eq!(
            request.address,
            Some("0x1EE6FA5A380360**********".to_string())
        );
        assert_eq!(request.address_memo, Some("".to_string()));
        assert_eq!(request.destination, Some("To Digital Address".to_string()));
        assert_eq!(request.account_type, None);
        assert_eq!(request.value, None);
        assert_eq!(request.area_code, None);
    }

    #[test]
    fn test_bitmart_account_withdraw_request() {
        let request = WithdrawRequest {
            currency: "USDT-TRC20".to_string(),
            amount: "100.000".to_string(),
            destination: None,
            address: None,
            address_memo: None,
            account_type: Some(1),
            value: Some("876940329".to_string()),
            area_code: Some("".to_string()),
        };

        assert_eq!(request.currency, "USDT-TRC20");
        assert_eq!(request.amount, "100.000");
        assert_eq!(request.address, None);
        assert_eq!(request.address_memo, None);
        assert_eq!(request.destination, None);
        assert_eq!(request.account_type, Some(1));
        assert_eq!(request.value, Some("876940329".to_string()));
        assert_eq!(request.area_code, Some("".to_string()));
    }

    #[test]
    fn test_phone_withdraw_request() {
        let request = WithdrawRequest {
            currency: "BTC".to_string(),
            amount: "0.1".to_string(),
            destination: None,
            address: None,
            address_memo: None,
            account_type: Some(3),
            value: Some("1234567890".to_string()),
            area_code: Some("61".to_string()),
        };

        assert_eq!(request.currency, "BTC");
        assert_eq!(request.amount, "0.1");
        assert_eq!(request.account_type, Some(3));
        assert_eq!(request.value, Some("1234567890".to_string()));
        assert_eq!(request.area_code, Some("61".to_string()));
    }

    #[test]
    fn test_withdraw_request_serialization() {
        let request = WithdrawRequest {
            currency: "BTC".to_string(),
            amount: "0.5".to_string(),
            destination: None,
            address: Some("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string()),
            address_memo: None,
            account_type: None,
            value: None,
            area_code: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("BTC"));
        assert!(serialized.contains("0.5"));
        assert!(serialized.contains("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"));
        // Should not contain fields that are None
        assert!(!serialized.contains("address_memo"));
        assert!(!serialized.contains("type"));
    }

    #[test]
    fn test_withdraw_response_structure() {
        let response = WithdrawResponse {
            withdraw_id: "121212".to_string(),
        };

        assert_eq!(response.withdraw_id, "121212");
    }

    #[test]
    fn test_withdraw_response_serialization_roundtrip() {
        let response = WithdrawResponse {
            withdraw_id: "98765".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: WithdrawResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.withdraw_id, deserialized.withdraw_id);
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "withdraw_id": "121212"
        }"#;

        let response: WithdrawResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.withdraw_id, "121212");
    }

    #[test]
    fn test_blockchain_withdraw_serialization() {
        let request = WithdrawRequest {
            currency: "USDT-TRC20".to_string(),
            amount: "100.000".to_string(),
            destination: Some("To Digital Address".to_string()),
            address: Some("0x1EE6FA5A380360**********".to_string()),
            address_memo: Some("".to_string()),
            account_type: None,
            value: None,
            area_code: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        // Check that optional None fields are not included
        assert!(!serialized.contains("\"type\""));
        assert!(!serialized.contains("\"value\""));
        assert!(!serialized.contains("\"areaCode\""));
        // Check that Some fields are included
        assert!(serialized.contains("\"destination\""));
        assert!(serialized.contains("\"address\""));
        assert!(serialized.contains("\"address_memo\""));
    }

    #[test]
    fn test_bitmart_account_withdraw_serialization() {
        let request = WithdrawRequest {
            currency: "USDT-TRC20".to_string(),
            amount: "100.000".to_string(),
            destination: None,
            address: None,
            address_memo: None,
            account_type: Some(1),
            value: Some("876940329".to_string()),
            area_code: Some("".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        // Check that optional None fields are not included
        assert!(!serialized.contains("\"destination\""));
        assert!(!serialized.contains("\"address\""));
        assert!(!serialized.contains("\"address_memo\""));
        // Check that Some fields are included
        assert!(serialized.contains("\"type\""));
        assert!(serialized.contains("\"value\""));
        assert!(serialized.contains("\"areaCode\""));
    }
}
