use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

use crate::bitget::{ApiError, PublicRestClient as RestClient, RestResponse};

/// Endpoint for getting coin information
const COIN_INFO_ENDPOINT: &str = "/api/v2/spot/public/coins";

/// Custom deserializer for boolean fields that may come as strings
fn deserialize_string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        other => other.parse().map_err(serde::de::Error::custom),
    }
}

/// Request for getting coin information
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCoinInfoRequest {
    /// Specific coin to query, if empty returns all coins
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

/// Chain information for a coin
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainInfo {
    /// Chain name
    pub chain: String,

    /// Whether tag is needed
    #[serde(deserialize_with = "deserialize_string_to_bool")]
    pub need_tag: bool,

    /// Whether withdrawal is supported
    #[serde(deserialize_with = "deserialize_string_to_bool")]
    pub withdrawable: bool,

    /// Whether deposit is supported
    #[serde(deserialize_with = "deserialize_string_to_bool")]
    pub rechargeable: bool,
    /// Withdrawal transaction fee
    pub withdraw_fee: String,
    /// Extra withdrawal fee
    pub extra_withdraw_fee: Option<String>,
    /// Deposit confirmation blocks
    pub deposit_confirm: String,
    /// Withdrawal confirmation blocks
    pub withdraw_confirm: String,
    /// Minimum deposit amount
    pub min_deposit_amount: String,
    /// Minimum withdrawal amount
    pub min_withdraw_amount: String,
    /// Blockchain explorer URL
    pub browser_url: Option<String>,
    /// Contract address
    pub contract_address: Option<String>,
    /// Withdrawal step
    pub withdraw_step: String,
    /// Withdrawal decimal places
    pub withdraw_min_scale: String,
    /// Network congestion status
    pub congestion: Option<String>,
}

/// Coin information
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    /// Currency ID
    pub coin_id: String,

    /// Token name
    pub coin: String,

    /// Transferability
    #[serde(deserialize_with = "deserialize_string_to_bool")]
    pub transfer: bool,

    /// Supported chain list
    pub chains: Vec<ChainInfo>,
}

/// Response for getting coin information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetCoinInfoResponse {
    /// List of coin information
    pub data: Vec<CoinInfo>,
}

impl RestClient {
    /// Get coin information
    ///
    /// Returns information about supported coins.
    ///
    /// [docs](https://www.bitget.com/api-doc/spot/public/Get-Coin-Info)
    ///
    /// Rate limit: see official docs
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// The coin information
    pub async fn get_coin_info(
        &self,
        request: &GetCoinInfoRequest,
    ) -> Result<RestResponse<Vec<CoinInfo>>, ApiError> {
        let endpoint = COIN_INFO_ENDPOINT;

        let mut params = HashMap::new();
        if let Some(coin) = &request.coin {
            params.insert("coin".to_string(), coin.clone());
        }

        let params = if params.is_empty() {
            None
        } else {
            Some(params)
        };

        self.get(endpoint, params).await
    }
}
