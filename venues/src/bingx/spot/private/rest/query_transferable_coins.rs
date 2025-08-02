use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const TRANSFERABLE_COINS_ENDPOINT: &str = "/openApi/api/asset/v1/transfer/supportCoins";

/// Request to query transferable currencies
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTransferableCoinsRequest {
    /// From account: fund=Funding Account, spot=Spot Account, stdFutures=Standard Contract, coinMPerp=COIN-M Perpetual Future, USDTMPerp=Perpetual Future
    pub from_account: String,

    /// To account: fund=Funding Account, spot=Spot Account, stdFutures=Standard Contract, coinMPerp=COIN-M Perpetual Future, USDTMPerp=Perpetual Future
    pub to_account: String,

    /// Execution window time, cannot be greater than 60000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Current timestamp e.g. 1658748648396
    pub timestamp: i64,
}

/// Coin asset information
#[derive(Debug, Clone, Deserialize)]
pub struct CoinAsset {
    /// Coin name
    pub asset: String,

    /// Available asset amount
    pub amount: String,
}

/// Response from the query transferable coins endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct QueryTransferableCoinsResponse {
    /// Array of coin assets
    pub coins: Vec<CoinAsset>,
}

impl RestClient {
    /// Query transferable currencies between accounts
    ///
    /// Get the list of supported coins that can be transferred between the specified account types.
    /// Rate limit: 2/s by UID & 2 by IP in group
    ///
    /// # Arguments
    /// * `request` - The query transferable coins request
    ///
    /// # Returns
    /// A result containing the supported coins or an error
    pub async fn query_transferable_coins(
        &self,
        request: &QueryTransferableCoinsRequest,
    ) -> RestResult<QueryTransferableCoinsResponse> {
        self.send_get_signed_request(TRANSFERABLE_COINS_ENDPOINT, request, EndpointType::Account)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_transferable_coins_request_serialization() {
        let request = QueryTransferableCoinsRequest {
            from_account: "fund".to_string(),
            to_account: "spot".to_string(),
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"fromAccount\":\"fund\""));
        assert!(json.contains("\"toAccount\":\"spot\""));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(json.contains("\"timestamp\":1658748648396"));
    }

    #[test]
    fn test_query_transferable_coins_response_deserialization() {
        let json = r#"
        {
            "coins": [
                {
                    "asset": "USDT",
                    "amount": "1000.50"
                },
                {
                    "asset": "BTC",
                    "amount": "0.001"
                },
                {
                    "asset": "ETH",
                    "amount": "1.5"
                }
            ]
        }
        "#;

        let response: QueryTransferableCoinsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.coins.len(), 3);

        assert_eq!(response.coins[0].asset, "USDT");
        assert_eq!(response.coins[0].amount, "1000.50");

        assert_eq!(response.coins[1].asset, "BTC");
        assert_eq!(response.coins[1].amount, "0.001");

        assert_eq!(response.coins[2].asset, "ETH");
        assert_eq!(response.coins[2].amount, "1.5");
    }

    #[test]
    fn test_minimal_request() {
        let request = QueryTransferableCoinsRequest {
            from_account: "spot".to_string(),
            to_account: "USDTMPerp".to_string(),
            recv_window: None,
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"fromAccount\":\"spot\""));
        assert!(json.contains("\"toAccount\":\"USDTMPerp\""));
        assert!(!json.contains("recvWindow"));
        assert!(json.contains("\"timestamp\":1658748648396"));
    }

    #[test]
    fn test_empty_coins_response() {
        let json = r#"
        {
            "coins": []
        }
        "#;

        let response: QueryTransferableCoinsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.coins.len(), 0);
    }
}
