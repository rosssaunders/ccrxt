use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures accounts
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesAccountsRequest {
    /// Settlement currency (BTC, USDT, etc.)
    pub settle: String,
}

/// Futures account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesAccount {
    /// Total balance
    pub total: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Position margin
    pub position_margin: String,

    /// Order margin
    pub order_margin: String,

    /// Available balance
    pub available: String,

    /// Point balance
    pub point: String,

    /// Currency
    pub currency: String,

    /// Balance in settlement currency
    pub in_dual_mode: bool,

    /// Enable credit
    pub enable_credit: bool,

    /// Position cross margin
    pub position_cross_margin: String,

    /// Order cross margin
    pub order_cross_margin: String,

    /// Available cross margin
    pub available_cross_margin: String,

    /// Total cross margin
    pub total_cross_margin: String,
}

/// Request parameters for futures positions
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesPositionsRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Hold mode (0: both, 1: long only, 2: short only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holding: Option<i32>,

    /// Page number for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Futures position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesPosition {
    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Position size (positive for long, negative for short)
    pub size: i64,

    /// Average entry price
    pub entry_price: String,

    /// Mark price
    pub mark_price: String,

    /// Realized PnL
    pub realised_pnl: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Position margin
    pub margin: String,

    /// Leverage
    pub leverage: String,

    /// Risk limit
    pub risk_limit: String,

    /// Liquidation price
    pub liq_price: String,

    /// Bankruptcy price
    pub bankruptcy_price: String,

    /// Cross margin mode
    pub cross_leverage_limit: String,

    /// Position mode (single or dual)
    pub mode: String,

    /// Last update timestamp
    pub update_time: i64,
}

/// Request to create futures order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFuturesOrderRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Order size (positive for long, negative for short)
    pub size: i64,

    /// Order price (omit for market orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force (gtc, ioc, poc, fok)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,

    /// Text label for order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Reduce only order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Close position order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<bool>,

    /// Iceberg order amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<i64>,

    /// Auto size for closing position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_size: Option<String>,
}

/// Futures order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesOrder {
    /// Order ID
    pub id: i64,

    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Creation timestamp
    pub create_time: f64,

    /// Finish timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<f64>,

    /// Finish reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_as: Option<String>,

    /// Order status
    pub status: String,

    /// Order size
    pub size: i64,

    /// Iceberg amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<i64>,

    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: Option<String>,

    /// Time in force
    pub tif: String,

    /// Left amount
    pub left: i64,

    /// Filled total
    pub fill_price: String,

    /// Order text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Reduce only
    pub reduce_only: bool,

    /// Close position
    pub close: bool,

    /// Reject post only
    pub reject_post_only: bool,

    /// STP action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,

    /// Amendment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,
}

/// Request parameters for listing futures orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListFuturesOrdersRequest {
    /// Settlement currency
    pub settle: String,

    /// Order status (open, finished)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-1000, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Count total records
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_total: Option<i32>,
}

/// Request to set leverage
#[derive(Debug, Clone, Serialize)]
pub struct SetLeverageRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Leverage value
    pub leverage: String,

    /// Cross margin leverage limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_leverage_limit: Option<String>,
}

/// Leverage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeverageResponse {
    /// Leverage value
    pub leverage: String,

    /// Cross margin leverage limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_leverage_limit: Option<String>,
}

/// Request to update position margin
#[derive(Debug, Clone, Serialize)]
pub struct UpdatePositionMarginRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Change amount (positive to add, negative to remove)
    pub change: String,
}

/// Position margin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionMarginResponse {
    /// New margin amount
    pub margin: String,
}

/// Request to update risk limit
#[derive(Debug, Clone, Serialize)]
pub struct UpdateRiskLimitRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Risk limit value
    pub risk_limit: String,
}

/// Risk limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimitResponse {
    /// Risk limit value
    pub risk_limit: String,
}

/// Request to set cross margin mode
#[derive(Debug, Clone, Serialize)]
pub struct CrossModeRequest {
    /// Mode ("cross" for cross margin)
    pub mode: String,
}

/// Request to enable/disable dual mode
#[derive(Debug, Clone, Serialize)]
pub struct DualModeRequest {
    /// Settlement currency
    pub settle: String,
    /// Enable dual mode
    pub dual_mode: bool,
}

/// Dual mode response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeResponse {
    /// Whether dual mode is enabled
    pub dual_mode: bool,
}

/// Position information in dual mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModePosition {
    /// User ID
    pub user: i64,
    /// Contract name
    pub contract: String,
    /// Long position size
    pub long_size: i64,
    /// Short position size
    pub short_size: i64,
    /// Long position entry price
    pub long_entry_price: String,
    /// Short position entry price
    pub short_entry_price: String,
    /// Long position leverage
    pub long_leverage: String,
    /// Short position leverage
    pub short_leverage: String,
    /// Long position margin
    pub long_margin: String,
    /// Short position margin
    pub short_margin: String,
    /// Long position PnL
    pub long_pnl: String,
    /// Short position PnL
    pub short_pnl: String,
    /// Mark price
    pub mark_price: String,
}

/// Request to update dual mode margin
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDualModeMarginRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Change amount
    pub change: String,
    /// Position side ("long" or "short")
    pub side: String,
}

/// Dual mode margin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeMarginResponse {
    /// New margin amount
    pub margin: String,
}

/// Request to update dual mode leverage
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDualModeLeverageRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Leverage value
    pub leverage: String,
    /// Position side ("long" or "short")
    pub side: String,
}

/// Dual mode leverage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeLeverageResponse {
    /// Leverage value
    pub leverage: String,
}

/// Request to update dual mode risk limit
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDualModeRiskLimitRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Risk limit value
    pub risk_limit: String,
    /// Position side ("long" or "short")
    pub side: String,
}

/// Dual mode risk limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeRiskLimitResponse {
    /// Risk limit value
    pub risk_limit: String,
}

/// Request to create batch orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchOrdersRequest {
    /// Settlement currency
    pub settle: String,
    /// List of orders to create
    pub orders: Vec<CreateFuturesOrderRequest>,
}

/// Request to cancel batch orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchCancelOrdersRequest {
    /// Settlement currency
    pub settle: String,
    /// List of order IDs to cancel
    pub order_ids: Vec<String>,
}

/// Request to amend batch orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchAmendOrdersRequest {
    /// Settlement currency
    pub settle: String,
    /// List of order amendments
    pub orders: Vec<AmendFuturesOrderRequest>,
}

/// Result of batch order operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOrderResult {
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Success status
    pub succeeded: bool,
    /// Error label if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Request for countdown cancel
#[derive(Debug, Clone, Serialize)]
pub struct CountdownCancelRequest {
    /// Settlement currency
    pub settle: String,
    /// Countdown time in seconds (0 to cancel the countdown)
    pub timeout: i32,
    /// Contract filter (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
}

/// Countdown cancel response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownCancelResponse {
    /// Remaining time in seconds
    pub timeout: i32,
}

/// Request to list orders by time range
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListOrdersByTimeRangeRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Start time in Unix seconds
    pub from: i64,
    /// End time in Unix seconds
    pub to: i64,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Maximum number of records per page (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Request to amend a futures order
#[derive(Debug, Clone, Serialize)]
pub struct AmendFuturesOrderRequest {
    /// Settlement currency
    pub settle: String,
    /// Order ID
    pub order_id: String,
    /// New size (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// New price (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Amendment text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,
}

/// Request to create a price-triggered order
#[derive(Debug, Clone, Serialize)]
pub struct CreatePriceOrderRequest {
    /// Settlement currency
    pub settle: String,
    /// Initial order (will be created when triggered)
    pub initial: CreateFuturesOrderRequest,
    /// Trigger condition
    pub trigger: TriggerCondition,
}

/// Trigger condition for price orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerCondition {
    /// Price comparison rule (>=, <=)
    pub rule: i32,
    /// Trigger price
    pub price: String,
    /// Expiration time (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
}

/// Request parameters for listing price orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPriceOrdersRequest {
    /// Settlement currency
    pub settle: String,
    /// Order status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Price-triggered order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrder {
    /// Price order ID
    pub id: i64,

    /// User ID
    pub user: i64,

    /// Creation time
    pub create_time: f64,

    /// Finish time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<f64>,

    /// Trade ID (if triggered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<i64>,

    /// Price order status
    pub status: String,

    /// Initial order details
    pub initial: CreateFuturesOrderRequest,

    /// Trigger condition
    pub trigger: TriggerCondition,

    /// Reason for order completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Request parameters for my trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct MyTradesRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Order ID filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// List offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Specify starting point
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,
    /// Count only (returns count instead of trades)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_total: Option<i32>,
}

/// Request parameters for my trades by time range
#[derive(Debug, Clone, Serialize, Default)]
pub struct MyTradesByTimeRangeRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Start time in Unix seconds
    pub from: i64,
    /// End time in Unix seconds
    pub to: i64,
    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Futures trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesTrade {
    /// Trade ID
    pub id: i64,

    /// Creation time
    pub create_time: f64,

    /// Contract name
    pub contract: String,

    /// Order ID
    pub order_id: i64,

    /// Trade size
    pub size: i64,

    /// Trade price
    pub price: String,

    /// Order role (maker/taker)
    pub role: String,

    /// Trading fee
    pub fee: String,

    /// Fee currency
    pub fee_currency: String,

    /// Point fee
    pub point_fee: String,

    /// GT fee
    pub gt_fee: String,

    /// Text
    pub text: String,
}

/// Request parameters for position close history
#[derive(Debug, Clone, Serialize, Default)]
pub struct PositionCloseHistoryRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Start time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Order side filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
}

/// Position close history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionClose {
    /// Position close time
    pub time: f64,

    /// PnL
    pub pnl: String,

    /// Position side
    pub side: String,

    /// Contract name
    pub contract: String,

    /// Text
    pub text: String,

    /// Maximum position size during the period
    pub max_size: i64,
}

/// Request parameters for liquidation history
#[derive(Debug, Clone, Serialize, Default)]
pub struct LiquidationHistoryRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Start time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Liquidation history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Liquidation {
    /// Liquidation time
    pub time: f64,

    /// Contract name
    pub contract: String,

    /// Liquidation size
    pub size: i64,

    /// Liquidation price
    pub price: String,

    /// Left position size after liquidation
    pub left: i64,

    /// Leverage
    pub leverage: String,

    /// Margin
    pub margin: String,

    /// Entry price
    pub entry_price: String,

    /// Liquidation fee
    pub liq_price: String,

    /// Mark price
    pub mark_price: String,
}

/// Request parameters for liquidation orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct LiquidationOrdersRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Start time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Liquidation order entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidationOrder {
    /// Order ID
    pub order_id: i64,

    /// Liquidation time
    pub create_time: f64,

    /// Contract name
    pub contract: String,

    /// Order size
    pub size: i64,

    /// Order price
    pub price: String,

    /// Filled amount
    pub fill_price: String,

    /// Order status
    pub status: String,
}

/// Request parameters for auto-deleveraging
#[derive(Debug, Clone, Serialize, Default)]
pub struct AutoDeleveragingRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Start time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Auto-deleveraging history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoDeleveraging {
    /// ADL time
    pub time: f64,

    /// Contract name
    pub contract: String,

    /// ADL size
    pub size: i64,

    /// ADL price
    pub price: String,

    /// Position size before ADL
    pub entry_price: String,

    /// Position size after ADL
    pub fill_price: String,

    /// Trading fee paid
    pub trade_size: i64,
}

/// Request parameters for futures account book
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesAccountBookRequest {
    /// Settlement currency
    pub settle: String,
    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Start time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End time filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Account book type filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Futures account book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesAccountBookEntry {
    /// Entry ID
    pub id: i64,

    /// Change time
    pub time: f64,

    /// Currency
    pub currency: String,

    /// Change amount
    pub change: String,

    /// Balance after change
    pub balance: String,

    /// Change type
    #[serde(rename = "type")]
    pub entry_type: String,

    /// Change text
    pub text: String,
}

/// Request parameters for futures fee rates
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesFeeRatesRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
}

/// Futures fee rates information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesFeeRates {
    /// Maker fee rate
    pub maker_fee: String,

    /// Taker fee rate
    pub taker_fee: String,

    /// Futures trading history for fee calculation
    pub futures_trading: bool,

    /// Point type
    pub point_type: String,
}

impl RestClient {
    /// Get futures account information
    ///
    /// This endpoint returns futures account balances and margin information.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#query-futures-account>
    pub async fn get_futures_accounts(
        &self,
        params: FuturesAccountsRequest,
    ) -> crate::gateio::Result<FuturesAccount> {
        let endpoint = format!("/futures/{}/accounts", params.settle);
        self.get(&endpoint).await
    }

    /// Get futures positions
    ///
    /// This endpoint returns all futures positions for the authenticated user.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-positions-of-a-user>
    pub async fn get_futures_positions(
        &self,
        params: FuturesPositionsRequest,
    ) -> crate::gateio::Result<Vec<FuturesPosition>> {
        let endpoint = format!("/futures/{}/positions", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a specific futures position
    ///
    /// This endpoint returns details for a specific futures position.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-single-position>
    pub async fn get_futures_position(
        &self,
        settle: &str,
        contract: &str,
    ) -> crate::gateio::Result<FuturesPosition> {
        let endpoint = format!("/futures/{}/positions/{}", settle, contract);
        self.get(&endpoint).await
    }

    /// Create a futures order
    ///
    /// This endpoint creates a new futures order.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#create-a-futures-order>
    pub async fn create_futures_order(
        &self,
        request: CreateFuturesOrderRequest,
    ) -> crate::gateio::Result<FuturesOrder> {
        let endpoint = format!("/futures/{}/orders", request.settle);
        self.post(&endpoint, &request).await
    }

    /// List futures orders
    ///
    /// This endpoint returns futures orders for the authenticated user.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-futures-orders>
    pub async fn list_futures_orders(
        &self,
        params: ListFuturesOrdersRequest,
    ) -> crate::gateio::Result<Vec<FuturesOrder>> {
        let endpoint = format!("/futures/{}/orders", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a specific futures order
    ///
    /// This endpoint returns details for a specific futures order.
    pub async fn get_futures_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::Result<FuturesOrder> {
        let endpoint = format!("/futures/{}/orders/{}", settle, order_id);
        self.get(&endpoint).await
    }

    /// Cancel all futures orders
    ///
    /// This endpoint cancels all futures orders for a specific contract or all contracts.
    pub async fn cancel_all_futures_orders(
        &self,
        settle: &str,
        contract: Option<&str>,
        side: Option<&str>,
    ) -> crate::gateio::Result<Vec<FuturesOrder>> {
        let mut endpoint = format!("/futures/{}/orders", settle);

        let mut query_params = Vec::new();
        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }
        if let Some(side) = side {
            query_params.push(format!("side={}", side));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        self.delete(&endpoint).await
    }

    /// Cancel a specific futures order
    ///
    /// This endpoint cancels a specific futures order.
    pub async fn cancel_futures_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::Result<FuturesOrder> {
        let endpoint = format!("/futures/{}/orders/{}", settle, order_id);
        self.delete(&endpoint).await
    }

    /// Set position leverage
    ///
    /// This endpoint sets the leverage for a specific contract position.
    pub async fn set_position_leverage(
        &self,
        request: SetLeverageRequest,
    ) -> crate::gateio::Result<LeverageResponse> {
        let endpoint = format!(
            "/futures/{}/positions/{}/leverage",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Update position margin
    ///
    /// Adjusts the margin for a specific position.
    pub async fn update_position_margin(
        &self,
        request: UpdatePositionMarginRequest,
    ) -> crate::gateio::Result<PositionMarginResponse> {
        let endpoint = format!(
            "/futures/{}/positions/{}/margin",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Update position risk limit
    ///
    /// Changes the risk limit for a specific position.
    pub async fn update_position_risk_limit(
        &self,
        request: UpdateRiskLimitRequest,
    ) -> crate::gateio::Result<RiskLimitResponse> {
        let endpoint = format!(
            "/futures/{}/positions/{}/risk_limit",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Switch to cross margin mode
    ///
    /// Switches all positions to cross margin mode.
    pub async fn switch_to_cross_margin(&self, settle: &str) -> crate::gateio::Result<()> {
        let endpoint = format!("/futures/{}/positions/cross_mode", settle);
        let request = CrossModeRequest {
            mode: "cross".to_string(),
        };
        self.post::<serde_json::Value>(&endpoint, &request).await?;
        Ok(())
    }

    /// Enable or disable dual mode
    ///
    /// Dual mode allows holding both long and short positions of the same contract simultaneously.
    pub async fn set_dual_mode(
        &self,
        request: DualModeRequest,
    ) -> crate::gateio::Result<DualModeResponse> {
        let endpoint = format!("/futures/{}/dual_mode", request.settle);
        self.post(&endpoint, &request).await
    }

    /// Get position detail in dual mode
    ///
    /// Retrieves detailed position information when dual mode is enabled.
    pub async fn get_dual_mode_position(
        &self,
        settle: &str,
        contract: &str,
    ) -> crate::gateio::Result<DualModePosition> {
        let endpoint = format!("/futures/{}/dual_comp/positions/{}", settle, contract);
        self.get(&endpoint).await
    }

    /// Update position margin in dual mode
    ///
    /// Adjusts margin for a specific position in dual mode.
    pub async fn update_dual_mode_position_margin(
        &self,
        request: UpdateDualModeMarginRequest,
    ) -> crate::gateio::Result<DualModeMarginResponse> {
        let endpoint = format!(
            "/futures/{}/dual_comp/positions/{}/margin",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Update position leverage in dual mode
    ///
    /// Changes leverage for a specific position in dual mode.
    pub async fn update_dual_mode_position_leverage(
        &self,
        request: UpdateDualModeLeverageRequest,
    ) -> crate::gateio::Result<DualModeLeverageResponse> {
        let endpoint = format!(
            "/futures/{}/dual_comp/positions/{}/leverage",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Update position risk limit in dual mode
    ///
    /// Changes risk limit for a specific position in dual mode.
    pub async fn update_dual_mode_position_risk_limit(
        &self,
        request: UpdateDualModeRiskLimitRequest,
    ) -> crate::gateio::Result<DualModeRiskLimitResponse> {
        let endpoint = format!(
            "/futures/{}/dual_comp/positions/{}/risk_limit",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Create a batch of futures orders
    ///
    /// Creates multiple orders in a single request for improved efficiency.
    pub async fn create_batch_futures_orders(
        &self,
        request: BatchOrdersRequest,
    ) -> crate::gateio::Result<Vec<BatchOrderResult>> {
        let endpoint = format!("/futures/{}/batch_orders", request.settle);
        self.post(&endpoint, &request).await
    }

    /// Cancel a batch of orders with ID list
    ///
    /// Cancels multiple orders specified by their IDs.
    pub async fn cancel_batch_futures_orders(
        &self,
        request: BatchCancelOrdersRequest,
    ) -> crate::gateio::Result<Vec<BatchOrderResult>> {
        let endpoint = format!("/futures/{}/batch_cancel_orders", request.settle);
        self.post(&endpoint, &request).await
    }

    /// Batch modify orders with specified IDs
    ///
    /// Modifies multiple orders in a single request.
    pub async fn amend_batch_futures_orders(
        &self,
        request: BatchAmendOrdersRequest,
    ) -> crate::gateio::Result<Vec<BatchOrderResult>> {
        let endpoint = format!("/futures/{}/batch_amend_orders", request.settle);
        self.post(&endpoint, &request).await
    }

    /// Countdown cancel orders
    ///
    /// Sets up a countdown timer to cancel all open orders after a specified time.
    pub async fn countdown_cancel_all_futures_orders(
        &self,
        request: CountdownCancelRequest,
    ) -> crate::gateio::Result<CountdownCancelResponse> {
        let endpoint = format!("/futures/{}/countdown_cancel_all", request.settle);
        self.post(&endpoint, &request).await
    }

    /// List futures orders by time range
    ///
    /// Retrieves orders within a specific time range for better filtering.
    pub async fn list_futures_orders_by_time_range(
        &self,
        params: ListOrdersByTimeRangeRequest,
    ) -> crate::gateio::Result<Vec<FuturesOrder>> {
        let endpoint = format!("/futures/{}/orders_timerange", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Amend a futures order
    ///
    /// Modifies an existing order's price and/or size.
    pub async fn amend_futures_order(
        &self,
        request: AmendFuturesOrderRequest,
    ) -> crate::gateio::Result<FuturesOrder> {
        let endpoint = format!("/futures/{}/orders/{}", request.settle, request.order_id);
        self.put(&endpoint, &request).await
    }

    /// Create a price-triggered order
    ///
    /// Creates a conditional order that triggers when the market price reaches a specified level.
    pub async fn create_price_triggered_order(
        &self,
        request: CreatePriceOrderRequest,
    ) -> crate::gateio::Result<PriceOrder> {
        let endpoint = format!("/futures/{}/price_orders", request.settle);
        self.post(&endpoint, &request).await
    }

    /// List all price-triggered orders
    ///
    /// Retrieves all price-triggered orders with optional filtering.
    pub async fn list_price_triggered_orders(
        &self,
        params: ListPriceOrdersRequest,
    ) -> crate::gateio::Result<Vec<PriceOrder>> {
        let endpoint = format!("/futures/{}/price_orders", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a price-triggered order
    ///
    /// Retrieves a specific price-triggered order by its ID.
    pub async fn get_price_triggered_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::Result<PriceOrder> {
        let endpoint = format!("/futures/{}/price_orders/{}", settle, order_id);
        self.get(&endpoint).await
    }

    /// Cancel a price-triggered order
    ///
    /// Cancels a specific price-triggered order.
    pub async fn cancel_price_triggered_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::Result<PriceOrder> {
        let endpoint = format!("/futures/{}/price_orders/{}", settle, order_id);
        self.delete(&endpoint).await
    }

    /// Cancel all price-triggered orders
    ///
    /// Cancels all price-triggered orders with optional contract filtering.
    pub async fn cancel_all_price_triggered_orders(
        &self,
        settle: &str,
        contract: Option<&str>,
    ) -> crate::gateio::Result<Vec<PriceOrder>> {
        let endpoint = format!("/futures/{}/price_orders", settle);

        #[derive(Serialize)]
        struct CancelAllParams<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            contract: Option<&'a str>,
        }

        let params = CancelAllParams { contract };
        self.delete_with_query(&endpoint, &params).await
    }

    /// List personal trading history
    ///
    /// Retrieves the user's trading history for futures contracts.
    pub async fn get_futures_my_trades(
        &self,
        params: MyTradesRequest,
    ) -> crate::gateio::Result<Vec<FuturesTrade>> {
        let endpoint = format!("/futures/{}/my_trades", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// List personal trading history by time range
    ///
    /// Retrieves trading history within a specific time range.
    pub async fn get_futures_my_trades_by_time_range(
        &self,
        params: MyTradesByTimeRangeRequest,
    ) -> crate::gateio::Result<Vec<FuturesTrade>> {
        let endpoint = format!("/futures/{}/my_trades_timerange", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// List position close history
    ///
    /// Retrieves history of closed positions.
    pub async fn get_futures_position_close_history(
        &self,
        params: PositionCloseHistoryRequest,
    ) -> crate::gateio::Result<Vec<PositionClose>> {
        let endpoint = format!("/futures/{}/position_close", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// List liquidation history
    ///
    /// Retrieves the user's liquidation history.
    pub async fn get_futures_liquidation_history(
        &self,
        params: LiquidationHistoryRequest,
    ) -> crate::gateio::Result<Vec<Liquidation>> {
        let endpoint = format!("/futures/{}/liquidates", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Retrieve liquidation history (alternative endpoint)
    ///
    /// Alternative endpoint for retrieving liquidation orders.
    pub async fn get_futures_liquidation_orders(
        &self,
        params: LiquidationOrdersRequest,
    ) -> crate::gateio::Result<Vec<LiquidationOrder>> {
        let endpoint = format!("/futures/{}/liq_orders", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// List auto-deleveraging history
    ///
    /// Retrieves history of auto-deleveraging events.
    pub async fn get_futures_auto_deleveraging_history(
        &self,
        params: AutoDeleveragingRequest,
    ) -> crate::gateio::Result<Vec<AutoDeleveraging>> {
        let endpoint = format!("/futures/{}/auto_deleverages", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Query futures account book
    ///
    /// Retrieves detailed account transaction history.
    pub async fn get_futures_account_book(
        &self,
        params: FuturesAccountBookRequest,
    ) -> crate::gateio::Result<Vec<FuturesAccountBookEntry>> {
        let endpoint = format!("/futures/{}/account_book", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Query user trading fee rates
    ///
    /// Retrieves the user's trading fee rates for futures contracts.
    pub async fn get_futures_fee_rates(
        &self,
        params: FuturesFeeRatesRequest,
    ) -> crate::gateio::Result<FuturesFeeRates> {
        let endpoint = format!("/futures/{}/fee", params.settle);
        self.get_with_query(&endpoint, &params).await
    }
}
