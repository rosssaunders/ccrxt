use serde::{Deserialize, Serialize};

use super::reset_mmp::IndexName;
use crate::deribit::{EndpointType, JsonRpcResult, PrivateRestClient, RestResult};

/// REST API endpoint constant
const GET_MMP_STATUS_ENDPOINT: &str = "private/get_mmp_status";

/// Request parameters for get MMP status
#[derive(Debug, Clone, Serialize)]
pub struct GetMmpStatusRequest {
    /// Index identifier of derivative instrument on the platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<IndexName>,

    /// Specifies the MMP group for which the status is being retrieved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_group: Option<String>,

    /// If true, retrieves MMP status for Block RFQ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq: Option<bool>,
}

/// MMP status object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MmpStatus {
    /// If true, indicates that the MMP status is for Block RFQ
    pub block_rfq: bool,

    /// Timestamp (milliseconds since the UNIX epoch) until the user will be frozen - 0 means that the user is frozen until manual reset
    pub frozen_until: i64,

    /// Index identifier, matches (base) cryptocurrency with quote currency
    pub index_name: String,

    /// Triggered mmp group, this parameter is optional (appears only for Mass Quote orders trigger)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_group: Option<String>,
}

/// Response for get MMP status endpoint
pub type GetMmpStatusResponse = JsonRpcResult<Vec<MmpStatus>>;

impl PrivateRestClient {
    /// Get MMP status
    ///
    /// Get MMP status for triggered index (or group). If the parameter is not provided,
    /// a list of all triggered MMP statuses is returned.
    /// This endpoint requires trade:read or block_rfq:read scope when block_rfq is true.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-get_mmp_status)
    ///
    /// Rate limit: Matching engine endpoint (tier-based limits)
    /// Scope: trade:read or block_rfq:read (when block_rfq = true)
    ///
    /// # Arguments
    /// * `index_name` - Optional index identifier of derivative instrument on the platform
    /// * `mmp_group` - Optional MMP group for which the status is being retrieved
    /// * `block_rfq` - Optional flag to retrieve MMP status for Block RFQ
    ///
    /// # Returns
    /// Result containing array of MMP status objects
    pub async fn get_mmp_status(
        &self,
        request: GetMmpStatusRequest,
    ) -> RestResult<GetMmpStatusResponse> {
        self.send_signed_request(
            GET_MMP_STATUS_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rest::secrets::SecretString;
    /// REST API endpoint constant
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::{AccountTier, credentials::Credentials};

    #[test]
    fn test_request_parameters_serialization_minimal() {
        let request = GetMmpStatusRequest {
            index_name: None,
            mmp_group: None,
            block_rfq: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        // When all parameters are None, they should be omitted from JSON
        assert!(json_value.get("index_name").is_none());
        assert!(json_value.get("mmp_group").is_none());
        assert!(json_value.get("block_rfq").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_with_index_name() {
        let request = GetMmpStatusRequest {
            index_name: Some(IndexName::BtcUsd),
            mmp_group: None,
            block_rfq: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("index_name").unwrap(), "btc_usd");
        assert!(json_value.get("mmp_group").is_none());
        assert!(json_value.get("block_rfq").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_full() {
        let request = GetMmpStatusRequest {
            index_name: Some(IndexName::EthUsdc),
            mmp_group: Some("group1".to_string()),
            block_rfq: Some(true),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("index_name").unwrap(), "eth_usdc");
        assert_eq!(json_value.get("mmp_group").unwrap(), "group1");
        assert_eq!(json_value.get("block_rfq").unwrap(), true);
    }

    #[test]
    fn test_request_parameters_serialization_with_all_index() {
        let request = GetMmpStatusRequest {
            index_name: Some(IndexName::All),
            mmp_group: Some("main".to_string()),
            block_rfq: Some(false),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("index_name").unwrap(), "all");
        assert_eq!(json_value.get("mmp_group").unwrap(), "main");
        assert_eq!(json_value.get("block_rfq").unwrap(), false);
    }

    #[test]
    fn test_mmp_status_deserialization() {
        let status_json = json!({
            "block_rfq": false,
            "frozen_until": 1234567890123i64,
            "index_name": "btc_usd",
            "mmp_group": "default"
        });

        let status: MmpStatus = serde_json::from_value(status_json).unwrap();

        assert!(!status.block_rfq);
        assert_eq!(status.frozen_until, 1234567890123i64);
        assert_eq!(status.index_name, "btc_usd");
        assert_eq!(status.mmp_group.unwrap(), "default");
    }

    #[test]
    fn test_mmp_status_deserialization_without_group() {
        let status_json = json!({
            "block_rfq": true,
            "frozen_until": 0,
            "index_name": "eth_usdc"
        });

        let status: MmpStatus = serde_json::from_value(status_json).unwrap();

        assert!(status.block_rfq);
        assert_eq!(status.frozen_until, 0);
        assert_eq!(status.index_name, "eth_usdc");
        assert!(status.mmp_group.is_none());
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": [
                {
                    "block_rfq": false,
                    "frozen_until": 1234567890123i64,
                    "index_name": "btc_usd",
                    "mmp_group": "default"
                },
                {
                    "block_rfq": true,
                    "frozen_until": 0,
                    "index_name": "eth_usdc"
                }
            ]
        });

        let response: GetMmpStatusResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 2);

        let first_status = &response.result[0];
        assert!(!first_status.block_rfq);
        assert_eq!(first_status.frozen_until, 1234567890123i64);
        assert_eq!(first_status.index_name, "btc_usd");
        assert_eq!(first_status.mmp_group.as_ref().unwrap(), "default");

        let second_status = &response.result[1];
        assert!(second_status.block_rfq);
        assert_eq!(second_status.frozen_until, 0);
        assert_eq!(second_status.index_name, "eth_usdc");
        assert!(second_status.mmp_group.is_none());
    }

    #[test]
    fn test_response_structures_deserialization_empty_result() {
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": []
        });

        let response: GetMmpStatusResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 0);
    }

    #[tokio::test]
    async fn test_get_mmp_status_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = PrivateRestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = PrivateRestClient::get_mmp_status;

        // Verify the client exists
        let _ = &rest_client;

        println!("get_mmp_status method is accessible and properly typed");
    }

    #[tokio::test]
    async fn test_get_mmp_status_endpoint_integration() {
        // This test demonstrates that the endpoint is properly integrated
        // and all types are accessible from the top-level module

        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = PrivateRestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can access all the types from the module
        let _index_name = IndexName::BtcUsd;
        let _index_name_all = IndexName::All;
        let _index_name_eth = IndexName::EthUsdc;

        // Test that we can create request structures
        let _request = GetMmpStatusRequest {
            index_name: Some(IndexName::BtcUsd),
            mmp_group: Some("test_group".to_string()),
            block_rfq: Some(true),
        };

        // Test that response type is accessible
        let _response_type = std::marker::PhantomData::<GetMmpStatusResponse>;
        let _status_type = std::marker::PhantomData::<MmpStatus>;

        // Test that the method exists and is accessible
        let _method_ref = PrivateRestClient::get_mmp_status;

        // Verify the client exists
        let _ = &rest_client;

        // Test that we can verify types are properly exported through module system
        use crate::deribit::private::rest::{
            GetMmpStatusRequest as ExportedRequest, GetMmpStatusResponse as ExportedResponse,
            MmpStatus as ExportedStatus,
        };
        let _exported_request = ExportedRequest {
            index_name: Some(IndexName::EthUsd),
            mmp_group: None,
            block_rfq: None,
        };
        let _exported_response_type = std::marker::PhantomData::<ExportedResponse>;
        let _exported_status_type = std::marker::PhantomData::<ExportedStatus>;

        println!("All get_mmp_status types and methods are properly integrated and accessible");
    }
}
