use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for getting sub-account maximum withdrawals
const GET_MAX_WITHDRAWAL_ENDPOINT: &str = "api/v5/account/subaccount/max-withdrawal";

/// Request to get sub-account maximum withdrawals
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxWithdrawalRequest {
    /// Sub-account name
    pub sub_acct: String,

    /// Single currency or multiple currencies (no more than 20) separated with comma
    /// e.g. "BTC" or "BTC,ETH"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Sub-account maximum withdrawal information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxWithdrawal {
    /// Currency
    pub ccy: String,

    /// Max withdrawal (excluding borrowed assets under Multi-currency margin)
    pub max_wd: String,

    /// Max withdrawal (including borrowed assets under Multi-currency margin)
    pub max_wd_ex: String,

    /// Max withdrawal under Spot-Derivatives risk offset mode (excluding borrowed assets under Portfolio margin)
    /// Applicable to Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_offset_max_wd: Option<String>,

    /// Max withdrawal under Spot-Derivatives risk offset mode (including borrowed assets under Portfolio margin)
    /// Applicable to Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_offset_max_wd_ex: Option<String>,
}

impl RestClient {
    /// Get sub-account maximum withdrawals
    ///
    /// Retrieve the maximum withdrawal information of a sub-account via the master account
    /// (applies to master accounts only). If no currency is specified, the transferable amount
    /// of all owned currencies will be returned.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-get-sub-account-maximum-withdrawals)
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The maximum withdrawal request parameters
    ///
    /// # Returns
    /// A result containing the maximum withdrawal information for the sub-account
    pub async fn get_subaccount_max_withdrawal(
        &self,
        request: GetMaxWithdrawalRequest,
    ) -> RestResult<MaxWithdrawal> {
        self.send_get_request(
            GET_MAX_WITHDRAWAL_ENDPOINT,
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
    fn test_get_max_withdrawal_request_serialization() {
        let request = GetMaxWithdrawalRequest {
            sub_acct: "test_sub_001".to_string(),
            ccy: Some("BTC".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("subAcct=test_sub_001"));
        assert!(serialized.contains("ccy=BTC"));
    }

    #[test]
    fn test_get_max_withdrawal_request_multiple_currencies() {
        let request = GetMaxWithdrawalRequest {
            sub_acct: "test_sub_001".to_string(),
            ccy: Some("BTC,ETH,USDT".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("subAcct=test_sub_001"));
        assert!(
            serialized.contains("ccy=BTC%2CETH%2CUSDT") || serialized.contains("ccy=BTC,ETH,USDT")
        );
    }

    #[test]
    fn test_get_max_withdrawal_request_no_currency() {
        let request = GetMaxWithdrawalRequest {
            sub_acct: "test_sub_002".to_string(),
            ccy: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("subAcct=test_sub_002"));
        assert!(!serialized.contains("ccy"));
    }

    #[test]
    fn test_max_withdrawal_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "maxWd": "0.1",
                    "maxWdEx": "0.2"
                }
            ]
        }"#;

        let response: ApiResponse<MaxWithdrawal> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let withdrawal = &response.data[0];
        assert_eq!(withdrawal.ccy, "BTC");
        assert_eq!(withdrawal.max_wd, "0.1");
        assert_eq!(withdrawal.max_wd_ex, "0.2");
        assert!(withdrawal.spot_offset_max_wd.is_none());
        assert!(withdrawal.spot_offset_max_wd_ex.is_none());
    }

    #[test]
    fn test_max_withdrawal_deserialization_with_spot_offset() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "ETH",
                    "maxWd": "5.0",
                    "maxWdEx": "7.5",
                    "spotOffsetMaxWd": "4.5",
                    "spotOffsetMaxWdEx": "6.8"
                }
            ]
        }"#;

        let response: ApiResponse<MaxWithdrawal> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let withdrawal = &response.data[0];
        assert_eq!(withdrawal.ccy, "ETH");
        assert_eq!(withdrawal.max_wd, "5.0");
        assert_eq!(withdrawal.max_wd_ex, "7.5");
        assert_eq!(withdrawal.spot_offset_max_wd, Some("4.5".to_string()));
        assert_eq!(withdrawal.spot_offset_max_wd_ex, Some("6.8".to_string()));
    }

    #[test]
    fn test_max_withdrawal_deserialization_multiple() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "maxWd": "0.1",
                    "maxWdEx": "0.2"
                },
                {
                    "ccy": "ETH",
                    "maxWd": "5.0",
                    "maxWdEx": "7.5",
                    "spotOffsetMaxWd": "4.5",
                    "spotOffsetMaxWdEx": "6.8"
                },
                {
                    "ccy": "USDT",
                    "maxWd": "1000",
                    "maxWdEx": "1500"
                }
            ]
        }"#;

        let response: ApiResponse<MaxWithdrawal> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 3);

        let btc_withdrawal = &response.data[0];
        assert_eq!(btc_withdrawal.ccy, "BTC");
        assert_eq!(btc_withdrawal.max_wd, "0.1");
        assert_eq!(btc_withdrawal.max_wd_ex, "0.2");
        assert!(btc_withdrawal.spot_offset_max_wd.is_none());

        let eth_withdrawal = &response.data[1];
        assert_eq!(eth_withdrawal.ccy, "ETH");
        assert_eq!(eth_withdrawal.max_wd, "5.0");
        assert_eq!(eth_withdrawal.max_wd_ex, "7.5");
        assert_eq!(eth_withdrawal.spot_offset_max_wd, Some("4.5".to_string()));
        assert_eq!(
            eth_withdrawal.spot_offset_max_wd_ex,
            Some("6.8".to_string())
        );

        let usdt_withdrawal = &response.data[2];
        assert_eq!(usdt_withdrawal.ccy, "USDT");
        assert_eq!(usdt_withdrawal.max_wd, "1000");
        assert_eq!(usdt_withdrawal.max_wd_ex, "1500");
        assert!(usdt_withdrawal.spot_offset_max_wd.is_none());
    }

    #[test]
    fn test_max_withdrawal_deserialization_empty() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": []
        }"#;

        let response: ApiResponse<MaxWithdrawal> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 0);
    }
}
