use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{
    AutoDepositStatus, MarginMode, PositionSide, ResponseHeaders, RestResponse,
    Result,
};

/// Get position request
#[derive(Debug, Clone, Serialize)]
pub struct GetPositionRequest {
    pub symbol: String,
}

/// Position details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Symbol
    pub symbol: String,
    /// Cross mode
    pub cross_mode: bool,
    /// Deleverage percentage
    pub deleverage_percentage: f64,
    /// Open size
    pub open_size: String,
    /// Value
    pub value: String,
    /// Leverage
    pub leverage: String,
    /// Position side
    pub side: PositionSide,
    /// Profit and loss
    pub pnl: String,
    /// Unrealized PnL
    pub unrealized_pnl: String,
    /// Unrealized cost
    pub unrealized_cost: String,
    /// Unrealized return of investment
    pub unrealized_roi: String,
    /// Unrealized PnL percentage
    pub unrealized_pnl_pct: String,
    /// Current quantity
    pub current_qty: String,
    /// Current cost
    pub current_cost: String,
    /// Current commission
    pub current_comm: String,
    /// Realized cost
    pub realized_cost: String,
    /// Realized PnL
    pub realized_pnl: String,
    /// Realized return of investment
    pub realized_roi: String,
    /// Realized commission
    pub realized_comm: String,
    /// Open time
    pub open_time: i64,
    /// Current timestamp
    pub current_timestamp: i64,
    /// Auto deposit status
    pub auto_deposit_status: AutoDepositStatus,
    /// Risk limit
    pub risk_limit: i64,
    /// Real leverage
    pub real_leverage: f64,
    /// Maintenance margin
    pub maintenance_margin: String,
    /// Risk limit level
    pub risk_limit_level: i32,
    /// Settlement currency
    pub settle_currency: String,
    /// Mark price
    pub mark_price: String,
    /// Position margin
    pub position_margin: String,
    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Cross margin
    pub cross_margin: f64,
    /// Isolated margin
    pub isolated_margin: f64,
    /// Available balance
    pub available_balance: f64,
}

/// Get all positions request
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAllPositionsRequest;

/// Response for getting all positions
pub type GetAllPositionsResponse = Vec<Position>;

/// Get margin mode request
#[derive(Debug, Clone, Serialize)]
pub struct GetMarginModeRequest {
    pub symbol: String,
}

/// Margin mode response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginModeResponse {
    /// Symbol
    pub symbol: String,
    /// Margin mode
    pub margin_mode: MarginMode,
    /// Cross margin leverage
    pub cross_margin_leverage: String,
    /// Isolated margin leverage
    pub isolated_margin_leverage: String,
}

/// Change margin mode request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMarginModeRequest {
    pub symbol: String,
    pub margin_mode: MarginMode,
}

/// Change margin mode response
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeMarginModeResponse {
    pub result: bool,
}

/// Add margin request
#[derive(Debug, Clone, Serialize)]
pub struct AddMarginRequest {
    pub symbol: String,
    pub margin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_no: Option<String>,
}

/// Add margin response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarginResponse {
    /// Transaction ID
    pub id: String,
    /// Symbol
    pub symbol: String,
    /// Auto deposit status
    pub auto_deposit_status: AutoDepositStatus,
    /// Margin
    pub margin: String,
    /// Risk limit
    pub risk_limit: i64,
    /// Realized return of investment
    pub realized_roi: String,
    /// Cross mode
    pub cross_mode: bool,
    /// Deleverage percentage
    pub deleverage_percentage: f64,
    /// Open size
    pub open_size: String,
    /// Value
    pub value: String,
    /// Available balance
    pub available_balance: f64,
}

/// Auto deposit margin request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoDepositMarginRequest {
    pub symbol: String,
    pub status: AutoDepositStatus,
}

/// Auto deposit margin response
#[derive(Debug, Clone, Deserialize)]
pub struct AutoDepositMarginResponse {
    pub result: bool,
}

impl super::RestClient {
    /// Get position for a specific symbol
    pub async fn get_position(
        &self,
        request: GetPositionRequest,
    ) -> Result<(RestResponse<Position>, ResponseHeaders)> {
        let endpoint = "/api/v1/position";
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol);
        self.get(endpoint, Some(params)).await
    }

    /// Get all positions
    pub async fn get_all_positions(
        &self,
        _request: GetAllPositionsRequest,
    ) -> Result<(RestResponse<GetAllPositionsResponse>, ResponseHeaders)> {
        let endpoint = "/api/v1/positions";
        self.get(endpoint, None).await
    }

    /// Get margin mode for a symbol
    pub async fn get_margin_mode(
        &self,
        request: GetMarginModeRequest,
    ) -> Result<(RestResponse<MarginModeResponse>, ResponseHeaders)> {
        let endpoint = "/api/v2/position/getMarginMode";
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), request.symbol);
        self.get(endpoint, Some(params)).await
    }

    /// Change margin mode
    pub async fn change_margin_mode(
        &self,
        request: ChangeMarginModeRequest,
    ) -> Result<(RestResponse<ChangeMarginModeResponse>, ResponseHeaders)> {
        let endpoint = "/api/v2/position/changeMarginMode";
        self.post(endpoint, &request).await
    }

    /// Add margin to position
    pub async fn add_margin(
        &self,
        request: AddMarginRequest,
    ) -> Result<(RestResponse<AddMarginResponse>, ResponseHeaders)> {
        let endpoint = "/api/v1/position/margin/deposit-margin";
        self.post(endpoint, &request).await
    }

    /// Enable/disable auto deposit margin
    pub async fn auto_deposit_margin(
        &self,
        request: AutoDepositMarginRequest,
    ) -> Result<(RestResponse<AutoDepositMarginResponse>, ResponseHeaders)> {
        let endpoint = "/api/v1/position/margin/auto-deposit-status";
        self.post(endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_position_request_serialization() {
        let request = GetPositionRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_position_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "crossMode": true,
            "deleveragePercentage": 0.0,
            "openSize": "0",
            "value": "0",
            "leverage": "10",
            "side": "long",
            "pnl": "0",
            "unrealizedPnl": "0",
            "unrealizedCost": "0",
            "unrealizedRoi": "0",
            "unrealizedPnlPct": "0",
            "currentQty": "0",
            "currentCost": "0",
            "currentComm": "0",
            "realizedCost": "0",
            "realizedPnl": "0",
            "realizedRoi": "0",
            "realizedComm": "0",
            "openTime": 1558167872000,
            "currentTimestamp": 1558167872000,
            "autoDepositStatus": "on",
            "riskLimit": 200000,
            "realLeverage": 10.0,
            "maintenanceMargin": "0",
            "riskLimitLevel": 1,
            "settleCurrency": "USDT",
            "markPrice": "50000",
            "positionMargin": "0",
            "crossMargin": 0.0,
            "isolatedMargin": 0.0,
            "availableBalance": 0.0
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.symbol, "XBTUSDTM");
        assert_eq!(position.side, PositionSide::Long);
        assert_eq!(position.leverage, "10");
    }

    #[test]
    fn test_change_margin_mode_request_serialization() {
        let request = ChangeMarginModeRequest {
            symbol: "XBTUSDTM".to_string(),
            margin_mode: MarginMode::CrossMargin,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("XBTUSDTM"));
        assert!(json.contains("CROSS_MARGIN"));
    }
}
