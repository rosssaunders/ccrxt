use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for funds transfer
const ASSET_TRANSFER_ENDPOINT: &str = "api/v5/asset/transfer";

/// Request parameters for funds transfer
#[derive(Debug, Clone, Serialize)]
pub struct FundsTransferRequest {
    /// Transfer type
    /// 0: transfer within account
    /// 1: master account to sub-account (Only applicable to API Key from master account)
    /// 2: sub-account to master account (Only applicable to API Key from master account)
    /// 3: sub-account to master account (Only applicable to APIKey from sub-account)
    /// 4: sub-account to sub-account (Only applicable to APIKey from sub-account)
    /// The default is 0.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,

    /// Transfer currency, e.g. USDT
    pub ccy: String,

    /// Amount to be transferred
    pub amt: String,

    /// The remitting account
    /// 6: Funding account
    /// 18: Trading account
    pub from: String,

    /// The beneficiary account
    /// 6: Funding account
    /// 18: Trading account
    pub to: String,

    /// Name of the sub-account
    /// When type is 1/2/4, this parameter is required.
    #[serde(rename = "subAcct", skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,

    /// Whether or not borrowed coins can be transferred out under Spot mode/Multi-currency margin/Portfolio margin
    /// true: borrowed coins can be transferred out
    /// false: borrowed coins cannot be transferred out
    /// the default is false
    #[serde(rename = "loanTrans", skip_serializing_if = "Option::is_none")]
    pub loan_trans: Option<bool>,

    /// Ignore position risk
    /// Default is false
    /// Applicable to Portfolio margin
    #[serde(rename = "omitPosRisk", skip_serializing_if = "Option::is_none")]
    pub omit_pos_risk: Option<String>,

    /// Client-supplied ID
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

/// Funds transfer response
#[derive(Debug, Clone, Deserialize)]
pub struct FundsTransferResponse {
    /// Transfer ID
    #[serde(rename = "transId")]
    pub trans_id: String,

    /// Client-supplied ID
    #[serde(rename = "clientId")]
    pub client_id: String,

    /// Currency
    pub ccy: String,

    /// The remitting account
    pub from: String,

    /// Transfer amount
    pub amt: String,

    /// The beneficiary account
    pub to: String,
}

impl RestClient {
    /// Funds transfer
    ///
    /// This endpoint supports the transfer of funds between your funding account and
    /// trading account, and from the master account to sub-accounts.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#funding-account-rest-api-funds-transfer)
    ///
    /// Rate limit: 2 requests per second
    ///
    /// # Arguments
    /// * `request` - The funds transfer request parameters
    ///
    /// # Returns
    /// A result containing the funds transfer response
    pub async fn funds_transfer(
        &self,
        request: FundsTransferRequest,
    ) -> RestResult<FundsTransferResponse> {
        self.send_post_request(
            ASSET_TRANSFER_ENDPOINT,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_funds_transfer_request_serialization() {
        let request = FundsTransferRequest {
            transfer_type: Some("0".to_string()),
            ccy: "USDT".to_string(),
            amt: "100.5".to_string(),
            from: "6".to_string(),
            to: "18".to_string(),
            sub_acct: None,
            loan_trans: Some(false),
            omit_pos_risk: Some("false".to_string()),
            client_id: Some("client123".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"type\":\"0\""));
        assert!(json.contains("\"ccy\":\"USDT\""));
        assert!(json.contains("\"amt\":\"100.5\""));
        assert!(json.contains("\"from\":\"6\""));
        assert!(json.contains("\"to\":\"18\""));
        assert!(json.contains("\"loanTrans\":false"));
        assert!(json.contains("\"clientId\":\"client123\""));
    }

    #[test]
    fn test_funds_transfer_request_minimal() {
        let request = FundsTransferRequest {
            transfer_type: None,
            ccy: "BTC".to_string(),
            amt: "0.1".to_string(),
            from: "18".to_string(),
            to: "6".to_string(),
            sub_acct: None,
            loan_trans: None,
            omit_pos_risk: None,
            client_id: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"BTC\""));
        assert!(json.contains("\"amt\":\"0.1\""));
        assert!(json.contains("\"from\":\"18\""));
        assert!(json.contains("\"to\":\"6\""));
        assert!(!json.contains("\"type\""));
        assert!(!json.contains("\"subAcct\""));
        assert!(!json.contains("\"loanTrans\""));
    }

    #[test]
    fn test_funds_transfer_response_deserialization() {
        let response_json = json!({
            "transId": "transfer_12345",
            "clientId": "client123",
            "ccy": "USDT",
            "from": "6",
            "amt": "100.5",
            "to": "18"
        });

        let response: FundsTransferResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.trans_id, "transfer_12345");
        assert_eq!(response.client_id, "client123");
        assert_eq!(response.ccy, "USDT");
        assert_eq!(response.from, "6");
        assert_eq!(response.amt, "100.5");
        assert_eq!(response.to, "18");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "transId": "transfer_12345",
                    "clientId": "client123",
                    "ccy": "USDT",
                    "from": "6",
                    "amt": "100.5",
                    "to": "18"
                }
            ]
        });

        let response: ApiResponse<FundsTransferResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].trans_id, "transfer_12345");
        assert_eq!(response.data[0].ccy, "USDT");
    }
}
