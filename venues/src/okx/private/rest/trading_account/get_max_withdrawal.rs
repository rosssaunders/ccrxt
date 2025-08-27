use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting maximum withdrawable amount
const ACCOUNT_MAX_WITHDRAWAL_ENDPOINT: &str = "api/v5/account/max-withdrawal";

/// Request parameters for getting maximum withdrawable amount
#[derive(Debug, Clone, Serialize)]
pub struct GetMaxWithdrawalRequest {
    /// Currency, e.g. BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Maximum withdrawable amount
#[derive(Debug, Clone, Deserialize)]
pub struct MaxWithdrawal {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// Maximum withdrawal amount
    #[serde(rename = "maxWd")]
    pub max_wd: String,

    /// Maximum withdrawal amount with borrowing
    #[serde(rename = "maxWdEx")]
    pub max_wd_ex: String,

    /// Spot margin isolated maximum withdrawal amount
    #[serde(rename = "spotOffsetMaxWd")]
    pub spot_offset_max_wd: String,

    /// Spot margin isolated maximum withdrawal amount with borrowing
    #[serde(rename = "spotOffsetMaxWdEx")]
    pub spot_offset_max_wd_ex: String,
}

impl RestClient {
    /// Get maximum withdrawable amount (trading account)
    ///
    /// Retrieve the maximum withdrawable amount for trading account.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-maximum-withdrawals)
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The maximum withdrawal request parameters
    ///
    /// # Returns
    /// A result containing the maximum withdrawable amounts
    pub async fn get_trading_max_withdrawal(
        &self,
        request: GetMaxWithdrawalRequest,
    ) -> RestResult<MaxWithdrawal> {
        self.send_get_request(
            ACCOUNT_MAX_WITHDRAWAL_ENDPOINT,
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
    fn test_get_max_withdrawal_request_serialization() {
        let request = GetMaxWithdrawalRequest {
            ccy: Some("USDT".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"USDT\""));
    }

    #[test]
    fn test_get_max_withdrawal_request_empty() {
        let request = GetMaxWithdrawalRequest { ccy: None };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_max_withdrawal_deserialization() {
        let withdrawal_json = json!({
            "ccy": "USDT",
            "maxWd": "1000.0",
            "maxWdEx": "1500.0",
            "spotOffsetMaxWd": "800.0",
            "spotOffsetMaxWdEx": "1200.0"
        });

        let withdrawal: MaxWithdrawal = serde_json::from_value(withdrawal_json).unwrap();
        assert_eq!(withdrawal.ccy, "USDT");
        assert_eq!(withdrawal.max_wd, "1000.0");
        assert_eq!(withdrawal.max_wd_ex, "1500.0");
        assert_eq!(withdrawal.spot_offset_max_wd, "800.0");
        assert_eq!(withdrawal.spot_offset_max_wd_ex, "1200.0");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "USDT",
                    "maxWd": "1000.0",
                    "maxWdEx": "1500.0",
                    "spotOffsetMaxWd": "800.0",
                    "spotOffsetMaxWdEx": "1200.0"
                }
            ]
        });

        let response: ApiResponse<MaxWithdrawal> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].ccy, "USDT");
    }
}
