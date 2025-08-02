use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::{OptionsContractType, OptionsOrderSide, RestResult};

const CREATE_BLOCK_TRADE_ENDPOINT: &str = "/eapi/v1/block/order/create";

/// Request parameters for creating a block trade order
#[derive(Debug, Clone, Serialize)]
pub struct CreateBlockTradeRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Order quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Order price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Counterparty ID
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: u64,

    /// User-defined order ID
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Block trade order response
#[derive(Debug, Clone, Deserialize)]
pub struct BlockTradeOrderResponse {
    /// System order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Order quantity
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// Buy/sell direction
    #[serde(rename = "side")]
    pub side: OptionsOrderSide,

    /// Order creation time
    #[serde(rename = "createTime")]
    pub create_time: u64,

    /// Order update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Block trade status
    #[serde(rename = "status")]
    pub status: String,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Option side (CALL or PUT)
    #[serde(rename = "optionSide")]
    pub option_side: OptionsContractType,

    /// Quote asset
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,

    /// Counterparty ID
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: u64,

    /// Order expire time
    #[serde(rename = "expireTime")]
    pub expire_time: u64,
}

impl RestClient {
    /// Send in a new block trade order
    ///
    /// Creates a new block trade order for institutional trading.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/option/block-trade/Send-in-a-New-Block-Trade-Order
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The create block trade request parameters
    ///
    /// # Returns
    /// Block trade order response with order details
    pub async fn create_block_trade_order(
        &self,
        params: CreateBlockTradeRequest,
    ) -> RestResult<BlockTradeOrderResponse> {
        self.send_post_signed_request(
            CREATE_BLOCK_TRADE_ENDPOINT,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }
}
