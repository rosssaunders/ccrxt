use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::{EndpointType, RestResult};

/// Request to get account balances
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalancesRequest {
    /// Timestamp of initiating the request, Unit: milliseconds
    /// This will be automatically set by the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
}

/// Response from the get balances endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetBalancesResponse {
    /// Asset list
    pub balances: Vec<Balance>,
}

/// Balance information for a specific asset
#[derive(Debug, Clone, Deserialize)]
pub struct Balance {
    /// Asset name (e.g., "BTC", "USDT")
    pub asset: String,

    /// Display name for the asset
    #[serde(rename = "displayName")]
    pub display_name: String,

    /// Available asset amount
    pub free: String,

    /// Frozen/locked asset amount
    pub locked: String,
}

impl RestClient {
    /// Get account balances
    ///
    /// Retrieves the account balance for all assets in the spot/fund account.
    /// Rate limit: 5/s by UID
    ///
    /// # Arguments
    /// * `request` - The get balances request (can be empty for default parameters)
    ///
    /// # Returns
    /// A result containing the account balances or an error
    ///
    /// # Example
    /// ```rust,no_run
    /// use venues::bingx::{PrivateRestClient, GetBalancesRequest};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client: PrivateRestClient = todo!("Provide a real PrivateRestClient instance for this example");
    ///     let request = GetBalancesRequest::default();
    ///     let balances = client.get_balances(&request).await?;
    ///     println!("Balances: {:?}", balances);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_balances(&self, request: &GetBalancesRequest) -> RestResult<GetBalancesResponse> {
        self.send_request(
            "/openApi/spot/v1/account/balance",
            reqwest::Method::GET,
            Some(request),
            EndpointType::Account,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_balances_request_serialization() {
        let request = GetBalancesRequest {
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_get_balances_request_default() {
        let request = GetBalancesRequest::default();
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_balance_deserialization() {
        let json = r#"{
            "balances": [
                {
                    "asset": "BTC",
                    "displayName": "Bitcoin",
                    "free": "1.00000000",
                    "locked": "0.00000000"
                },
                {
                    "asset": "USDT",
                    "displayName": "Tether USD",
                    "free": "1000.50000000",
                    "locked": "50.00000000"
                }
            ]
        }"#;

        let response: GetBalancesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.balances.len(), 2);

        let btc_balance = &response.balances[0];
        assert_eq!(btc_balance.asset, "BTC");
        assert_eq!(btc_balance.display_name, "Bitcoin");
        assert_eq!(btc_balance.free, "1.00000000");
        assert_eq!(btc_balance.locked, "0.00000000");

        let usdt_balance = &response.balances[1];
        assert_eq!(usdt_balance.asset, "USDT");
        assert_eq!(usdt_balance.display_name, "Tether USD");
        assert_eq!(usdt_balance.free, "1000.50000000");
        assert_eq!(usdt_balance.locked, "50.00000000");
    }
}
