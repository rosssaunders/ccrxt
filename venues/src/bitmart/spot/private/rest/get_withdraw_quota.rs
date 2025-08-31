use serde::{Deserialize, Serialize};

use crate::bitmart::{RestResult, rate_limit::EndpointType, spot::private_client::RestClient};

const WITHDRAW_QUOTA_ENDPOINT: &str = "/account/v1/withdraw/charge";

/// Request parameters for getting withdraw quota
#[derive(Debug, Serialize)]
pub struct GetWithdrawQuotaRequest {
    /// Token symbol, e.g., 'BTC'
    pub currency: String,
}

/// Response for withdraw quota endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWithdrawQuotaResponse {
    /// Amount available for withdrawal today, unit: BTC
    #[serde(rename = "today_available_withdraw_BTC")]
    pub today_available_withdraw_btc: String,
    /// Minimum withdrawal amount
    pub min_withdraw: String,
    /// Withdrawal amount must be accurate to several decimal places.
    pub withdraw_precision: i32,
    /// Withdrawal fee
    pub withdraw_fee: String,
    /// Withdrawal amount must be an integral multiple of this value. If it is null, it means there is no such requirement.
    #[serde(rename = "withdraw_Precision_GeTen")]
    pub withdraw_precision_ge_ten: Option<i64>,
}

impl RestClient {
    /// Withdraw Quota
    ///
    /// Query withdraw quota for currencies
    ///
    /// Note: This interface is not available for sub-account
    ///
    /// [docs](https://developer-pro.bitmart.com/en/spot/#withdraw-quota-keyed)
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Withdraw quota information
    pub async fn get_withdraw_quota(
        &self,
        request: GetWithdrawQuotaRequest,
    ) -> RestResult<GetWithdrawQuotaResponse> {
        self.send_get_signed_request(
            WITHDRAW_QUOTA_ENDPOINT,
            &request,
            EndpointType::FundingAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_withdraw_quota_request() {
        let request = GetWithdrawQuotaRequest {
            currency: "BTC".to_string(),
        };
        assert_eq!(request.currency, "BTC");
    }

    #[test]
    fn test_get_withdraw_quota_request_serialization() {
        let request = GetWithdrawQuotaRequest {
            currency: "USDT".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("USDT"));
        assert!(serialized.contains("currency"));
    }

    #[test]
    fn test_get_withdraw_quota_response_structure() {
        let response = GetWithdrawQuotaResponse {
            today_available_withdraw_btc: "100.0000".to_string(),
            min_withdraw: "0.00000000".to_string(),
            withdraw_precision: 8,
            withdraw_fee: "0.00000000".to_string(),
            withdraw_precision_ge_ten: Some(10),
        };

        assert_eq!(response.today_available_withdraw_btc, "100.0000");
        assert_eq!(response.min_withdraw, "0.00000000");
        assert_eq!(response.withdraw_precision, 8);
        assert_eq!(response.withdraw_fee, "0.00000000");
        assert_eq!(response.withdraw_precision_ge_ten, Some(10));
    }

    #[test]
    fn test_get_withdraw_quota_response_without_precision_ge_ten() {
        let response = GetWithdrawQuotaResponse {
            today_available_withdraw_btc: "50.0000".to_string(),
            min_withdraw: "0.001".to_string(),
            withdraw_precision: 6,
            withdraw_fee: "0.0005".to_string(),
            withdraw_precision_ge_ten: None,
        };

        assert_eq!(response.today_available_withdraw_btc, "50.0000");
        assert_eq!(response.min_withdraw, "0.001");
        assert_eq!(response.withdraw_precision, 6);
        assert_eq!(response.withdraw_fee, "0.0005");
        assert_eq!(response.withdraw_precision_ge_ten, None);
    }

    #[test]
    fn test_withdraw_quota_serialization_roundtrip() {
        let response = GetWithdrawQuotaResponse {
            today_available_withdraw_btc: "75.5000".to_string(),
            min_withdraw: "0.0001".to_string(),
            withdraw_precision: 4,
            withdraw_fee: "0.002".to_string(),
            withdraw_precision_ge_ten: Some(100),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetWithdrawQuotaResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            response.today_available_withdraw_btc,
            deserialized.today_available_withdraw_btc
        );
        assert_eq!(response.min_withdraw, deserialized.min_withdraw);
        assert_eq!(response.withdraw_precision, deserialized.withdraw_precision);
        assert_eq!(response.withdraw_fee, deserialized.withdraw_fee);
        assert_eq!(
            response.withdraw_precision_ge_ten,
            deserialized.withdraw_precision_ge_ten
        );
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "today_available_withdraw_BTC": "100.0000",
            "min_withdraw": "0.00000000",
            "withdraw_precision": 8,
            "withdraw_fee": "0.00000000",
            "withdraw_Precision_GeTen": 10
        }"#;

        let response: GetWithdrawQuotaResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.today_available_withdraw_btc, "100.0000");
        assert_eq!(response.min_withdraw, "0.00000000");
        assert_eq!(response.withdraw_precision, 8);
        assert_eq!(response.withdraw_fee, "0.00000000");
        assert_eq!(response.withdraw_precision_ge_ten, Some(10));
    }

    #[test]
    fn test_response_json_parsing_null_precision() {
        let json = r#"{
            "today_available_withdraw_BTC": "50.0000",
            "min_withdraw": "0.001",
            "withdraw_precision": 6,
            "withdraw_fee": "0.0005",
            "withdraw_Precision_GeTen": null
        }"#;

        let response: GetWithdrawQuotaResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.today_available_withdraw_btc, "50.0000");
        assert_eq!(response.min_withdraw, "0.001");
        assert_eq!(response.withdraw_precision, 6);
        assert_eq!(response.withdraw_fee, "0.0005");
        assert_eq!(response.withdraw_precision_ge_ten, None);
    }
}
