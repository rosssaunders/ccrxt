use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

const COMMAND_ENDPOINT: &str = "/v1/command";

/// Command type for transfer asset operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferCommandType {
    /// V1 transfer asset command
    #[serde(rename = "V1TransferAsset")]
    V1TransferAsset,
}

impl Default for TransferCommandType {
    fn default() -> Self {
        Self::V1TransferAsset
    }
}

/// Request payload for asset transfer between trading accounts
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferAssetRequest {
    /// Timestamp (ms since epoch) as string
    pub timestamp: String,

    /// Nonce, unsigned 64-bit integer as string
    pub nonce: String,

    /// Authorizer obtained with the JWT token
    pub authorizer: String,

    /// Command object for the transfer
    pub command: TransferAssetCommand,
}

/// Transfer command body sent under `command`
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferAssetCommand {
    /// The command type, must be 'V1TransferAsset'
    #[serde(rename = "commandType")]
    pub command_type: TransferCommandType,

    /// Symbol of the asset (e.g., BTC)
    #[serde(rename = "assetSymbol")]
    pub asset_symbol: String,

    /// Quantity of the asset
    pub quantity: String,

    /// Source trading account ID
    #[serde(rename = "fromTradingAccountId")]
    pub from_trading_account_id: String,

    /// Destination trading account ID
    #[serde(rename = "toTradingAccountId")]
    pub to_trading_account_id: String,
}

/// Response for transfer asset command
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferAssetResponse {
    /// Acknowledgment message
    pub message: String,

    /// Unique request ID
    #[serde(rename = "requestId")]
    pub request_id: String,
}

impl RestClient {
    /// Transfer Asset (V1TransferAsset)
    ///
    /// Sends a command to transfer asset between two trading accounts.
    /// Requires JWT (Authorization) and BX-* signature headers are handled by the client.
    ///
    /// [docs](https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#post-/v1/command)
    pub async fn transfer_asset(
        &mut self,
        request: TransferAssetRequest,
    ) -> RestResult<TransferAssetResponse> {
        // API requires commandType as a query parameter as well as within the body
        let endpoint_with_query = format!("{}?commandType=V1TransferAsset", COMMAND_ENDPOINT);
        self.send_post_request(
            &endpoint_with_query,
            request,
            EndpointType::PrivateTradingAccounts,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_asset_request_serialization() {
        let req = TransferAssetRequest {
            timestamp: "1621490985000".to_string(),
            nonce: "123456789".to_string(),
            authorizer: "03E02367E8C9".to_string(),
            command: TransferAssetCommand {
                command_type: TransferCommandType::V1TransferAsset,
                asset_symbol: "BTC".to_string(),
                quantity: "100".to_string(),
                from_trading_account_id: "111000000000001".to_string(),
                to_trading_account_id: "111000000000002".to_string(),
            },
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("V1TransferAsset"));
        assert!(json.contains("assetSymbol"));
        assert!(json.contains("fromTradingAccountId"));
    }
}
