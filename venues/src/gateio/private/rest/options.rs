use serde::{Deserialize, Serialize};

use super::RestClient;

/// Options account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsAccount {
    /// Total balance
    pub total: String,
    
    /// Unrealized PnL
    pub unrealised_pnl: String,
    
    /// Initial margin
    pub init_margin: String,
    
    /// Maintenance margin
    pub maint_margin: String,
    
    /// Option value
    pub option_value: String,
    
    /// Available balance
    pub available: String,
    
    /// Point balance
    pub point: String,
    
    /// Currency
    pub currency: String,
    
    /// Portfolio margin requirement
    pub portfolio_margin: String,
}

/// Request parameters for options positions
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsPositionsRequest {
    /// Underlying asset filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    
    /// Maximum number of records to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    
    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Options position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsPosition {
    /// User ID
    pub user: i64,
    
    /// Contract name
    pub contract: String,
    
    /// Position size
    pub size: String,
    
    /// Average entry price
    pub entry_price: String,
    
    /// Mark price
    pub mark_price: String,
    
    /// Mark IV (implied volatility)
    pub mark_iv: String,
    
    /// Realized PnL
    pub realised_pnl: String,
    
    /// Unrealized PnL
    pub unrealised_pnl: String,
    
    /// Pending orders
    pub pending_orders: i32,
    
    /// Close order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_order: Option<serde_json::Value>,
    
    /// Delta
    pub delta: String,
    
    /// Gamma
    pub gamma: String,
    
    /// Vega
    pub vega: String,
    
    /// Theta
    pub theta: String,
}

/// Request to create options order
#[derive(Debug, Clone, Serialize)]
pub struct CreateOptionsOrderRequest {
    /// Contract name
    pub contract: String,
    
    /// Order size
    pub size: String,
    
    /// Order price
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
}

/// Options order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsOrder {
    /// Order ID
    pub id: i64,
    
    /// User ID
    pub user: i64,
    
    /// Contract name
    pub contract: String,
    
    /// Underlying asset
    pub underlying: String,
    
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
    pub size: String,
    
    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    
    /// Time in force
    pub tif: String,
    
    /// Left amount
    pub left: String,
    
    /// Filled total
    pub filled_total: String,
    
    /// Average fill price
    pub avg_deal_price: String,
    
    /// Order text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    
    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,
    
    /// Is reduce only
    pub is_reduce_only: bool,
    
    /// Is close order
    pub is_close: bool,
    
    /// Order fee
    pub fee: String,
    
    /// Rebate
    pub rebate: String,
}

/// Request parameters for listing options orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListOptionsOrdersRequest {
    /// Order status (open, finished)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    
    /// Underlying filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    
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

/// Market Maker Protection (MMP) settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MMPSettings {
    /// User ID
    pub user: i64,
    
    /// Underlying asset
    pub underlying: String,
    
    /// Enable MMP
    pub enable: bool,
    
    /// Window size in seconds
    pub window: i32,
    
    /// Freeze time in seconds
    pub freeze_time: i32,
    
    /// Trade limit
    pub trade_limit: i32,
    
    /// Delta limit
    pub delta_limit: String,
    
    /// Vega limit
    pub vega_limit: String,
}

/// Request to update MMP settings
#[derive(Debug, Clone, Serialize)]
pub struct UpdateMMPRequest {
    /// Underlying asset
    pub underlying: String,
    
    /// Enable MMP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    
    /// Window size in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<i32>,
    
    /// Freeze time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_time: Option<i32>,
    
    /// Trade limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_limit: Option<i32>,
    
    /// Delta limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_limit: Option<String>,
    
    /// Vega limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vega_limit: Option<String>,
}

/// Options settlement record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsSettlement {
    /// Settlement time
    pub time: i64,
    
    /// Contract
    pub contract: String,
    
    /// Strike price
    pub strike: String,
    
    /// Settlement price
    pub settle_price: String,
    
    /// Size at settlement
    pub size: String,
    
    /// Profit
    pub profit: String,
}

/// Request parameters for options account book
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsAccountBookRequest {
    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    
    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    
    /// Start time filter (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    
    /// End time filter (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    
    /// Account book type filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Options account book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsAccountBookEntry {
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

/// Request parameters for options position close history
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsPositionCloseRequest {
    /// Underlying asset filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    
    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    
    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    
    /// Start time filter (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    
    /// End time filter (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Options position close history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsPositionClose {
    /// Position close time
    pub time: f64,
    
    /// PnL
    pub pnl: String,
    
    /// Contract name
    pub contract: String,
    
    /// Text
    pub text: String,
    
    /// Size at close
    pub size: String,
}

/// Request parameters for options my trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsMyTradesRequest {
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    
    /// Underlying filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    
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

/// Options trade information (personal trading history)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsMyTrade {
    /// Trade ID
    pub id: i64,
    
    /// Trade creation time
    pub create_time: f64,
    
    /// Order ID
    pub order_id: i64,
    
    /// Contract name
    pub contract: String,
    
    /// Trade size
    pub size: String,
    
    /// Trading price
    pub price: String,
    
    /// Fee paid
    pub fee: String,
    
    /// Fee currency
    pub fee_currency: String,
    
    /// Rebate
    pub rebate: String,
    
    /// Rebate currency
    pub rebate_currency: String,
    
    /// Trading role (maker/taker)
    pub role: String,
    
    /// Order text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    
    /// Underlying asset
    pub underlying: String,
}

impl RestClient {
    /// Get options account information
    /// 
    /// This endpoint returns options account balances and margin information.
    pub async fn get_options_accounts(&self) -> crate::gateio::Result<OptionsAccount> {
        self.get("/options/accounts").await
    }
    
    /// Get options positions
    /// 
    /// This endpoint returns all options positions for the authenticated user.
    pub async fn get_options_positions(&self, params: OptionsPositionsRequest) -> crate::gateio::Result<Vec<OptionsPosition>> {
        self.get_with_query("/options/positions", &params).await
    }
    
    /// Get a specific options position
    /// 
    /// This endpoint returns details for a specific options position.
    pub async fn get_options_position(&self, contract: &str) -> crate::gateio::Result<OptionsPosition> {
        let endpoint = format!("/options/positions/{}", contract);
        self.get(&endpoint).await
    }
    
    /// Create an options order
    /// 
    /// This endpoint creates a new options order.
    pub async fn create_options_order(&self, request: CreateOptionsOrderRequest) -> crate::gateio::Result<OptionsOrder> {
        self.post("/options/orders", &request).await
    }
    
    /// List options orders
    /// 
    /// This endpoint returns options orders for the authenticated user.
    pub async fn list_options_orders(&self, params: ListOptionsOrdersRequest) -> crate::gateio::Result<Vec<OptionsOrder>> {
        self.get_with_query("/options/orders", &params).await
    }
    
    /// Get a specific options order
    /// 
    /// This endpoint returns details for a specific options order.
    pub async fn get_options_order(&self, order_id: &str) -> crate::gateio::Result<OptionsOrder> {
        let endpoint = format!("/options/orders/{}", order_id);
        self.get(&endpoint).await
    }
    
    /// Cancel all options orders
    /// 
    /// This endpoint cancels all options orders for a specific underlying or contract.
    pub async fn cancel_all_options_orders(&self, underlying: Option<&str>, contract: Option<&str>) -> crate::gateio::Result<Vec<OptionsOrder>> {
        let mut endpoint = "/options/orders".to_string();
        let mut query_params = Vec::new();
        
        if let Some(underlying) = underlying {
            query_params.push(format!("underlying={}", underlying));
        }
        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }
        
        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }
        
        self.delete(&endpoint).await
    }
    
    /// Cancel a specific options order
    /// 
    /// This endpoint cancels a specific options order.
    pub async fn cancel_options_order(&self, order_id: &str) -> crate::gateio::Result<OptionsOrder> {
        let endpoint = format!("/options/orders/{}", order_id);
        self.delete(&endpoint).await
    }
    
    /// Get MMP settings
    /// 
    /// This endpoint returns Market Maker Protection settings for options trading.
    pub async fn get_mmp_settings(&self, underlying: &str) -> crate::gateio::Result<MMPSettings> {
        let endpoint = format!("/options/mmp?underlying={}", underlying);
        self.get(&endpoint).await
    }
    
    /// Update MMP settings
    /// 
    /// This endpoint updates Market Maker Protection settings.
    pub async fn update_mmp_settings(&self, request: UpdateMMPRequest) -> crate::gateio::Result<MMPSettings> {
        self.post("/options/mmp", &request).await
    }
    
    /// Reset MMP
    /// 
    /// This endpoint resets the Market Maker Protection state.
    pub async fn reset_mmp(&self, underlying: &str) -> crate::gateio::Result<()> {
        let request = serde_json::json!({
            "underlying": underlying
        });
        self.post("/options/mmp/reset", &request).await
    }
    
    /// Get options settlements
    /// 
    /// This endpoint returns settlement history for options contracts.
    pub async fn get_options_settlements(&self, underlying: Option<&str>, limit: Option<i32>) -> crate::gateio::Result<Vec<OptionsSettlement>> {
        let mut endpoint = "/options/settlements".to_string();
        let mut query_params = Vec::new();
        
        if let Some(underlying) = underlying {
            query_params.push(format!("underlying={}", underlying));
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
    
    /// Get user options settlements
    /// 
    /// This endpoint returns settlement history for the authenticated user.
    pub async fn get_user_options_settlements(&self, underlying: Option<&str>, limit: Option<i32>) -> crate::gateio::Result<Vec<OptionsSettlement>> {
        let mut endpoint = "/options/my_settlements".to_string();
        let mut query_params = Vec::new();
        
        if let Some(underlying) = underlying {
            query_params.push(format!("underlying={}", underlying));
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
    
    /// List account changing history
    ///
    /// Retrieves detailed account transaction history for options trading.
    pub async fn get_options_account_book(&self, params: OptionsAccountBookRequest) -> crate::gateio::Result<Vec<OptionsAccountBookEntry>> {
        self.get_with_query("/options/account_book", &params).await
    }
    
    /// List user's liquidation history of specified underlying
    ///
    /// Retrieves history of closed options positions.
    pub async fn get_options_position_close_history(&self, params: OptionsPositionCloseRequest) -> crate::gateio::Result<Vec<OptionsPositionClose>> {
        self.get_with_query("/options/position_close", &params).await
    }
    
    /// List personal trading history
    ///
    /// Retrieves the user's trading history for options contracts.
    pub async fn get_options_my_trades(&self, params: OptionsMyTradesRequest) -> crate::gateio::Result<Vec<OptionsMyTrade>> {
        self.get_with_query("/options/my_trades", &params).await
    }
    
    /// Countdown cancel orders
    ///
    /// Sets a countdown timer to cancel all open options orders after specified time.
    pub async fn countdown_cancel_options_orders(&self, timeout: i32, underlying: Option<&str>) -> crate::gateio::Result<serde_json::Value> {
        let mut request = serde_json::json!({
            "timeout": timeout
        });
        
        if let Some(underlying) = underlying {
            #[allow(clippy::indexing_slicing)]
            {
                request["underlying"] = serde_json::Value::String(underlying.to_string());
            }
        }
        
        self.post("/options/countdown_cancel_all", &request).await
    }
}