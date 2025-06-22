use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to get max withdrawal
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxWithdrawalRequest {
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Max withdrawal details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxWithdrawal {
    /// Currency
    pub ccy: String,

    /// Max withdrawal amount
    pub max_wd: String,

    /// Max withdrawal amount for spot
    pub max_wd_ex: String,

    /// Spot available balance
    pub spot_offset_max_wd: String,

    /// Whether spot offsetting is enabled
    pub spot_off_set_max_wd_ex: String,
}

impl RestClient {
    /// Get max withdrawal
    ///
    /// # Arguments
    /// * `request` - The get max withdrawal request
    ///
    /// # Returns
    /// A result containing the max withdrawal or an error
    pub async fn get_max_withdrawal(&self, request: &GetMaxWithdrawalRequest) -> RestResult<OkxApiResponse<MaxWithdrawal>> {
        self.send_request(
            "api/v5/account/max-withdrawal",
            reqwest::Method::GET,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_withdrawal_request_serialization() {
        let request = GetMaxWithdrawalRequest {
            ccy: Some("USDT".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=USDT"));
    }

    #[test]
    fn test_get_max_withdrawal_all_currencies() {
        let request = GetMaxWithdrawalRequest { ccy: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_max_withdrawal_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "USDT",
                    "maxWd": "1000.5",
                    "maxWdEx": "800.3",
                    "spotOffsetMaxWd": "200.2",
                    "spotOffSetMaxWdEx": "150.1"
                }
            ]
        }"#;

        let response: OkxApiResponse<MaxWithdrawal> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let withdrawal = &response.data[0];
        assert_eq!(withdrawal.ccy, "USDT");
        assert_eq!(withdrawal.max_wd, "1000.5");
        assert_eq!(withdrawal.max_wd_ex, "800.3");
    }
}
