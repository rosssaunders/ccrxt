use serde::{Deserialize, Serialize};

use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::coinm::{OrderSide, PositionSide, RestResult};

/// Request parameters for the Account Trade List endpoint (GET /dapi/v1/userTrades).
///
/// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Account-Trade-List>
#[derive(Debug, Clone, Serialize, Default)]
pub struct AccountTradeListRequest {
    /// The trading symbol (e.g., "BTCUSD_PERP").
    /// Either `symbol` or `pair` must be sent, but not both.
    /// Format: "{ASSET}USD_PERP" for perpetual contracts, "{ASSET}USD_{EXPIRY}" for delivery contracts.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// The trading pair (e.g., "BTCUSD").
    /// Either `symbol` or `pair` must be sent, but not both.
    /// Format: "{ASSET}USD" where ASSET is the base asset (e.g., BTC, ETH).
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,

    /// The order ID to filter trades by. Can only be sent with `symbol`.
    /// Must be a valid order ID from a previous order.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Start time in milliseconds since epoch.
    /// Must be less than endTime if both are provided.
    /// Range: 0 to current timestamp.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds since epoch.
    /// Must be greater than startTime if both are provided.
    /// Range: 0 to current timestamp.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Trade ID to fetch from. Default gets most recent trades. Cannot be sent with `pair`.
    /// Must be a valid trade ID from a previous trade.
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Number of results to return. Default 50; max 1000.
    /// Range: 1 to 1000.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// The value cannot be greater than 60000.
    /// Range: 0 to 60000 milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds since epoch. Mandatory.
    /// Must be within 1000ms of server time.
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Represents a single trade returned by the Account Trade List endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct AccountTrade {
    /// The trading symbol (e.g., "BTCUSD_200626").
    /// Format: "{ASSET}USD_{EXPIRY}" for delivery contracts, "{ASSET}USD_PERP" for perpetual contracts.
    pub symbol: String,

    /// Trade ID.
    /// Unique identifier for the trade.
    pub id: u64,

    /// Order ID associated with this trade.
    /// Links to the original order that generated this trade.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// The trading pair (e.g., "BTCUSD").
    /// Format: "{ASSET}USD" where ASSET is the base asset.
    pub pair: String,

    /// The side of the trade (BUY or SELL).
    /// Indicates whether the trade was a buy or sell order.
    pub side: OrderSide,

    /// The price at which the trade was executed.
    /// Format: Decimal string with precision up to 8 decimal places.
    pub price: String,

    /// The quantity of the trade.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "qty")]
    pub quantity: String,

    /// Realized PnL for this trade.
    /// Format: Decimal string with precision up to 8 decimal places.
    /// Positive for profit, negative for loss.
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: String,

    /// The margin asset used for this trade (e.g., "BTC").
    /// The asset used for margin calculations and fee payments.
    #[serde(rename = "marginAsset")]
    pub margin_asset: String,

    /// The base quantity for this trade.
    /// Format: Decimal string with precision up to 8 decimal places.
    #[serde(rename = "baseQty")]
    pub base_qty: String,

    /// The commission paid for this trade.
    /// Format: Decimal string with precision up to 8 decimal places.
    pub commission: String,

    /// The asset in which commission was paid.
    /// The asset used to pay the trading fee.
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,

    /// The timestamp of the trade in milliseconds since epoch.
    /// When the trade was executed on the exchange.
    pub time: u64,

    /// The position side (BOTH, LONG, SHORT).
    /// Indicates the position direction for this trade.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    /// Whether the buyer is the taker.
    /// True if the buyer was the taker, false if the seller was the taker.
    pub buyer: bool,

    /// Whether the trade was a maker trade.
    /// True if the trade was a maker trade, false if it was a taker trade.
    pub maker: bool,
}

impl RestClient {
    /// Fetches trades for a specific account and symbol or pair.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Account-Trade-List>
    ///
    /// GET /dapi/v1/userTrades
    /// Weight: 20 with symbol, 40 with pair
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`AccountTradeListRequest`])
    ///
    /// # Returns
    /// A vector of [`AccountTrade`] objects.
    pub async fn get_account_trades(
        &self,
        params: AccountTradeListRequest,
    ) -> RestResult<Vec<AccountTrade>> {
        let weight = if params.pair.is_some() { 40 } else { 20 };
        self.send_signed_request(
            "/dapi/v1/userTrades",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
