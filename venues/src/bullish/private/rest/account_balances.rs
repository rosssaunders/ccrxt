//! Account asset endpoints for Bullish Exchange API

use serde::{Deserialize, Serialize};

use crate::bullish::{EndpointType, PrivateRestClient as RestClient, RestResult};

/// Endpoint URL path for asset accounts
const ASSET_ACCOUNTS_ENDPOINT: &str = "/v1/accounts/asset";

/// Endpoint URL path for single asset account (with parameter)
const SINGLE_ASSET_ACCOUNT_ENDPOINT: &str = "/v1/accounts/asset/{}";

/// Query params for asset account endpoints
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct TradingAccountIdParam {
    #[serde(rename = "tradingAccountId")]
    trading_account_id: String,
}

/// Asset account information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetAccount {
    /// unique trading account ID
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,

    /// unique asset ID
    #[serde(rename = "assetId")]
    pub asset_id: String,

    /// asset symbol as denoted in the world
    #[serde(rename = "assetSymbol")]
    pub asset_symbol: String,

    /// the assets that are available to use on the account
    #[serde(rename = "availableQuantity")]
    pub available_quantity: String,

    /// the assets on the account that are borrowed
    #[serde(rename = "borrowedQuantity")]
    pub borrowed_quantity: String,

    /// the assets on the account that are locked in orders, loans and AMM instructions
    #[serde(rename = "lockedQuantity")]
    pub locked_quantity: String,

    /// the assets on the account that are being loaned
    #[serde(rename = "loanedQuantity")]
    pub loaned_quantity: String,

    /// ISO 8601 with millisecond as string
    #[serde(rename = "updatedAtDatetime")]
    pub updated_at_datetime: String,

    /// unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string
    #[serde(rename = "updatedAtTimestamp")]
    pub updated_at_timestamp: String,
}

impl RestClient {
    /// Get asset accounts
    ///
    /// Gets the asset accounts for a trading account.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/accounts/asset)
    ///
    /// # Arguments
    /// * `trading_account_id` - Id of the trading account
    ///
    /// # Returns
    /// Vector of asset accounts
    pub async fn get_asset_accounts(
        &mut self,
        trading_account_id: &str,
    ) -> RestResult<Vec<AssetAccount>> {
        let params = TradingAccountIdParam {
            trading_account_id: trading_account_id.to_string(),
        };

        self.send_get_authenticated_request(
            ASSET_ACCOUNTS_ENDPOINT,
            params,
            EndpointType::PrivateAssetBalances,
        )
        .await
    }

    /// Get asset account by symbol
    ///
    /// Gets the asset account for a specific symbol.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/accounts/asset/-symbol-)
    /// # Arguments
    /// * `symbol` - Asset symbol (e.g., BTC)
    /// * `trading_account_id` - Id of the trading account
    ///
    /// # Returns
    /// Asset account for the symbol
    pub async fn get_asset_account_by_symbol(
        &mut self,
        symbol: &str,
        trading_account_id: &str,
    ) -> RestResult<AssetAccount> {
        let endpoint = SINGLE_ASSET_ACCOUNT_ENDPOINT.replace("{}", symbol);
        let params = TradingAccountIdParam {
            trading_account_id: trading_account_id.to_string(),
        };

        self.send_get_authenticated_request(&endpoint, params, EndpointType::PrivateAssetBalances)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_account_deserialization() {
        let json = r#"{
            "tradingAccountId": "111000000000001",
            "assetId": "1",
            "assetSymbol": "BTC",
            "availableQuantity": "1.00000000",
            "borrowedQuantity": "0.00000000",
            "lockedQuantity": "0.00000000",
            "loanedQuantity": "0.00000000",
            "updatedAtDatetime": "2025-05-20T01:01:01.000Z",
            "updatedAtTimestamp": "1621490985000"
        }"#;

        let account: AssetAccount = serde_json::from_str(json).unwrap();
        assert_eq!(account.trading_account_id, "111000000000001");
        assert_eq!(account.asset_symbol, "BTC");
        assert_eq!(account.available_quantity, "1.00000000");
        assert_eq!(account.locked_quantity, "0.00000000");
    }

    #[test]
    fn test_asset_accounts_array_deserialization() {
        let json = r#"[
            {
                "tradingAccountId": "111000000000001",
                "assetId": "1",
                "assetSymbol": "BTC",
                "availableQuantity": "1.00000000",
                "borrowedQuantity": "0.00000000",
                "lockedQuantity": "0.00000000",
                "loanedQuantity": "0.00000000",
                "updatedAtDatetime": "2025-05-20T01:01:01.000Z",
                "updatedAtTimestamp": "1621490985000"
            }
        ]"#;

        let accounts: Vec<AssetAccount> = serde_json::from_str(json).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].asset_symbol, "BTC");
    }
}
