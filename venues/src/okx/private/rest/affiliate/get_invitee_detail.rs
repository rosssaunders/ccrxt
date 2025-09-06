use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const AFFILIATE_INVITEE_DETAIL_ENDPOINT: &str = "api/v5/affiliate/invitee/detail";

/// Request parameters for getting invitee detail
#[derive(Debug, Clone, Serialize)]
pub struct GetInviteeDetailRequest {
    /// UID of the invitee. Only applicable to the UID of invitee master account.
    /// The data returned covers invitee master account and invitee sub-accounts.
    pub uid: String,
}

/// Invitee detail information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteeDetail {
    /// Invitee's relative level to the affiliate
    /// If the user is a invitee, the level will be "2".
    pub invitee_level: String,

    /// Timestamp that the rebate relationship is established, Unix timestamp in millisecond format
    pub join_time: String,

    /// Self rebate rate of the invitee (in decimal), e.g. "0.01" represents 10%
    pub invitee_rebate_rate: String,

    /// Total commission earned from the invitee, unit in USDT
    pub total_commission: String,

    /// Timestamp that the first trade is completed after the latest rebate relationship is established
    /// Unix timestamp in millisecond format. If user has not traded, "" will be returned
    pub first_trade_time: String,

    /// Invitee trading fee level, e.g. "Lv1"
    pub level: String,

    /// Accumulated amount of deposit in USDT
    /// If user has not deposited, "0" will be returned
    pub dep_amt: String,

    /// Accumulated Trading volume in the current month in USDT
    /// If user has not traded, "0" will be returned
    pub vol_month: String,

    /// Accumulated Amount of trading fee in USDT
    /// If there is no any fee, "0" will be returned
    pub acc_fee: String,

    /// KYC2 verification time. Unix timestamp in millisecond format and the precision is in day
    /// If user has not passed KYC2, "" will be returned
    pub kyc_time: String,

    /// User country or region, e.g. "United Kingdom"
    pub region: String,

    /// Affiliate invite code that the invitee registered/recalled via
    pub affiliate_code: String,
}

impl RestClient {
    /// Get the invitee's detail
    ///
    /// Retrieve detailed information about a specific invitee including trading volume,
    /// commissions, and account status.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#affiliate-rest-api-get-the-invitee-39-s-detail)
    ///
    /// Rate limit: 20 requests per 2 seconds
    /// Rate limit rule: User ID
    ///
    /// # Arguments
    /// * `request` - The get invitee detail request containing the invitee UID
    ///
    /// # Returns
    /// Response containing the invitee's detailed information
    pub async fn get_invitee_detail(
        &self,
        request: GetInviteeDetailRequest,
    ) -> RestResult<InviteeDetail> {
        self.send_get_request(
            AFFILIATE_INVITEE_DETAIL_ENDPOINT,
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
    fn test_get_invitee_detail_request_serialization() {
        let request = GetInviteeDetailRequest {
            uid: "12345678".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "uid=12345678");
    }

    #[test]
    fn test_invitee_detail_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [{
                "inviteeLevel": "2",
                "joinTime": "1597026383085",
                "inviteeRebateRate": "0.01",
                "totalCommission": "100.5",
                "firstTradeTime": "1597026383085",
                "level": "Lv1",
                "depAmt": "1000.0",
                "volMonth": "50000.0",
                "accFee": "25.75",
                "kycTime": "1597026383085",
                "region": "United Kingdom",
                "affiliateCode": "ABC123"
            }]
        });

        let response: ApiResponse<InviteeDetail> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let detail = &response.data[0];
        assert_eq!(detail.invitee_level, "2");
        assert_eq!(detail.join_time, "1597026383085");
        assert_eq!(detail.invitee_rebate_rate, "0.01");
        assert_eq!(detail.total_commission, "100.5");
        assert_eq!(detail.first_trade_time, "1597026383085");
        assert_eq!(detail.level, "Lv1");
        assert_eq!(detail.dep_amt, "1000.0");
        assert_eq!(detail.vol_month, "50000.0");
        assert_eq!(detail.acc_fee, "25.75");
        assert_eq!(detail.kyc_time, "1597026383085");
        assert_eq!(detail.region, "United Kingdom");
        assert_eq!(detail.affiliate_code, "ABC123");
    }

    #[test]
    fn test_invitee_detail_with_empty_fields() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [{
                "inviteeLevel": "2",
                "joinTime": "1597026383085",
                "inviteeRebateRate": "0.01",
                "totalCommission": "0",
                "firstTradeTime": "",
                "level": "Lv1",
                "depAmt": "0",
                "volMonth": "0",
                "accFee": "0",
                "kycTime": "",
                "region": "United States",
                "affiliateCode": "DEF456"
            }]
        });

        let response: ApiResponse<InviteeDetail> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let detail = &response.data[0];
        assert_eq!(detail.total_commission, "0");
        assert_eq!(detail.first_trade_time, "");
        assert_eq!(detail.dep_amt, "0");
        assert_eq!(detail.vol_month, "0");
        assert_eq!(detail.acc_fee, "0");
        assert_eq!(detail.kyc_time, "");
    }

    #[test]
    fn test_request_serialization_roundtrip() {
        let original = GetInviteeDetailRequest {
            uid: "987654321".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&original).unwrap();
        let params: std::collections::HashMap<String, String> =
            serde_urlencoded::from_str(&serialized).unwrap();

        assert_eq!(params.get("uid"), Some(&"987654321".to_string()));
    }

    #[test]
    fn test_error_response_handling() {
        let error_response = json!({
            "code": "50001",
            "msg": "Internal system error",
            "data": []
        });

        let response: ApiResponse<InviteeDetail> = serde_json::from_value(error_response).unwrap();
        assert_eq!(response.code, "50001");
        assert_eq!(response.msg, "Internal system error");
        assert_eq!(response.data.len(), 0);
    }

    #[tokio::test]
    async fn test_get_invitee_detail_method_compilation() {
        // This test ensures the method compiles and is accessible
        use secrets::SecretString;

        use crate::okx::{Credentials, RateLimiter};

        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
            api_passphrase: SecretString::from("test_passphrase".to_string()),
        };
        use std::{collections::HashMap, sync::Arc};

        use async_trait::async_trait;
        use rest::{HttpClient, HttpError, Response};

        #[derive(Debug)]
        struct MockHttpClient;

        #[async_trait]
        impl HttpClient for MockHttpClient {
            async fn execute(&self, _request: rest::Request) -> Result<Response, HttpError> {
                Ok(Response {
                    status: 200,
                    headers: HashMap::new(),
                    body: br#"{"code":"0","msg":"","data":[]}"#.to_vec().into(),
                })
            }
        }

        let http_client: Arc<dyn HttpClient> = Arc::new(MockHttpClient);
        let rate_limiter = RateLimiter::new();
        let rest_client = super::RestClient::new(
            credentials,
            "https://www.okx.com",
            http_client,
            rate_limiter,
        );

        // Verify the method exists and is properly typed
        let _ = super::RestClient::get_invitee_detail;
        let _ = &rest_client;

        println!("get_invitee_detail method is accessible and properly typed");
    }
}
