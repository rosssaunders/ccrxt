use serde::{Deserialize, Serialize};

use crate::bullish::private::rest::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

const COMMAND_ENDPOINT: &str = "/v2/command";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CommandType {
    #[serde(rename = "V1CancelAllOrdersByMarket")]
    V1CancelAllOrdersByMarket,
}

impl Default for CommandType {
    fn default() -> Self {
        CommandType::V1CancelAllOrdersByMarket
    }
}

/// Request parameters for cancelling all orders for a specific market.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersByMarketRequest {
    /// The command type, it must be 'V1CancelAllOrdersByMarket'.
    #[serde(rename = "commandType")]
    pub command_type: CommandType,

    /// Market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market.
    pub symbol: String,

    /// Unique trading account ID.
    #[serde(rename = "tradingAccountId")]
    pub trading_account_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersByMarketResponse {
    pub message: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
}

impl RestClient {
    /// Cancel all orders by market (V1CancelAllOrdersByMarket)
    ///
    /// Cancels all outstanding orders for the given market and trading account.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#post-/v2/command#cancellations
    pub async fn cancel_all_orders_by_market(
        &mut self,
        request: CancelAllOrdersByMarketRequest,
    ) -> RestResult<CancelAllOrdersByMarketResponse> {
        self
            .send_post_request(COMMAND_ENDPOINT, request, EndpointType::PrivateOrders)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_orders_by_market_request_serialization() {
        let req = CancelAllOrdersByMarketRequest {
            command_type: CommandType::V1CancelAllOrdersByMarket,
            symbol: "BTCUSDC".to_string(),
            trading_account_id: "111000000000001".to_string(),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("V1CancelAllOrdersByMarket"));
        assert!(json.contains("symbol"));
    }
}
