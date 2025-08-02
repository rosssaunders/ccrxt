use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::spot::{AssetType, RestResult};

/// Endpoint for getting account assets
const GET_ACCOUNT_ASSETS_ENDPOINT: &str = "/api/v2/spot/account/assets";

/// Request parameters for getting account assets
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetAccountAssetsRequest {
    /// Token name, e.g. USDT. Used for querying positions of a single coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,

    /// Asset type filter
    /// - `hold_only`: Position coin (assets that have holdings)
    /// - `all`: All coins including zero balances
    ///   Default is `hold_only`
    #[serde(rename = "assetType", skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetType>,
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
#[derive(Debug, Clone, Deserialize)]
pub struct GetAccountAssetsResponse {
    /// List of account assets
    pub assets: Vec<AssetInfo>,
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
    pub async fn get_account_assets(
        &self,
        request: GetAccountAssetsRequest,
    ) -> RestResult<GetAccountAssetsResponse> {
        self.send_get_signed_request(
            GET_ACCOUNT_ASSETS_ENDPOINT,
            request,
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
        let request = GetAccountAssetsRequest::default();
        assert!(request.coin.is_none());
        assert!(request.asset_type.is_none());
    }

    #[test]
    fn test_get_account_assets_request_for_coin() {
        let request = GetAccountAssetsRequest {
            coin: Some("USDT".to_string()),
            ..Default::default()
        };
        assert_eq!(request.coin, Some("USDT".to_string()));
        assert!(request.asset_type.is_none());
    }

    #[test]
    fn test_get_account_assets_request_all_assets() {
        let request = GetAccountAssetsRequest::default();
        assert!(request.coin.is_none());
        assert!(request.asset_type.is_none()); // Default should be None, not Some(AssetType::All)
    }

    #[test]
    fn test_get_account_assets_request_builder() {
        let request = GetAccountAssetsRequest {
            coin: Some("BTC".to_string()),
            asset_type: Some(AssetType::All),
        };
        assert_eq!(request.coin, Some("BTC".to_string()));
        assert_eq!(request.asset_type, Some(AssetType::All));
    }

    #[test]
    fn test_get_account_assets_request_serialization() {
        let request = GetAccountAssetsRequest {
            coin: Some("USDT".to_string()),
            asset_type: Some(AssetType::All),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap_or_else(|e| {
            eprintln!("Serialization failed: {}", e);
            String::new()
        });
        // Should contain both parameters
        assert!(serialized.contains("coin=USDT"));
        assert!(serialized.contains("assetType=all"));
    }

    #[test]
    fn test_get_account_assets_request_serialization_empty() {
        let request = GetAccountAssetsRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap_or_else(|e| {
            eprintln!("Serialization failed: {}", e);
            String::new()
        });

        // Should be empty since both fields are None and skipped
        assert!(serialized.is_empty());
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

        let asset: AssetInfo = match serde_json::from_str(json) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Deserialization failed: {}", e);
                return;
            }
        };

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

        let response: GetAccountAssetsResponse = match serde_json::from_str(json) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Deserialization failed: {}", e);
                return;
            }
        };

        assert_eq!(response.assets.len(), 2);
        assert_eq!(
            response.assets.first().map(|a| &a.coin),
            Some(&"USDT".to_string())
        );
        assert_eq!(
            response.assets.get(1).map(|a| &a.coin),
            Some(&"BTC".to_string())
        );
        assert_eq!(
            response.assets.get(1).map(|a| &a.frozen),
            Some(&"0.1".to_string())
        );
    }
}
