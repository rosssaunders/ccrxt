use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const RESET_MMP_ENDPOINT: &str = "private/reset_mmp";

/// Index names supported by Deribit for MMP operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IndexName {
    #[serde(rename = "btc_usd")]
    BtcUsd,
    #[serde(rename = "eth_usd")]
    EthUsd,
    #[serde(rename = "btc_usdc")]
    BtcUsdc,
    #[serde(rename = "eth_usdc")]
    EthUsdc,
    #[serde(rename = "ada_usdc")]
    AdaUsdc,
    #[serde(rename = "algo_usdc")]
    AlgoUsdc,
    #[serde(rename = "avax_usdc")]
    AvaxUsdc,
    #[serde(rename = "bch_usdc")]
    BchUsdc,
    #[serde(rename = "bnb_usdc")]
    BnbUsdc,
    #[serde(rename = "doge_usdc")]
    DogeUsdc,
    #[serde(rename = "dot_usdc")]
    DotUsdc,
    #[serde(rename = "link_usdc")]
    LinkUsdc,
    #[serde(rename = "ltc_usdc")]
    LtcUsdc,
    #[serde(rename = "near_usdc")]
    NearUsdc,
    #[serde(rename = "paxg_usdc")]
    PaxgUsdc,
    #[serde(rename = "shib_usdc")]
    ShibUsdc,
    #[serde(rename = "sol_usdc")]
    SolUsdc,
    #[serde(rename = "trx_usdc")]
    TrxUsdc,
    #[serde(rename = "trump_usdc")]
    TrumpUsdc,
    #[serde(rename = "uni_usdc")]
    UniUsdc,
    #[serde(rename = "xrp_usdc")]
    XrpUsdc,
    #[serde(rename = "usde_usdc")]
    UsdeUsdc,
    #[serde(rename = "buidl_usdc")]
    BuidlUsdc,
    #[serde(rename = "btcdvol_usdc")]
    BtcdvolUsdc,
    #[serde(rename = "ethdvol_usdc")]
    EthdvolUsdc,
    #[serde(rename = "btc_usdt")]
    BtcUsdt,
    #[serde(rename = "eth_usdt")]
    EthUsdt,
    #[serde(rename = "all")]
    All,
}

/// Request parameters for reset MMP
#[derive(Debug, Clone, Serialize)]
pub struct ResetMmpRequest {
    /// Index identifier of derivative instrument on the platform
    pub index_name: IndexName,

    /// Specifies the MMP group for which limits are being reset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_group: Option<String>,

    /// If true, resets MMP for Block RFQ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq: Option<bool>,
}

/// Response for reset MMP endpoint
pub type ResetMmpResponse = JsonRpcResult<String>;

impl RestClient {
    /// Reset MMP (Market Maker Protection) for the specified index and optional MMP group.
    /// This endpoint requires trade:read_write or block_rfq:read_write scope when block_rfq is true.
    ///
    /// [docs]: https://docs.deribit.com/v2/#private-reset_mmp
    ///
    /// Rate limit: Matching engine endpoint (tier-based limits)
    /// Scope: trade:read_write or block_rfq:read_write (when block_rfq = true)
    ///
    /// # Arguments
    /// * `params` - Parameters for the reset MMP request
    ///
    /// # Returns
    /// Result containing "ok" string on success
    pub async fn reset_mmp(&self, params: ResetMmpRequest) -> RestResult<ResetMmpResponse> {
        self.send_signed_request(RESET_MMP_ENDPOINT, &params, EndpointType::MatchingEngine)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
use serde_json::{Value, json};

    use super::*;
    use crate::deribit::AccountTier;

    
    use crate::deribit::private::rest::credentials::Credentials;
    use rest::secrets::SecretString;


    #[test]
    fn test_index_name_serialization() {
        assert_eq!(
            serde_json::to_string(&IndexName::BtcUsd).unwrap(),
            "\"btc_usd\""
        );
        assert_eq!(
            serde_json::to_string(&IndexName::EthUsd).unwrap(),
            "\"eth_usd\""
        );
        assert_eq!(serde_json::to_string(&IndexName::All).unwrap(), "\"all\"");
        assert_eq!(
            serde_json::to_string(&IndexName::BtcUsdc).unwrap(),
            "\"btc_usdc\""
        );
        assert_eq!(
            serde_json::to_string(&IndexName::TrumpUsdc).unwrap(),
            "\"trump_usdc\""
        );
    }

    #[test]
    fn test_index_name_deserialization() {
        let btc_usd: IndexName = serde_json::from_str("\"btc_usd\"").unwrap();
        assert_eq!(btc_usd, IndexName::BtcUsd);

        let eth_usd: IndexName = serde_json::from_str("\"eth_usd\"").unwrap();
        assert_eq!(eth_usd, IndexName::EthUsd);

        let all: IndexName = serde_json::from_str("\"all\"").unwrap();
        assert_eq!(all, IndexName::All);
    }

    #[test]
    fn test_request_parameters_serialization_minimal() {
        let request = ResetMmpRequest {
            index_name: IndexName::BtcUsd,
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
        let request = ResetMmpRequest {
            index_name: IndexName::EthUsdc,
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
        let request = ResetMmpRequest {
            index_name: IndexName::All,
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
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": "ok"
        });

        let response: ResetMmpResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, "ok");
    }

    #[tokio::test]
    async fn test_reset_mmp_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::reset_mmp;

        // Verify the client exists
        let _ = &rest_client;

        println!("reset_mmp method is accessible and properly typed");
    }

    #[test]
    fn test_index_name_comprehensive_coverage() {
        // Test that all documented index names are supported
        let index_names = vec![
            IndexName::BtcUsd,
            IndexName::EthUsd,
            IndexName::BtcUsdc,
            IndexName::EthUsdc,
            IndexName::AdaUsdc,
            IndexName::AlgoUsdc,
            IndexName::AvaxUsdc,
            IndexName::BchUsdc,
            IndexName::BnbUsdc,
            IndexName::DogeUsdc,
            IndexName::DotUsdc,
            IndexName::LinkUsdc,
            IndexName::LtcUsdc,
            IndexName::NearUsdc,
            IndexName::PaxgUsdc,
            IndexName::ShibUsdc,
            IndexName::SolUsdc,
            IndexName::TrxUsdc,
            IndexName::TrumpUsdc,
            IndexName::UniUsdc,
            IndexName::XrpUsdc,
            IndexName::UsdeUsdc,
            IndexName::BuidlUsdc,
            IndexName::BtcdvolUsdc,
            IndexName::EthdvolUsdc,
            IndexName::BtcUsdt,
            IndexName::EthUsdt,
            IndexName::All,
        ];

        // Test serialization/deserialization for all variants
        for index_name in index_names {
            let serialized = serde_json::to_string(&index_name).unwrap();
            let deserialized: IndexName = serde_json::from_str(&serialized).unwrap();
            assert_eq!(index_name, deserialized);
        }

        println!("All {} index names are properly supported", 28);
    }

    #[tokio::test]
    async fn test_reset_mmp_endpoint_integration() {
        // This test demonstrates that the endpoint is properly integrated
        // and all types are accessible from the top-level module

        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
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
        let _request = ResetMmpRequest {
            index_name: IndexName::BtcUsd,
            mmp_group: Some("test_group".to_string()),
            block_rfq: Some(true),
        };

        // Test that response type is accessible
        let _response_type = std::marker::PhantomData::<ResetMmpResponse>;

        // Test that the method exists and is accessible
        let _method_ref = RestClient::reset_mmp;

        // Verify the client exists
        let _ = &rest_client;

        println!("All reset_mmp types and methods are properly integrated and accessible");
    }
}

// The following endpoints will be refactored in their respective files to ensure all RestClient methods take a single request struct as parameter, and all send_signed_request calls include endpoint_type, per project conventions.
// No further changes needed in this file for reset_mmp.
