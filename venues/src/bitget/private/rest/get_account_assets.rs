//! Get Account Assets endpoint for Bitget Spot API
//!
//! This endpoint allows retrieving account balance information for spot trading.
//!
//! Reference: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitget/spot_api.md
//! Endpoint: GET /api/v2/spot/account/assets
//! Rate limit: 10 times/1s (User ID)

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::{AssetType, RestResult};

/// Request parameters for getting account assets
#[derive(Debug, Clone, Serialize)]
pub struct GetAccountAssetsRequest {
    /// Token name, e.g. USDT. Used for querying positions of a single coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,

    /// Asset type filter
    /// - `hold_only`: Position coin (assets that have holdings)
    /// - `all`: All coins including zero balances
    /// Default is `hold_only`
    #[serde(rename = "assetType", skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetType>,
}

impl GetAccountAssetsRequest {
    /// Create a new request to get all assets with holdings
    pub fn new() -> Self {
        Self {
            coin: None,
            asset_type: None,
        }
    }

    /// Create a request for a specific coin
    pub fn for_coin(coin: impl Into<String>) -> Self {
        Self {
            coin: Some(coin.into()),
            asset_type: None,
        }
    }

    /// Create a request for all coins (including zero balances)
    pub fn all_assets() -> Self {
        Self {
            coin: None,
            asset_type: Some(AssetType::All),
        }
    }

    /// Set the coin to query
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }

    /// Set the asset type filter
    pub fn asset_type(mut self, asset_type: AssetType) -> Self {
        self.asset_type = Some(asset_type);
        self
    }
}

impl Default for GetAccountAssetsRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual asset information in the account
#[derive(Debug, Clone, Deserialize)]
pub struct AssetInfo {
    /// Token name
    pub coin: String,

    /// Available assets
    pub available: String,

    /// Amount of frozen assets
    /// Usually frozen when the limit order is placed or join the Launchpad
    pub frozen: String,

    /// Amount of locked assets
    /// Locked assets required to become a fiat merchants, etc.
    pub locked: String,

    /// Restricted availability
    /// For spot copy trading
    #[serde(rename = "limitAvailable")]
    pub limit_available: String,

    /// Update time (milliseconds)
    #[serde(rename = "uTime")]
    pub update_time: String,
}

/// Response from the get account assets endpoint
#[derive(Debug, Clone)]
pub struct GetAccountAssetsResponse {
    /// List of account assets
    pub assets: Vec<AssetInfo>,
}

// Implementation for direct response deserialization when the API returns an array
impl<'de> Deserialize<'de> for GetAccountAssetsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let assets = Vec::<AssetInfo>::deserialize(deserializer)?;
        Ok(GetAccountAssetsResponse { assets })
    }
}

impl RestClient {
    /// Get account assets for spot trading
    ///
    /// Retrieves balance information for the authenticated account.
    /// Can filter by specific coin or asset type.
    ///
    /// # Arguments
    /// * `request` - The request parameters specifying filters
    ///
    /// # Rate Limit
    /// 10 requests per second per UID
    ///
    /// # Returns
    /// A result containing the account assets or an error
    ///
    /// # Example
    /// ```rust,no_run
    /// use venues::bitget::{PrivateRestClient, GetAccountAssetsRequest};
    ///
    /// async fn example(client: &PrivateRestClient) -> Result<(), Box<dyn std::error::Error>> {
    ///     // Get all assets with holdings
    ///     let assets = client.get_account_assets(GetAccountAssetsRequest::new()).await?;
    ///     
    ///     // Get specific coin balance
    ///     let usdt_balance = client.get_account_assets(
    ///         GetAccountAssetsRequest::for_coin("USDT")
    ///     ).await?;
    ///     
    ///     // Get all assets including zero balances
    ///     let all_assets = client.get_account_assets(
    ///         GetAccountAssetsRequest::all_assets()
    ///     ).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_account_assets(&self, request: GetAccountAssetsRequest) -> RestResult<GetAccountAssetsResponse> {
        let query_string = if request.coin.is_some() || request.asset_type.is_some() {
            Some(serde_urlencoded::to_string(&request).map_err(|e| crate::bitget::Errors::Error(format!("Failed to encode query: {e}")))?)
        } else {
            None
        };

        self.send_signed_request(
            "/api/v2/spot/account/assets",
            reqwest::Method::GET,
            query_string.as_deref(),
            None,  // No body for GET request
            10,    // 10 requests per second rate limit
            false, // Not an order endpoint
            None,  // No order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_account_assets_request_default() {
        let request = GetAccountAssetsRequest::new();
        assert!(request.coin.is_none());
        assert!(request.asset_type.is_none());
    }

    #[test]
    fn test_get_account_assets_request_for_coin() {
        let request = GetAccountAssetsRequest::for_coin("USDT");
        assert_eq!(request.coin, Some("USDT".to_string()));
        assert!(request.asset_type.is_none());
    }

    #[test]
    fn test_get_account_assets_request_all_assets() {
        let request = GetAccountAssetsRequest::all_assets();
        assert!(request.coin.is_none());
        assert_eq!(request.asset_type, Some(AssetType::All));
    }

    #[test]
    fn test_get_account_assets_request_builder() {
        let request = GetAccountAssetsRequest::new()
            .coin("BTC")
            .asset_type(AssetType::All);

        assert_eq!(request.coin, Some("BTC".to_string()));
        assert_eq!(request.asset_type, Some(AssetType::All));
    }

    #[test]
    fn test_get_account_assets_request_serialization() {
        let request = GetAccountAssetsRequest::new()
            .coin("USDT")
            .asset_type(AssetType::All);

        let serialized = serde_urlencoded::to_string(&request).expect("Serialization failed");

        // Should contain both parameters
        assert!(serialized.contains("coin=USDT"));
        assert!(serialized.contains("assetType=all"));
    }

    #[test]
    fn test_get_account_assets_request_serialization_empty() {
        let request = GetAccountAssetsRequest::new();
        let serialized = serde_urlencoded::to_string(&request).expect("Serialization failed");

        // Should be empty since both fields are None and skipped
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_asset_info_deserialization() {
        let json = r#"{
            "coin": "USDT",
            "available": "1000.50",
            "frozen": "0",
            "locked": "0",
            "limitAvailable": "1000.50",
            "uTime": "1622697148000"
        }"#;

        let asset: AssetInfo = serde_json::from_str(json).expect("Deserialization failed");

        assert_eq!(asset.coin, "USDT");
        assert_eq!(asset.available, "1000.50");
        assert_eq!(asset.frozen, "0");
        assert_eq!(asset.locked, "0");
        assert_eq!(asset.limit_available, "1000.50");
        assert_eq!(asset.update_time, "1622697148000");
    }

    #[test]
    fn test_get_account_assets_response_deserialization() {
        let json = r#"[
            {
                "coin": "USDT",
                "available": "1000.50",
                "frozen": "0",
                "locked": "0",
                "limitAvailable": "1000.50",
                "uTime": "1622697148000"
            },
            {
                "coin": "BTC",
                "available": "0.5",
                "frozen": "0.1",
                "locked": "0",
                "limitAvailable": "0.5",
                "uTime": "1622697148000"
            }
        ]"#;

        let response: GetAccountAssetsResponse = serde_json::from_str(json).expect("Deserialization failed");

        assert_eq!(response.assets.len(), 2);
        assert_eq!(response.assets.get(0).map(|a| &a.coin), Some(&"USDT".to_string()));
        assert_eq!(response.assets.get(1).map(|a| &a.coin), Some(&"BTC".to_string()));
        assert_eq!(response.assets.get(1).map(|a| &a.frozen), Some(&"0.1".to_string()));
    }
}
