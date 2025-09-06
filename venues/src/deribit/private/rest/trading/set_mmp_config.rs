use serde::{Deserialize, Serialize};

use crate::deribit::{EndpointType, JsonRpcResult, PrivateRestClient, RestResult};

/// REST API endpoint constant
const SET_MMP_CONFIG_ENDPOINT: &str = "private/set_mmp_config";

/// Index name for MMP configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
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

/// Request parameters for set MMP config
#[derive(Debug, Clone, Serialize)]
pub struct SetMmpConfigRequest {
    /// Index identifier of derivative instrument on the platform
    pub index_name: IndexName,

    /// MMP Interval in seconds, if set to 0 MMP is removed
    pub interval: i32,

    /// MMP frozen time in seconds, if set to 0 manual reset is required
    pub frozen_time: i32,

    /// Designates the MMP group for which the configuration is being set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp_group: Option<String>,

    /// Quantity limit, positive value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_limit: Option<f64>,

    /// Delta limit, positive value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_limit: Option<f64>,

    /// Vega limit, positive value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vega_limit: Option<f64>,

    /// If true, configures MMP for Block RFQ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_rfq: Option<bool>,

    /// For Block RFQ only (block_rfq = true). Sets the maximum number of Block RFQ trades
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_count_limit: Option<i32>,
}

/// MMP configuration object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MmpConfig {
    /// If true, indicates MMP configuration for Block RFQ
    pub block_rfq: bool,

    /// Delta limit
    pub delta_limit: f64,

    /// MMP frozen time in seconds, if set to 0 manual reset is required
    pub frozen_time: i32,

    /// Index identifier, matches (base) cryptocurrency with quote currency
    pub index_name: String,

    /// MMP Interval in seconds, if set to 0 MMP is disabled
    pub interval: i32,

    /// Specified MMP Group
    pub mmp_group: String,

    /// Quantity limit
    pub quantity_limit: f64,

    /// For Block RFQ only. The maximum number of Block RFQ trades allowed in the lookback window
    pub trade_count_limit: i32,

    /// Vega limit
    pub vega_limit: f64,
}

/// Response for set MMP config endpoint
pub type SetMmpConfigResponse = JsonRpcResult<Vec<MmpConfig>>;

impl PrivateRestClient {
    /// Set config for MMP - triggers MMP reset
    ///
    /// Set config for MMP (Market Maker Protection) which triggers MMP reset.
    /// This endpoint requires trade:read_write or block_rfq:read_write scope (when block_rfq = true).
    /// This is a matching engine method.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-set_mmp_config)
    ///
    /// Rate limit: Matching engine endpoint (tier-based limits)
    /// Scope: trade:read_write or block_rfq:read_write (when block_rfq = true)
    ///
    /// # Arguments
    /// * `request` - The MMP configuration request parameters
    ///
    /// # Returns
    /// Result with array of MMP configuration objects
    pub async fn set_mmp_config(
        &self,
        request: SetMmpConfigRequest,
    ) -> RestResult<SetMmpConfigResponse> {
        self.send_signed_request(
            SET_MMP_CONFIG_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use secrets::SecretString;
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::{AccountTier, credentials::Credentials};

    #[test]
    fn test_index_name_serialization() {
        let btc_usd = IndexName::BtcUsd;
        let eth_usd = IndexName::EthUsd;
        let all = IndexName::All;

        let btc_json = serde_json::to_string(&btc_usd).unwrap();
        let eth_json = serde_json::to_string(&eth_usd).unwrap();
        let all_json = serde_json::to_string(&all).unwrap();

        assert_eq!(btc_json, "\"btc_usd\"");
        assert_eq!(eth_json, "\"eth_usd\"");
        assert_eq!(all_json, "\"all\"");
    }

    #[test]
    fn test_index_name_deserialization() {
        let btc_usd: IndexName = serde_json::from_str("\"btc_usd\"").unwrap();
        let eth_usd: IndexName = serde_json::from_str("\"eth_usd\"").unwrap();
        let all: IndexName = serde_json::from_str("\"all\"").unwrap();

        assert_eq!(btc_usd, IndexName::BtcUsd);
        assert_eq!(eth_usd, IndexName::EthUsd);
        assert_eq!(all, IndexName::All);
    }

    #[test]
    fn test_request_parameters_serialization_minimal() {
        let request = SetMmpConfigRequest {
            index_name: IndexName::BtcUsd,
            interval: 60,
            frozen_time: 30,
            mmp_group: None,
            quantity_limit: None,
            delta_limit: None,
            vega_limit: None,
            block_rfq: None,
            trade_count_limit: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("index_name").unwrap(), "btc_usd");
        assert_eq!(json_value.get("interval").unwrap(), 60);
        assert_eq!(json_value.get("frozen_time").unwrap(), 30);
        assert!(json_value.get("mmp_group").is_none());
        assert!(json_value.get("quantity_limit").is_none());
        assert!(json_value.get("delta_limit").is_none());
        assert!(json_value.get("vega_limit").is_none());
        assert!(json_value.get("block_rfq").is_none());
        assert!(json_value.get("trade_count_limit").is_none());
    }

    #[test]
    fn test_request_parameters_serialization_full() {
        let request = SetMmpConfigRequest {
            index_name: IndexName::EthUsdc,
            interval: 120,
            frozen_time: 60,
            mmp_group: Some("group1".to_string()),
            quantity_limit: Some(100.0),
            delta_limit: Some(50.0),
            vega_limit: Some(25.0),
            block_rfq: Some(true),
            trade_count_limit: Some(10),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("index_name").unwrap(), "eth_usdc");
        assert_eq!(json_value.get("interval").unwrap(), 120);
        assert_eq!(json_value.get("frozen_time").unwrap(), 60);
        assert_eq!(json_value.get("mmp_group").unwrap(), "group1");
        assert_eq!(json_value.get("quantity_limit").unwrap(), 100.0);
        assert_eq!(json_value.get("delta_limit").unwrap(), 50.0);
        assert_eq!(json_value.get("vega_limit").unwrap(), 25.0);
        assert!(
            json_value
                .get("block_rfq")
                .unwrap()
                .as_bool()
                .unwrap_or(false)
        );
        assert_eq!(json_value.get("trade_count_limit").unwrap(), 10);
    }

    #[test]
    fn test_request_parameters_serialization_block_rfq() {
        let request = SetMmpConfigRequest {
            index_name: IndexName::All,
            interval: 0,    // MMP disabled
            frozen_time: 0, // Manual reset required
            mmp_group: None,
            quantity_limit: None,
            delta_limit: None,
            vega_limit: None,
            block_rfq: Some(true),
            trade_count_limit: Some(5),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("index_name").unwrap(), "all");
        assert_eq!(json_value.get("interval").unwrap(), 0);
        assert_eq!(json_value.get("frozen_time").unwrap(), 0);
        assert!(
            json_value
                .get("block_rfq")
                .unwrap()
                .as_bool()
                .unwrap_or(false)
        );
        assert_eq!(json_value.get("trade_count_limit").unwrap(), 5);
    }

    #[test]
    fn test_response_structures_deserialization() {
        let response_json = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "result": [
                {
                    "block_rfq": false,
                    "delta_limit": 100.0,
                    "frozen_time": 30,
                    "index_name": "btc_usd",
                    "interval": 60,
                    "mmp_group": "default",
                    "quantity_limit": 1000.0,
                    "trade_count_limit": 0,
                    "vega_limit": 50.0
                }
            ]
        });

        let response: SetMmpConfigResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 1);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 1);

        let config = &response.result[0];
        assert!(!config.block_rfq);
        assert_eq!(config.delta_limit, 100.0);
        assert_eq!(config.frozen_time, 30);
        assert_eq!(config.index_name, "btc_usd");
        assert_eq!(config.interval, 60);
        assert_eq!(config.mmp_group, "default");
        assert_eq!(config.quantity_limit, 1000.0);
        assert_eq!(config.trade_count_limit, 0);
        assert_eq!(config.vega_limit, 50.0);
    }

    #[test]
    fn test_response_structures_deserialization_multiple_configs() {
        let response_json = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "result": [
                {
                    "block_rfq": false,
                    "delta_limit": 100.0,
                    "frozen_time": 30,
                    "index_name": "btc_usd",
                    "interval": 60,
                    "mmp_group": "group1",
                    "quantity_limit": 1000.0,
                    "trade_count_limit": 0,
                    "vega_limit": 50.0
                },
                {
                    "block_rfq": true,
                    "delta_limit": 0.0,
                    "frozen_time": 0,
                    "index_name": "all",
                    "interval": 0,
                    "mmp_group": "block_rfq_group",
                    "quantity_limit": 0.0,
                    "trade_count_limit": 10,
                    "vega_limit": 0.0
                }
            ]
        });

        let response: SetMmpConfigResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 2);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.len(), 2);

        let normal_config = &response.result[0];
        assert!(!normal_config.block_rfq);
        assert_eq!(normal_config.index_name, "btc_usd");
        assert_eq!(normal_config.mmp_group, "group1");

        let block_rfq_config = &response.result[1];
        assert!(block_rfq_config.block_rfq);
        assert_eq!(block_rfq_config.index_name, "all");
        assert_eq!(block_rfq_config.trade_count_limit, 10);
    }

    #[test]
    fn test_all_index_names() {
        // Test that all index names serialize correctly
        let index_names = vec![
            (IndexName::BtcUsd, "btc_usd"),
            (IndexName::EthUsd, "eth_usd"),
            (IndexName::BtcUsdc, "btc_usdc"),
            (IndexName::EthUsdc, "eth_usdc"),
            (IndexName::AdaUsdc, "ada_usdc"),
            (IndexName::AlgoUsdc, "algo_usdc"),
            (IndexName::AvaxUsdc, "avax_usdc"),
            (IndexName::BchUsdc, "bch_usdc"),
            (IndexName::BnbUsdc, "bnb_usdc"),
            (IndexName::DogeUsdc, "doge_usdc"),
            (IndexName::DotUsdc, "dot_usdc"),
            (IndexName::LinkUsdc, "link_usdc"),
            (IndexName::LtcUsdc, "ltc_usdc"),
            (IndexName::NearUsdc, "near_usdc"),
            (IndexName::PaxgUsdc, "paxg_usdc"),
            (IndexName::ShibUsdc, "shib_usdc"),
            (IndexName::SolUsdc, "sol_usdc"),
            (IndexName::TrxUsdc, "trx_usdc"),
            (IndexName::TrumpUsdc, "trump_usdc"),
            (IndexName::UniUsdc, "uni_usdc"),
            (IndexName::XrpUsdc, "xrp_usdc"),
            (IndexName::UsdeUsdc, "usde_usdc"),
            (IndexName::BuidlUsdc, "buidl_usdc"),
            (IndexName::BtcdvolUsdc, "btcdvol_usdc"),
            (IndexName::EthdvolUsdc, "ethdvol_usdc"),
            (IndexName::BtcUsdt, "btc_usdt"),
            (IndexName::EthUsdt, "eth_usdt"),
            (IndexName::All, "all"),
        ];

        for (index_name, expected_str) in index_names {
            let serialized = serde_json::to_string(&index_name).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_str));

            let deserialized: IndexName = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, index_name);
        }
    }

    #[tokio::test]
    async fn test_set_mmp_config_method_exists() {
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
        let _ = PrivateRestClient::set_mmp_config;

        // Verify the client exists
        let _ = &rest_client;

        println!("set_mmp_config method is accessible and properly typed");
    }
}
