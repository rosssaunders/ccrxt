use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for delivery accounts
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryAccountsRequest {
    /// Settlement currency (BTC, USDT, etc.)
    pub settle: String,
}

/// Delivery account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryAccount {
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

    /// Enable credit
    pub enable_credit: bool,

    /// Positions cross margin
    pub position_cross_margin: String,

    /// Orders cross margin
    pub order_cross_margin: String,

    /// Available cross margin
    pub available_cross_margin: String,

    /// Total cross margin
    pub total_cross_margin: String,
}

/// Request parameters for delivery positions
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryPositionsRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Maximum number of records to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Delivery position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPosition {
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

    /// Cross margin leverage limit
    pub cross_leverage_limit: String,

    /// Position mode
    pub mode: String,

    /// Last update timestamp
    pub update_time: i64,
}

/// Request to create delivery order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeliveryOrderRequest {
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

/// Delivery order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryOrder {
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

    /// STP action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,
}

/// Request parameters for listing delivery orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListDeliveryOrdersRequest {
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
}

/// Request to set delivery leverage
#[derive(Debug, Clone, Serialize)]
pub struct SetDeliveryLeverageRequest {
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

/// Delivery leverage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryLeverageResponse {
    /// Leverage value
    pub leverage: String,

    /// Cross margin leverage limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_leverage_limit: Option<String>,
}

/// Delivery settlement record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverySettlement {
    /// Settlement time
    pub time: i64,

    /// Contract
    pub contract: String,

    /// Profit in settlement currency
    pub profit: String,

    /// Settlement price
    pub settle_price: String,

    /// Position size at settlement
    pub size: i64,
}

/// Request to update delivery position margin
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDeliveryPositionMarginRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Change amount (positive to add, negative to remove)
    pub change: String,
}

/// Delivery position margin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPositionMarginResponse {
    /// New margin amount
    pub margin: String,
}

/// Request to update delivery risk limit
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDeliveryRiskLimitRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Risk limit value
    pub risk_limit: String,
}

/// Delivery risk limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRiskLimitResponse {
    /// Risk limit value
    pub risk_limit: String,
}

/// Request parameters for delivery my trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryMyTradesRequest {
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

/// Delivery trade information (reusing the public DeliveryTrade from public module)
pub type DeliveryTrade = crate::gateio::public::rest::delivery_trades::DeliveryTrade;

/// Request parameters for delivery position close history
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryPositionCloseHistoryRequest {
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

/// Delivery position close history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPositionClose {
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

/// Request parameters for delivery liquidation history
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryLiquidationHistoryRequest {
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

/// Delivery liquidation history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryLiquidation {
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

/// Request parameters for delivery account book
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryAccountBookRequest {
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

/// Delivery account book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryAccountBookEntry {
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

/// Request to create a delivery price-triggered order
#[derive(Debug, Clone, Serialize)]
pub struct CreateDeliveryPriceOrderRequest {
    /// Settlement currency
    pub settle: String,
    /// Initial order (will be created when triggered)
    pub initial: CreateDeliveryOrderRequest,
    /// Trigger condition
    pub trigger: DeliveryTriggerCondition,
}

/// Trigger condition for delivery price orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryTriggerCondition {
    /// Price comparison rule (>=, <=)
    pub rule: i32,
    /// Trigger price
    pub price: String,
    /// Expiration time (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
}

/// Request parameters for listing delivery price orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListDeliveryPriceOrdersRequest {
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

/// Delivery price-triggered order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPriceOrder {
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
    pub initial: CreateDeliveryOrderRequest,

    /// Trigger condition
    pub trigger: DeliveryTriggerCondition,

    /// Reason for order completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl RestClient {
    /// Get delivery account information
    ///
    /// This endpoint returns delivery account balances and margin information.
    pub async fn get_delivery_accounts(
        &self,
        params: DeliveryAccountsRequest,
    ) -> crate::gateio::Result<DeliveryAccount> {
        let endpoint = format!("/delivery/{}/accounts", params.settle);
        self.get(&endpoint).await
    }

    /// Get delivery positions
    ///
    /// This endpoint returns all delivery positions for the authenticated user.
    pub async fn get_delivery_positions(
        &self,
        params: DeliveryPositionsRequest,
    ) -> crate::gateio::Result<Vec<DeliveryPosition>> {
        let endpoint = format!("/delivery/{}/positions", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a specific delivery position
    ///
    /// This endpoint returns details for a specific delivery position.
    pub async fn get_delivery_position(
        &self,
        settle: &str,
        contract: &str,
    ) -> crate::gateio::Result<DeliveryPosition> {
        let endpoint = format!("/delivery/{}/positions/{}", settle, contract);
        self.get(&endpoint).await
    }

    /// Create a delivery order
    ///
    /// This endpoint creates a new delivery order.
    pub async fn create_delivery_order(
        &self,
        request: CreateDeliveryOrderRequest,
    ) -> crate::gateio::Result<DeliveryOrder> {
        let endpoint = format!("/delivery/{}/orders", request.settle);
        self.post(&endpoint, &request).await
    }

    /// List delivery orders
    ///
    /// This endpoint returns delivery orders for the authenticated user.
    pub async fn list_delivery_orders(
        &self,
        params: ListDeliveryOrdersRequest,
    ) -> crate::gateio::Result<Vec<DeliveryOrder>> {
        let endpoint = format!("/delivery/{}/orders", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a specific delivery order
    ///
    /// This endpoint returns details for a specific delivery order.
    pub async fn get_delivery_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::Result<DeliveryOrder> {
        let endpoint = format!("/delivery/{}/orders/{}", settle, order_id);
        self.get(&endpoint).await
    }

    /// Cancel all delivery orders
    ///
    /// This endpoint cancels all delivery orders for a specific contract or all contracts.
    pub async fn cancel_all_delivery_orders(
        &self,
        settle: &str,
        contract: Option<&str>,
    ) -> crate::gateio::Result<Vec<DeliveryOrder>> {
        let mut endpoint = format!("/delivery/{}/orders", settle);

        if let Some(contract) = contract {
            endpoint.push_str(&format!("?contract={}", contract));
        }

        self.delete(&endpoint).await
    }

    /// Cancel a specific delivery order
    ///
    /// This endpoint cancels a specific delivery order.
    pub async fn cancel_delivery_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::Result<DeliveryOrder> {
        let endpoint = format!("/delivery/{}/orders/{}", settle, order_id);
        self.delete(&endpoint).await
    }

    /// Set delivery position leverage
    ///
    /// This endpoint sets the leverage for a specific delivery contract position.
    pub async fn set_delivery_position_leverage(
        &self,
        request: SetDeliveryLeverageRequest,
    ) -> crate::gateio::Result<DeliveryLeverageResponse> {
        let endpoint = format!(
            "/delivery/{}/positions/{}/leverage",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Get delivery settlements
    ///
    /// This endpoint returns settlement history for delivery contracts.
    pub async fn get_delivery_settlements(
        &self,
        settle: &str,
        contract: Option<&str>,
        limit: Option<i32>,
    ) -> crate::gateio::Result<Vec<DeliverySettlement>> {
        let mut endpoint = format!("/delivery/{}/settlements", settle);
        let mut query_params = Vec::new();

        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        self.get(&endpoint).await
    }

    /// Update delivery position margin
    ///
    /// Adjusts the margin for a specific delivery position.
    pub async fn update_delivery_position_margin(
        &self,
        request: UpdateDeliveryPositionMarginRequest,
    ) -> crate::gateio::Result<DeliveryPositionMarginResponse> {
        let endpoint = format!(
            "/delivery/{}/positions/{}/margin",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Update delivery position risk limit
    ///
    /// Changes the risk limit for a specific delivery position.
    pub async fn update_delivery_position_risk_limit(
        &self,
        request: UpdateDeliveryRiskLimitRequest,
    ) -> crate::gateio::Result<DeliveryRiskLimitResponse> {
        let endpoint = format!(
            "/delivery/{}/positions/{}/risk_limit",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// List personal delivery trading history
    ///
    /// Retrieves the user's trading history for delivery contracts.
    pub async fn get_delivery_my_trades(
        &self,
        params: DeliveryMyTradesRequest,
    ) -> crate::gateio::Result<Vec<DeliveryTrade>> {
        let endpoint = format!("/delivery/{}/my_trades", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// List delivery position close history
    ///
    /// Retrieves history of closed delivery positions.
    pub async fn get_delivery_position_close_history(
        &self,
        params: DeliveryPositionCloseHistoryRequest,
    ) -> crate::gateio::Result<Vec<DeliveryPositionClose>> {
        let endpoint = format!("/delivery/{}/position_close", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// List delivery liquidation history
    ///
    /// Retrieves the user's liquidation history for delivery contracts.
    pub async fn get_delivery_liquidation_history(
        &self,
        params: DeliveryLiquidationHistoryRequest,
    ) -> crate::gateio::Result<Vec<DeliveryLiquidation>> {
        let endpoint = format!("/delivery/{}/liquidates", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Query delivery account book
    ///
    /// Retrieves detailed account transaction history for delivery trading.
    pub async fn get_delivery_account_book(
        &self,
        params: DeliveryAccountBookRequest,
    ) -> crate::gateio::Result<Vec<DeliveryAccountBookEntry>> {
        let endpoint = format!("/delivery/{}/account_book", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Create a delivery price-triggered order
    ///
    /// Creates a conditional order that triggers when the market price reaches a specified level.
    pub async fn create_delivery_price_triggered_order(
        &self,
        request: CreateDeliveryPriceOrderRequest,
    ) -> crate::gateio::Result<DeliveryPriceOrder> {
        let endpoint = format!("/delivery/{}/price_orders", request.settle);
        self.post(&endpoint, &request).await
    }

    /// List all delivery price-triggered orders
    ///
    /// Retrieves all price-triggered orders with optional filtering.
    pub async fn list_delivery_price_triggered_orders(
        &self,
        params: ListDeliveryPriceOrdersRequest,
    ) -> crate::gateio::Result<Vec<DeliveryPriceOrder>> {
        let endpoint = format!("/delivery/{}/price_orders", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a delivery price-triggered order
    ///
    /// Retrieves a specific price-triggered order by its ID.
    pub async fn get_delivery_price_triggered_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::Result<DeliveryPriceOrder> {
        let endpoint = format!("/delivery/{}/price_orders/{}", settle, order_id);
        self.get(&endpoint).await
    }

    /// Cancel a delivery price-triggered order
    ///
    /// Cancels a specific price-triggered order.
    pub async fn cancel_delivery_price_triggered_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::Result<DeliveryPriceOrder> {
        let endpoint = format!("/delivery/{}/price_orders/{}", settle, order_id);
        self.delete(&endpoint).await
    }

    /// Cancel all delivery price-triggered orders
    ///
    /// Cancels all price-triggered orders with optional contract filtering.
    pub async fn cancel_all_delivery_price_triggered_orders(
        &self,
        settle: &str,
        contract: Option<&str>,
    ) -> crate::gateio::Result<Vec<DeliveryPriceOrder>> {
        let endpoint = format!("/delivery/{}/price_orders", settle);

        #[derive(Serialize)]
        struct CancelAllParams<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            contract: Option<&'a str>,
        }

        let params = CancelAllParams { contract };
        self.delete_with_query(&endpoint, &params).await
    }
}
