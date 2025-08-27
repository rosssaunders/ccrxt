use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for getting sub-account funding balance
const GET_FUNDING_BALANCE_ENDPOINT: &str = "api/v5/asset/subaccount/balances";

/// Request to get sub-account funding balance
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingBalanceRequest {
    /// Sub-account name
    pub sub_acct: String,

    /// Single currency or multiple currencies (no more than 20) separated with comma
    /// e.g. "BTC" or "BTC,ETH"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Sub-account funding balance information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingBalance {
    /// Currency
    pub ccy: String,

    /// Balance
    pub bal: String,

    /// Frozen balance
    pub frozen_bal: String,

    /// Available balance
    pub avail_bal: String,
}

impl RestClient {
    /// Get sub-account funding balance
    ///
    /// Query detailed balance info of Funding Account of a sub-account via the master account
    /// (applies to master accounts only)
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-get-sub-account-funding-balance)
    ///
    /// Rate limit: 6 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The funding balance request parameters
    ///
    /// # Returns
    /// A result containing the detailed funding balance information for the sub-account
    pub async fn get_funding_balance(
        &self,
        request: GetFundingBalanceRequest,
    ) -> RestResult<FundingBalance> {
        self.send_get_request(
            GET_FUNDING_BALANCE_ENDPOINT,
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
    fn test_get_funding_balance_request_serialization() {
        let request = GetFundingBalanceRequest {
            sub_acct: "test_sub_001".to_string(),
            ccy: Some("BTC".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("subAcct=test_sub_001"));
        assert!(serialized.contains("ccy=BTC"));
    }

    #[test]
    fn test_get_funding_balance_request_multiple_currencies() {
        let request = GetFundingBalanceRequest {
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
    fn test_get_funding_balance_request_no_currency() {
        let request = GetFundingBalanceRequest {
            sub_acct: "test_sub_002".to_string(),
            ccy: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("subAcct=test_sub_002"));
        assert!(!serialized.contains("ccy"));
    }

    #[test]
    fn test_funding_balance_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "bal": "0.1",
                    "frozenBal": "0",
                    "availBal": "0.1"
                }
            ]
        }"#;

        let response: ApiResponse<FundingBalance> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let balance = &response.data[0];
        assert_eq!(balance.ccy, "BTC");
        assert_eq!(balance.bal, "0.1");
        assert_eq!(balance.frozen_bal, "0");
        assert_eq!(balance.avail_bal, "0.1");
    }

    #[test]
    fn test_funding_balance_deserialization_multiple() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "bal": "0.1",
                    "frozenBal": "0",
                    "availBal": "0.1"
                },
                {
                    "ccy": "ETH",
                    "bal": "2.5",
                    "frozenBal": "0.5",
                    "availBal": "2.0"
                },
                {
                    "ccy": "USDT",
                    "bal": "1000",
                    "frozenBal": "100",
                    "availBal": "900"
                }
            ]
        }"#;

        let response: ApiResponse<FundingBalance> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 3);

        let btc_balance = &response.data[0];
        assert_eq!(btc_balance.ccy, "BTC");
        assert_eq!(btc_balance.bal, "0.1");
        assert_eq!(btc_balance.frozen_bal, "0");
        assert_eq!(btc_balance.avail_bal, "0.1");

        let eth_balance = &response.data[1];
        assert_eq!(eth_balance.ccy, "ETH");
        assert_eq!(eth_balance.bal, "2.5");
        assert_eq!(eth_balance.frozen_bal, "0.5");
        assert_eq!(eth_balance.avail_bal, "2.0");

        let usdt_balance = &response.data[2];
        assert_eq!(usdt_balance.ccy, "USDT");
        assert_eq!(usdt_balance.bal, "1000");
        assert_eq!(usdt_balance.frozen_bal, "100");
        assert_eq!(usdt_balance.avail_bal, "900");
    }

    #[test]
    fn test_funding_balance_deserialization_empty() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": []
        }"#;

        let response: ApiResponse<FundingBalance> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 0);
    }
}
