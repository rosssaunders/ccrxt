use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for managing transfers between sub-accounts
const MANAGE_TRANSFER_ENDPOINT: &str = "api/v5/asset/subaccount/transfer";

/// Request to manage transfers between sub-accounts
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManageTransferRequest {
    /// Currency
    pub ccy: String,

    /// Transfer amount
    pub amt: String,

    /// Account type of transfer from sub-account
    /// 6: Funding Account
    /// 18: Trading account
    pub from: String,

    /// Account type of transfer to sub-account
    /// 6: Funding Account
    /// 18: Trading account
    pub to: String,

    /// Sub-account name of the account that transfers funds out
    pub from_sub_account: String,

    /// Sub-account name of the account that transfers funds in
    pub to_sub_account: String,

    /// Whether or not borrowed coins can be transferred out under Multi-currency margin/Portfolio margin
    /// The default is false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_trans: Option<bool>,

    /// Ignore position risk
    /// Default is false
    /// Applicable to Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omit_pos_risk: Option<String>,
}

/// Response from managing transfers between sub-accounts
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManageTransferResponse {
    /// Transfer ID
    pub trans_id: String,
}

impl RestClient {
    /// Master accounts manage the transfers between sub-accounts
    ///
    /// Applies to master accounts only.
    /// Only API keys with `Trade` privilege can call this endpoint.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-master-accounts-manage-the-transfers-between-sub-accounts)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The transfer management request parameters
    ///
    /// # Returns
    /// A result containing the transfer ID confirmation
    pub async fn manage_transfer(
        &self,
        request: ManageTransferRequest,
    ) -> RestResult<ManageTransferResponse> {
        self.send_post_request(
            MANAGE_TRANSFER_ENDPOINT,
            request,
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_manage_transfer_request_serialization() {
        let request = ManageTransferRequest {
            ccy: "BTC".to_string(),
            amt: "0.1".to_string(),
            from: "6".to_string(),
            to: "18".to_string(),
            from_sub_account: "sub_001".to_string(),
            to_sub_account: "sub_002".to_string(),
            loan_trans: Some(true),
            omit_pos_risk: Some("false".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"BTC\""));
        assert!(json.contains("\"amt\":\"0.1\""));
        assert!(json.contains("\"from\":\"6\""));
        assert!(json.contains("\"to\":\"18\""));
        assert!(json.contains("\"fromSubAccount\":\"sub_001\""));
        assert!(json.contains("\"toSubAccount\":\"sub_002\""));
        assert!(json.contains("\"loanTrans\":true"));
        assert!(json.contains("\"omitPosRisk\":\"false\""));
    }

    #[test]
    fn test_manage_transfer_request_minimal() {
        let request = ManageTransferRequest {
            ccy: "ETH".to_string(),
            amt: "2.5".to_string(),
            from: "18".to_string(),
            to: "6".to_string(),
            from_sub_account: "trading_sub".to_string(),
            to_sub_account: "funding_sub".to_string(),
            loan_trans: None,
            omit_pos_risk: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"ETH\""));
        assert!(json.contains("\"amt\":\"2.5\""));
        assert!(json.contains("\"from\":\"18\""));
        assert!(json.contains("\"to\":\"6\""));
        assert!(json.contains("\"fromSubAccount\":\"trading_sub\""));
        assert!(json.contains("\"toSubAccount\":\"funding_sub\""));
        assert!(!json.contains("loanTrans"));
        assert!(!json.contains("omitPosRisk"));
    }

    #[test]
    fn test_manage_transfer_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "transId": "754147"
                }
            ]
        }"#;

        let response: ApiResponse<ManageTransferResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let transfer = &response.data[0];
        assert_eq!(transfer.trans_id, "754147");
    }

    #[test]
    fn test_account_type_values() {
        // Test funding account type
        let funding_request = ManageTransferRequest {
            ccy: "USDT".to_string(),
            amt: "1000".to_string(),
            from: "6".to_string(), // Funding Account
            to: "18".to_string(),  // Trading account
            from_sub_account: "fund_sub".to_string(),
            to_sub_account: "trade_sub".to_string(),
            loan_trans: None,
            omit_pos_risk: None,
        };

        let json = serde_json::to_string(&funding_request).unwrap();
        assert!(json.contains("\"from\":\"6\""));
        assert!(json.contains("\"to\":\"18\""));

        // Test trading account type
        let trading_request = ManageTransferRequest {
            ccy: "USDT".to_string(),
            amt: "500".to_string(),
            from: "18".to_string(), // Trading account
            to: "6".to_string(),    // Funding Account
            from_sub_account: "trade_sub".to_string(),
            to_sub_account: "fund_sub".to_string(),
            loan_trans: None,
            omit_pos_risk: None,
        };

        let json = serde_json::to_string(&trading_request).unwrap();
        assert!(json.contains("\"from\":\"18\""));
        assert!(json.contains("\"to\":\"6\""));
    }

    #[test]
    fn test_boolean_loan_trans_values() {
        let request_true = ManageTransferRequest {
            ccy: "BTC".to_string(),
            amt: "0.01".to_string(),
            from: "6".to_string(),
            to: "18".to_string(),
            from_sub_account: "sub_a".to_string(),
            to_sub_account: "sub_b".to_string(),
            loan_trans: Some(true),
            omit_pos_risk: None,
        };

        let json_true = serde_json::to_string(&request_true).unwrap();
        assert!(json_true.contains("\"loanTrans\":true"));

        let request_false = ManageTransferRequest {
            ccy: "BTC".to_string(),
            amt: "0.01".to_string(),
            from: "6".to_string(),
            to: "18".to_string(),
            from_sub_account: "sub_a".to_string(),
            to_sub_account: "sub_b".to_string(),
            loan_trans: Some(false),
            omit_pos_risk: None,
        };

        let json_false = serde_json::to_string(&request_false).unwrap();
        assert!(json_false.contains("\"loanTrans\":false"));
    }
}
