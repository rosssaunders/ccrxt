use serde::{Deserialize, Serialize};

use crate::kucoin::{AutoDepositStatus, PositionSide, ResponseHeaders, RestResponse, Result};

/// Endpoint URL for get position
pub const GET_POSITION_ENDPOINT: &str = "/api/v1/position";
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct GetPositionRequest {
    pub symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub symbol: String,
    pub cross_mode: bool,
    pub deleverage_percentage: f64,
    pub open_size: String,
    pub value: String,
    pub leverage: String,
    pub side: PositionSide,
    pub pnl: String,
    pub unrealized_pnl: String,
    pub unrealized_cost: String,
    pub unrealized_roi: String,
    pub unrealized_pnl_pct: String,
    pub current_qty: String,
    pub current_cost: String,
    pub current_comm: String,
    pub realized_cost: String,
    pub realized_pnl: String,
    pub realized_roi: String,
    pub realized_comm: String,
    pub open_time: i64,
    pub current_timestamp: i64,
    pub auto_deposit_status: AutoDepositStatus,
    pub risk_limit: i64,
    pub real_leverage: f64,
    pub maintenance_margin: String,
    pub risk_limit_level: i32,
    pub settle_currency: String,
    pub mark_price: String,
    pub position_margin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub cross_margin: f64,
    pub isolated_margin: f64,
    pub available_balance: f64,
}

impl super::super::RestClient {
    /// Get position for a specific symbol
    pub async fn get_position(
        &self,
        request: GetPositionRequest,
    ) -> Result<(RestResponse<Position>, ResponseHeaders)> {
        let endpoint = GET_POSITION_ENDPOINT;
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol);
        self.get(endpoint, Some(params)).await
    }
}
