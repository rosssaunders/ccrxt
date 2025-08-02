//! Account asset balances endpoint for Bullish Exchange API

use serde::Deserialize;

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL path for asset balances
const ASSET_BALANCES_ENDPOINT: &str = "/v1/accounts/asset";

/// Endpoint URL path for single asset balance (with parameter)
const SINGLE_ASSET_BALANCE_ENDPOINT: &str = "/v1/accounts/asset/{}";

/// Asset balance information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetBalance {
    /// Asset symbol
    pub symbol: String,
    /// Total balance
    pub balance: String,
    /// Available balance (not locked in orders)
    #[serde(rename = "availableBalance")]
    pub available_balance: String,
    /// Balance locked in open orders
    #[serde(rename = "lockedBalance")]
    pub locked_balance: String,
    /// Borrowed amount
    #[serde(rename = "borrowedBalance")]
    pub borrowed_balance: String,
    /// Interest owed on borrowed amount
    #[serde(rename = "interestOwed")]
    pub interest_owed: String,
    /// Net balance (balance - borrowed - interest)
    #[serde(rename = "netBalance")]
    pub net_balance: String,
    /// USD value of the balance
    #[serde(rename = "usdValue")]
    pub usd_value: String,
    /// Whether this asset can be borrowed
    #[serde(rename = "canBorrow")]
    pub can_borrow: bool,
    /// Whether this asset can be used as collateral
    #[serde(rename = "canCollateralize")]
    pub can_collateralize: bool,
    /// Collateral factor for this asset
    #[serde(rename = "collateralFactor")]
    pub collateral_factor: String,
    /// Maximum borrowing limit for this asset
    #[serde(rename = "borrowLimit")]
    pub borrow_limit: String,
}

/// Response for asset balances query
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetBalancesResponse {
    /// List of asset balances
    pub data: Vec<AssetBalance>,
}

/// Single asset balance response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleAssetBalanceResponse {
    /// Asset balance details
    pub data: AssetBalance,
}

impl RestClient {
    /// Get all asset balances
    ///
    /// Retrieve balance information for all assets in a trading account.
    ///
    /// # Arguments
    /// * `trading_account_id` - Trading account ID
    ///
    /// # Returns
    /// List of all asset balances
    pub async fn get_asset_balances(
        &mut self,
        trading_account_id: &str,
    ) -> RestResult<AssetBalancesResponse> {
        let url = format!(
            "{}?tradingAccountId={}",
            ASSET_BALANCES_ENDPOINT, trading_account_id
        );

        self.send_get_authenticated_request(&url, (), EndpointType::PrivateAssetBalances)
            .await
    }

    /// Get balance for a specific asset
    ///
    /// Retrieve balance information for a specific asset in a trading account.
    ///
    /// # Arguments
    /// * `symbol` - Asset symbol
    /// * `trading_account_id` - Trading account ID
    ///
    /// # Returns
    /// Balance information for the specified asset
    pub async fn get_asset_balance(
        &mut self,
        symbol: &str,
        trading_account_id: &str,
    ) -> RestResult<SingleAssetBalanceResponse> {
        let url = format!(
            "{}?tradingAccountId={}",
            SINGLE_ASSET_BALANCE_ENDPOINT.replace("{}", symbol),
            trading_account_id
        );

        self.send_get_authenticated_request(&url, (), EndpointType::PrivateAssetBalances)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_balance_deserialization() {
        let json = r#"{
            "symbol": "BTC",
            "balance": "1.0",
            "availableBalance": "0.8",
            "lockedBalance": "0.2",
            "borrowedBalance": "0.0",
            "interestOwed": "0.0",
            "netBalance": "1.0",
            "usdValue": "30000.0",
            "canBorrow": true,
            "canCollateralize": true,
            "collateralFactor": "0.8",
            "borrowLimit": "10.0"
        }"#;

        let balance: AssetBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.symbol, "BTC");
        assert_eq!(balance.balance, "1.0");
        assert_eq!(balance.available_balance, "0.8");
        assert_eq!(balance.locked_balance, "0.2");
        assert!(balance.can_borrow);
        assert!(balance.can_collateralize);
    }

    #[test]
    fn test_asset_balances_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "symbol": "BTC",
                    "balance": "1.0",
                    "availableBalance": "1.0",
                    "lockedBalance": "0.0",
                    "borrowedBalance": "0.0",
                    "interestOwed": "0.0",
                    "netBalance": "1.0",
                    "usdValue": "30000.0",
                    "canBorrow": true,
                    "canCollateralize": true,
                    "collateralFactor": "0.8",
                    "borrowLimit": "10.0"
                }
            ]
        }"#;

        let response: AssetBalancesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].symbol, "BTC");
    }
}
