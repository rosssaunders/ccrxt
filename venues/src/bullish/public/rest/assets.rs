//! Assets endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Asset status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AssetStatus {
    Active,
    Inactive,
    Suspended,
}

/// Asset information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// Asset symbol
    pub symbol: String,
    /// Asset display name
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Asset description
    pub description: Option<String>,
    /// Asset status
    pub status: AssetStatus,
    /// Whether deposits are enabled
    #[serde(rename = "depositEnabled")]
    pub deposit_enabled: bool,
    /// Whether withdrawals are enabled
    #[serde(rename = "withdrawalEnabled")]
    pub withdrawal_enabled: bool,
    /// Whether trading is enabled
    #[serde(rename = "tradingEnabled")]
    pub trading_enabled: bool,
    /// Whether borrowing is enabled
    #[serde(rename = "borrowingEnabled")]
    pub borrowing_enabled: bool,
    /// Whether this asset can be used as collateral
    #[serde(rename = "collateralEnabled")]
    pub collateral_enabled: bool,
    /// Decimal precision for this asset
    pub precision: u8,
    /// Minimum deposit amount
    #[serde(rename = "minDeposit")]
    pub min_deposit: String,
    /// Minimum withdrawal amount
    #[serde(rename = "minWithdrawal")]
    pub min_withdrawal: String,
    /// Maximum withdrawal amount
    #[serde(rename = "maxWithdrawal")]
    pub max_withdrawal: String,
    /// Withdrawal fee
    #[serde(rename = "withdrawalFee")]
    pub withdrawal_fee: String,
    /// Network confirmations required for deposits
    #[serde(rename = "depositConfirmations")]
    pub deposit_confirmations: Option<u32>,
    /// Network information for crypto assets
    pub networks: Option<Vec<AssetNetwork>>,
}

/// Network information for crypto assets
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetNetwork {
    /// Network name (e.g., "Ethereum", "Bitcoin")
    pub network: String,
    /// Network display name
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Whether this network is enabled
    pub enabled: bool,
    /// Contract address (for tokens)
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
    /// Minimum deposit amount for this network
    #[serde(rename = "minDeposit")]
    pub min_deposit: String,
    /// Minimum withdrawal amount for this network
    #[serde(rename = "minWithdrawal")]
    pub min_withdrawal: String,
    /// Withdrawal fee for this network
    #[serde(rename = "withdrawalFee")]
    pub withdrawal_fee: String,
    /// Required confirmations for deposits
    #[serde(rename = "depositConfirmations")]
    pub deposit_confirmations: u32,
}

/// Response for assets query
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsResponse {
    /// List of assets
    pub data: Vec<Asset>,
}

/// Single asset response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleAssetResponse {
    /// Asset details
    pub data: Asset,
}

impl RestClient {
    /// Get all assets
    ///
    /// Retrieve information for all assets available on the exchange.
    ///
    /// # Returns
    /// List of all assets with their properties and trading parameters
    pub async fn get_assets(&self) -> RestResult<AssetsResponse> {
        self.send_request(
            "/v1/assets",
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PublicAssets,
        )
        .await
    }

    /// Get specific asset by symbol
    ///
    /// Retrieve detailed information for a specific asset.
    ///
    /// # Arguments
    /// * `symbol` - Asset symbol
    ///
    /// # Returns
    /// Detailed asset information including network details and trading parameters
    pub async fn get_asset(&self, symbol: &str) -> RestResult<SingleAssetResponse> {
        let url = format!("/v1/assets/{}", symbol);
        
        self.send_request(
            &url,
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PublicAssets,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_status_serialization() {
        assert_eq!(serde_json::to_string(&AssetStatus::Active).unwrap(), "\"ACTIVE\"");
        assert_eq!(serde_json::to_string(&AssetStatus::Inactive).unwrap(), "\"INACTIVE\"");
        assert_eq!(serde_json::to_string(&AssetStatus::Suspended).unwrap(), "\"SUSPENDED\"");
    }

    #[test]
    fn test_asset_deserialization() {
        let json = r#"{
            "symbol": "BTC",
            "displayName": "Bitcoin",
            "description": "Bitcoin cryptocurrency",
            "status": "ACTIVE",
            "depositEnabled": true,
            "withdrawalEnabled": true,
            "tradingEnabled": true,
            "borrowingEnabled": true,
            "collateralEnabled": true,
            "precision": 8,
            "minDeposit": "0.001",
            "minWithdrawal": "0.001",
            "maxWithdrawal": "100.0",
            "withdrawalFee": "0.0005",
            "depositConfirmations": 6,
            "networks": [
                {
                    "network": "Bitcoin",
                    "displayName": "Bitcoin Network",
                    "enabled": true,
                    "contractAddress": null,
                    "minDeposit": "0.001",
                    "minWithdrawal": "0.001",
                    "withdrawalFee": "0.0005",
                    "depositConfirmations": 6
                }
            ]
        }"#;

        let asset: Asset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.symbol, "BTC");
        assert_eq!(asset.display_name, "Bitcoin");
        assert_eq!(asset.status, AssetStatus::Active);
        assert!(asset.deposit_enabled);
        assert!(asset.trading_enabled);
        assert_eq!(asset.precision, 8);
        assert!(asset.networks.is_some());
        
        let networks = asset.networks.unwrap();
        assert_eq!(networks.len(), 1);
        assert_eq!(networks[0].network, "Bitcoin");
        assert_eq!(networks[0].deposit_confirmations, 6);
    }

    #[test]
    fn test_assets_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "symbol": "BTC",
                    "displayName": "Bitcoin",
                    "status": "ACTIVE",
                    "depositEnabled": true,
                    "withdrawalEnabled": true,
                    "tradingEnabled": true,
                    "borrowingEnabled": true,
                    "collateralEnabled": true,
                    "precision": 8,
                    "minDeposit": "0.001",
                    "minWithdrawal": "0.001",
                    "maxWithdrawal": "100.0",
                    "withdrawalFee": "0.0005"
                }
            ]
        }"#;

        let response: AssetsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].symbol, "BTC");
    }
}
