use serde::{Deserialize, Serialize};

use crate::deribit::{
    EndpointType, JsonRpcResult, PrivateRestClient, RestResult, enums::OrderDirection,
};

/// REST API endpoint constant
const GET_PENDING_BLOCK_TRADES_ENDPOINT: &str = "private/get_pending_block_trades";

/// Request parameters for the get_pending_block_trades endpoint.
///
/// This endpoint does not take any parameters.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetPendingBlockTradesRequest;

/// Represents the state of a pending block trade (for user or counterparty).
#[derive(Debug, Clone, Deserialize)]
pub struct PendingBlockTradeState {
    /// State timestamp.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,

    /// State value.
    #[serde(rename = "value")]
    pub value: String,
}

/// Represents a single trade within a pending block trade approval.
#[derive(Debug, Clone, Deserialize)]
pub struct PendingBlockTradeTrade {
    /// Trade amount. For perpetual and inverse futures the amount is in USD units. For options and linear futures it is the underlying base currency coin.
    #[serde(rename = "amount")]
    pub amount: f64,

    /// Direction: `buy` or `sell`.
    #[serde(rename = "direction")]
    pub direction: OrderDirection,

    /// Unique instrument identifier.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Price in base currency.
    #[serde(rename = "price")]
    pub price: f64,
}

/// Represents a single pending block trade approval.
#[derive(Debug, Clone, Deserialize)]
pub struct PendingBlockTrade {
    /// The name of the application that executed the block trade on behalf of the user (optional).
    #[serde(rename = "app_name")]
    pub app_name: Option<String>,

    /// State of the pending block trade for the other party (optional).
    #[serde(rename = "counterparty_state")]
    pub counterparty_state: Option<PendingBlockTradeState>,

    /// Nonce that can be used to approve or reject pending block trade.
    #[serde(rename = "nonce")]
    pub nonce: String,

    /// Trade role of the user: `maker` or `taker`.
    #[serde(rename = "role")]
    pub role: PendingBlockTradeRole,

    /// State of the pending block trade for current user.
    #[serde(rename = "state")]
    pub state: PendingBlockTradeState,

    /// Timestamp that can be used to approve or reject pending block trade.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,

    /// Trades included in this block trade approval.
    #[serde(rename = "trades")]
    pub trades: Vec<PendingBlockTradeTrade>,

    /// Unique user identifier.
    #[serde(rename = "user_id")]
    pub user_id: i64,
}

/// Enum for trade role of the user: `maker` or `taker`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PendingBlockTradeRole {
    #[serde(rename = "maker")]
    Maker,

    #[serde(rename = "taker")]
    Taker,
}

/// Response for get_pending_block_trades endpoint.
pub type GetPendingBlockTradesResponse = JsonRpcResult<Vec<PendingBlockTrade>>;

impl PrivateRestClient {
    /// Provides a list of pending block trade approvals.
    ///
    /// [docs](https://docs.deribit.com/v2/#private-get_pending_block_trades)
    ///
    /// This endpoint requires authentication and the `block_trade:read` scope.
    pub async fn get_pending_block_trades(
        &self,
        request: GetPendingBlockTradesRequest,
    ) -> RestResult<GetPendingBlockTradesResponse> {
        self.send_signed_request(
            GET_PENDING_BLOCK_TRADES_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    /// REST API endpoint constant
    use super::*;

    #[test]
    fn test_deserialize_pending_block_trade() {
        let data = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": [
                {
                    "app_name": "test_app",
                    "counterparty_state": { "timestamp": 1710000000, "value": "pending" },
                    "nonce": "abc123",
                    "role": "maker",
                    "state": { "timestamp": 1710000001, "value": "pending" },
                    "timestamp": 1710000002,
                    "trades": [
                        {
                            "amount": 1.23,
                            "direction": "buy",
                            "instrument_name": "BTC-PERPETUAL",
                            "price": 50000.0
                        }
                    ],
                    "user_id": 456
                }
            ]
        });
        let resp: GetPendingBlockTradesResponse = serde_json::from_value(data).unwrap();
        assert_eq!(resp.id, 123);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 1);
        let trade = &resp.result[0];
        assert_eq!(trade.app_name.as_deref(), Some("test_app"));
        assert_eq!(trade.nonce, "abc123");
        assert_eq!(trade.role, PendingBlockTradeRole::Maker);
        assert_eq!(trade.state.value, "pending");
        assert_eq!(trade.trades.len(), 1);
        assert_eq!(trade.trades[0].direction, OrderDirection::Buy);
    }
}
