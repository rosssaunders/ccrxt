use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bybit::{EndpointType, RestResult};

/// Request parameters for withdraw
#[derive(Debug, Clone, Serialize)]
pub struct WithdrawRequest {
    /// Coin to withdraw, e.g. BTC, ETH, USDT
    pub coin: String,
    /// Withdraw amount
    pub amount: String,
    /// Withdraw address
    pub address: String,
    /// Address tag (memo) for certain coins
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Chain name
    pub chain: String,
    /// Account type from which to withdraw
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// Force chain type
    #[serde(rename = "forceChain", skip_serializing_if = "Option::is_none")]
    pub force_chain: Option<i32>,
    /// Beneficiary address (required for UAE users)
    #[serde(rename = "beneficiaryAddress", skip_serializing_if = "Option::is_none")]
    pub beneficiary_address: Option<String>,
    /// Beneficiary name (required for UAE users)
    #[serde(rename = "beneficiaryName", skip_serializing_if = "Option::is_none")]
    pub beneficiary_name: Option<String>,
    /// Beneficiary country code (required for UAE users)
    #[serde(rename = "beneficiaryCountry", skip_serializing_if = "Option::is_none")]
    pub beneficiary_country: Option<String>,
    /// Beneficiary city (required for UAE users)
    #[serde(rename = "beneficiaryCity", skip_serializing_if = "Option::is_none")]
    pub beneficiary_city: Option<String>,
    /// Beneficiary postal code (required for UAE users)
    #[serde(rename = "beneficiaryPostalCode", skip_serializing_if = "Option::is_none")]
    pub beneficiary_postal_code: Option<String>,
}

/// Response for withdraw endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawResponse {
    /// Response code (0 for success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    /// Response message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    /// Extended response information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    /// Response data
    pub result: WithdrawResult,
    /// Response timestamp
    pub time: u64,
}

/// Result data for withdraw response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawResult {
    /// Withdraw ID
    pub id: String,
}

impl RestClient {
    /// Withdraw crypto
    ///
    /// This endpoint allows you to withdraw cryptocurrency from your account to an external address.
    /// The address must be whitelisted first via the web interface.
    ///
    /// **Important for UAE users:** Additional beneficiary information is required:
    /// - beneficiary_address
    /// - beneficiary_name  
    /// - beneficiary_country
    /// - beneficiary_city
    /// - beneficiary_postal_code
    ///
    /// # Arguments
    /// * `request` - The withdraw request parameters
    ///
    /// # Rate Limit
    /// 10 requests per minute
    ///
    /// # Returns
    /// A result containing the withdraw response with withdrawal ID or an error
    pub async fn withdraw(&self, request: WithdrawRequest) -> RestResult<WithdrawResponse> {
        self.send_signed_request(
            "/v5/asset/withdraw/create",
            reqwest::Method::POST,
            request,
            EndpointType::Asset,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdraw_request_structure() {
        let request = WithdrawRequest {
            coin: "BTC".to_string(),
            amount: "0.1".to_string(),
            address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            tag: None,
            chain: "BTC".to_string(),
            account_type: "UNIFIED".to_string(),
            force_chain: None,
            beneficiary_address: None,
            beneficiary_name: None,
            beneficiary_country: None,
            beneficiary_city: None,
            beneficiary_postal_code: None,
        };
        
        assert_eq!(request.coin, "BTC");
        assert_eq!(request.amount, "0.1");
        assert_eq!(request.account_type, "UNIFIED");
    }

    #[test]
    fn test_withdraw_request_with_uae_fields() {
        let request = WithdrawRequest {
            coin: "BTC".to_string(),
            amount: "0.1".to_string(),
            address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            tag: None,
            chain: "BTC".to_string(),
            account_type: "UNIFIED".to_string(),
            force_chain: None,
            beneficiary_address: Some("123 Blockchain Street".to_string()),
            beneficiary_name: Some("John Doe".to_string()),
            beneficiary_country: Some("AE".to_string()),
            beneficiary_city: Some("Dubai".to_string()),
            beneficiary_postal_code: Some("00000".to_string()),
        };
        
        assert_eq!(request.beneficiary_name, Some("John Doe".to_string()));
        assert_eq!(request.beneficiary_country, Some("AE".to_string()));
        assert_eq!(request.beneficiary_city, Some("Dubai".to_string()));
    }

    #[test]
    fn test_withdraw_request_serialization() {
        let request = WithdrawRequest {
            coin: "USDT".to_string(),
            amount: "100".to_string(),
            address: "0x742d35Cc6634C0532925a3b8D4C0B6dd2b6937FD".to_string(),
            tag: Some("memo123".to_string()),
            chain: "ETH".to_string(),
            account_type: "SPOT".to_string(),
            force_chain: Some(1),
            beneficiary_address: None,
            beneficiary_name: None,
            beneficiary_country: None,
            beneficiary_city: None,
            beneficiary_postal_code: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("\"coin\":\"USDT\""));
        assert!(serialized.contains("\"amount\":\"100\""));
        assert!(serialized.contains("\"accountType\":\"SPOT\""));
        assert!(serialized.contains("\"tag\":\"memo123\""));
        assert!(serialized.contains("\"forceChain\":1"));
    }

    #[test]
    fn test_withdraw_response_structure() {
        let response_json = r#"
        {
            "retCode": 0,
            "retMsg": "success",
            "retExtInfo": {},
            "result": {
                "id": "10195"
            },
            "time": 1672734174346
        }
        "#;

        let response: WithdrawResponse = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.ret_code, 0);
        assert_eq!(response.ret_msg, "success");
        assert_eq!(response.result.id, "10195");
        assert_eq!(response.time, 1672734174346);
    }
}