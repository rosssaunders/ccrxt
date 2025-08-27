use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting funding account balance
const ASSET_BALANCES_ENDPOINT: &str = "api/v5/asset/balances";

/// Request parameters for getting funding account balance
#[derive(Debug, Clone, Serialize)]
pub struct GetFundingBalanceRequest {
    /// Single currency or multiple currencies (no more than 20) separated with comma, e.g. BTC or BTC,ETH
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Funding account balance information
#[derive(Debug, Clone, Deserialize)]
pub struct FundingAccountBalance {
    /// Currency
    pub ccy: String,

    /// Balance
    pub bal: String,

    /// Frozen balance
    #[serde(rename = "frozenBal")]
    pub frozen_bal: String,

    /// Available balance
    #[serde(rename = "availBal")]
    pub avail_bal: String,
}

impl RestClient {
    /// Get funding account balance
    ///
    /// Retrieve the funding account balances of all the assets and the amount that is
    /// available or on hold. Only asset information of a currency with a balance greater than 0 will be returned.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-balance)
    ///
    /// Rate limit: 6 requests per second
    ///
    /// # Arguments
    /// * `request` - The balance request parameters
    ///
    /// # Returns
    /// A result containing the list of funding account balances
    pub async fn get_funding_account_balance(
        &self,
        request: GetFundingBalanceRequest,
    ) -> RestResult<FundingAccountBalance> {
        self.send_get_request(
            ASSET_BALANCES_ENDPOINT,
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
    fn test_get_funding_account_balance_request_serialization() {
        let request = GetFundingBalanceRequest {
            ccy: Some("BTC,ETH,USDT".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"BTC,ETH,USDT\""));
    }

    #[test]
    fn test_get_funding_account_balance_request_empty() {
        let request = GetFundingBalanceRequest { ccy: None };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_funding_account_balance_deserialization() {
        let balance_json = json!({
            "ccy": "USDT",
            "bal": "1000.5",
            "frozenBal": "50.0",
            "availBal": "950.5"
        });

        let balance: FundingAccountBalance = serde_json::from_value(balance_json).unwrap();
        assert_eq!(balance.ccy, "USDT");
        assert_eq!(balance.bal, "1000.5");
        assert_eq!(balance.frozen_bal, "50.0");
        assert_eq!(balance.avail_bal, "950.5");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "bal": "2.5",
                    "frozenBal": "0.1",
                    "availBal": "2.4"
                },
                {
                    "ccy": "USDT",
                    "bal": "1000.5",
                    "frozenBal": "50.0",
                    "availBal": "950.5"
                }
            ]
        });

        let response: ApiResponse<FundingAccountBalance> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].ccy, "BTC");
        assert_eq!(response.data[1].ccy, "USDT");
    }
}
