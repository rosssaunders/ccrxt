use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{AutoDepositStatus, PositionSide, ResponseHeaders, RestResponse, Result};

/// Endpoint URL for get position
pub const GET_POSITION_ENDPOINT: &str = "/api/v1/position";

#[derive(Debug, Clone, Serialize)]
pub struct GetPositionRequest {
    pub symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub symbol: String,
    pub cross_mode: bool,
    #[serde(rename = "delevPercentage")]
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

impl super::RestClient {
    /// Get position for a specific symbol
    ///
    /// Reference: <https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-position-details>
    pub async fn get_position(
        &self,
        request: GetPositionRequest,
    ) -> Result<(Position, ResponseHeaders)> {
        let (response, headers): (RestResponse<Position>, ResponseHeaders) =
            self.get_with_request(GET_POSITION_ENDPOINT, &request).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_position_request_creation() {
        let request = GetPositionRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_position_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "crossMode": false,
            "delevPercentage": 0.1,
            "openSize": "1000",
            "value": "50000",
            "leverage": "10",
            "side": "long",
            "pnl": "100",
            "unrealizedPnl": "50",
            "unrealizedCost": "5000",
            "unrealizedRoi": "0.01",
            "unrealizedPnlPct": "0.01",
            "currentQty": "1000",
            "currentCost": "50000",
            "currentComm": "25",
            "realizedCost": "0",
            "realizedPnl": "50",
            "realizedRoi": "0.001",
            "realizedComm": "12.5",
            "openTime": 1234567890000,
            "currentTimestamp": 1234567900000,
            "autoDepositStatus": "on",
            "riskLimit": 200000,
            "realLeverage": 9.8,
            "maintenanceMargin": "2500",
            "riskLimitLevel": 2,
            "settleCurrency": "USDT",
            "markPrice": "50050",
            "positionMargin": "5000",
            "userId": "user123",
            "crossMargin": 0.0,
            "isolatedMargin": 5000.0,
            "availableBalance": 10000.0
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.symbol, "XBTUSDTM");
        assert_eq!(position.cross_mode, false);
        assert_eq!(position.deleverage_percentage, 0.1);
        assert_eq!(position.open_size, "1000");
        assert_eq!(position.value, "50000");
        assert_eq!(position.leverage, "10");
        assert_eq!(position.side, PositionSide::Long);
        assert_eq!(position.auto_deposit_status, AutoDepositStatus::On);
        assert_eq!(position.risk_limit, 200000);
        assert_eq!(position.real_leverage, 9.8);
        assert_eq!(position.user_id, Some("user123".to_string()));
    }

    #[test]
    fn test_position_deserialization_without_optional_fields() {
        let json = r#"{
            "symbol": "ETHUSDTM",
            "crossMode": true,
            "delevPercentage": 0.05,
            "openSize": "500",
            "value": "15000",
            "leverage": "5",
            "side": "short",
            "pnl": "-50",
            "unrealizedPnl": "-25",
            "unrealizedCost": "3000",
            "unrealizedRoi": "-0.008",
            "unrealizedPnlPct": "-0.008",
            "currentQty": "500",
            "currentCost": "15000",
            "currentComm": "7.5",
            "realizedCost": "0",
            "realizedPnl": "-25",
            "realizedRoi": "-0.0016",
            "realizedComm": "3.75",
            "openTime": 1234567890000,
            "currentTimestamp": 1234567900000,
            "autoDepositStatus": "off",
            "riskLimit": 100000,
            "realLeverage": 4.9,
            "maintenanceMargin": "750",
            "riskLimitLevel": 1,
            "settleCurrency": "USDT",
            "markPrice": "30050",
            "positionMargin": "3000",
            "crossMargin": 3000.0,
            "isolatedMargin": 0.0,
            "availableBalance": 5000.0
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.symbol, "ETHUSDTM");
        assert_eq!(position.side, PositionSide::Short);
        assert_eq!(position.auto_deposit_status, AutoDepositStatus::Off);
        assert!(position.user_id.is_none());
    }

    #[test]
    fn test_get_position_endpoint() {
        assert_eq!(GET_POSITION_ENDPOINT, "/api/v1/position");
    }
}
