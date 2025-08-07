use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{AutoDepositStatus, PositionSide, ResponseHeaders, RestResponse, Result};

/// Endpoint URL for getting position details
const GET_POSITION_ENDPOINT: &str = "/api/v1/position";

/// Request parameters for getting position details.
#[derive(Debug, Clone, Serialize)]
pub struct GetPositionRequest {
    /// Trading symbol for which to retrieve position details (e.g., "XBTUSDTM").
    pub symbol: String,
}

/// Position details for a specific symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Trading symbol for the position.
    pub symbol: String,

    /// Whether the position uses cross margin mode.
    pub cross_mode: bool,

    /// Deleveraging percentage threshold.
    #[serde(rename = "delevPercentage")]
    pub deleverage_percentage: f64,

    /// Open position size as a string.
    pub open_size: String,

    /// Position value in settlement currency as a string.
    pub value: String,

    /// Current leverage ratio as a string.
    pub leverage: String,

    /// Position side (long or short).
    pub side: PositionSide,

    /// Total profit and loss as a string.
    pub pnl: String,

    /// Unrealized profit and loss as a string.
    pub unrealized_pnl: String,

    /// Unrealized cost of the position as a string.
    pub unrealized_cost: String,

    /// Unrealized return on investment as a string.
    pub unrealized_roi: String,

    /// Unrealized PnL percentage as a string.
    pub unrealized_pnl_pct: String,

    /// Current position quantity as a string.
    pub current_qty: String,

    /// Current position cost as a string.
    pub current_cost: String,

    /// Current commission paid as a string.
    pub current_comm: String,

    /// Realized cost of the position as a string.
    pub realized_cost: String,

    /// Realized profit and loss as a string.
    pub realized_pnl: String,

    /// Realized return on investment as a string.
    pub realized_roi: String,

    /// Realized commission paid as a string.
    pub realized_comm: String,

    /// Position opening time (milliseconds since epoch).
    pub open_time: i64,

    /// Current timestamp (milliseconds since epoch).
    pub current_timestamp: i64,

    /// Auto deposit status for margin.
    pub auto_deposit_status: AutoDepositStatus,

    /// Risk limit for the position.
    pub risk_limit: i64,

    /// Real leverage ratio as a floating point number.
    pub real_leverage: f64,

    /// Maintenance margin requirement as a string.
    pub maintenance_margin: String,

    /// Risk limit level tier.
    pub risk_limit_level: i32,

    /// Settlement currency for the position.
    pub settle_currency: String,

    /// Current mark price as a string.
    pub mark_price: String,

    /// Position margin amount as a string.
    pub position_margin: String,

    /// User ID associated with the position.
    /// Optional field - may not be present in all responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Cross margin amount as a floating point number.
    pub cross_margin: f64,

    /// Isolated margin amount as a floating point number.
    pub isolated_margin: f64,

    /// Available balance as a floating point number.
    pub available_balance: f64,
}

impl super::RestClient {
    /// Get Position Details
    ///
    /// Get detailed position information for a specific symbol.
    ///
    /// [docs]: https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-position-details
    ///
    /// Rate limit: 9
    ///
    /// # Arguments
    /// * `request` - The position request parameters
    ///
    /// # Returns
    /// Detailed position information for the specified symbol
    pub async fn get_position(
        &self,
        request: GetPositionRequest,
    ) -> Result<(RestResponse<Position>, ResponseHeaders)> {
        self.get(GET_POSITION_ENDPOINT, Some(&request)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kucoin::spot::{AutoDepositStatus, PositionSide};

    #[test]
    fn test_get_position_request_creation() {
        let request = GetPositionRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_request_serialization() {
        let request = GetPositionRequest {
            symbol: "ETHUSDTM".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDTM");
    }

    #[test]
    fn test_symbol_variations() {
        let symbols = ["XBTUSDTM", "ETHUSDTM", "ADAUSDTM", "DOTUSDTM"];

        for symbol in symbols.iter() {
            let request = GetPositionRequest {
                symbol: symbol.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], *symbol);
        }
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
    fn test_position_side_variations() {
        let sides = [("long", PositionSide::Long), ("short", PositionSide::Short)];

        for (side_str, expected_side) in sides.iter() {
            let json = format!(r#"{{
                "symbol": "XBTUSDTM",
                "crossMode": false,
                "delevPercentage": 0.1,
                "openSize": "1000",
                "value": "50000",
                "leverage": "10",
                "side": "{}",
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
                "crossMargin": 0.0,
                "isolatedMargin": 5000.0,
                "availableBalance": 10000.0
            }}"#, side_str);

            let position: Position = serde_json::from_str(&json).unwrap();
            assert_eq!(position.side, *expected_side);
        }
    }

    #[test]
    fn test_auto_deposit_status_variations() {
        let statuses = [
            ("on", AutoDepositStatus::On),
            ("off", AutoDepositStatus::Off),
        ];

        for (status_str, expected_status) in statuses.iter() {
            let json = format!(r#"{{
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
                "autoDepositStatus": "{}",
                "riskLimit": 200000,
                "realLeverage": 9.8,
                "maintenanceMargin": "2500",
                "riskLimitLevel": 2,
                "settleCurrency": "USDT",
                "markPrice": "50050",
                "positionMargin": "5000",
                "crossMargin": 0.0,
                "isolatedMargin": 5000.0,
                "availableBalance": 10000.0
            }}"#, status_str);

            let position: Position = serde_json::from_str(&json).unwrap();
            assert_eq!(position.auto_deposit_status, *expected_status);
        }
    }

    #[test]
    fn test_cross_mode_variations() {
        let cross_modes = [true, false];

        for cross_mode in cross_modes.iter() {
            let json = format!(r#"{{
                "symbol": "XBTUSDTM",
                "crossMode": {},
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
                "crossMargin": 0.0,
                "isolatedMargin": 5000.0,
                "availableBalance": 10000.0
            }}"#, cross_mode);

            let position: Position = serde_json::from_str(&json).unwrap();
            assert_eq!(position.cross_mode, *cross_mode);
        }
    }

    #[test]
    fn test_field_types() {
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
            "crossMargin": 0.0,
            "isolatedMargin": 5000.0,
            "availableBalance": 10000.0
        }"#;

        let json_value = serde_json::from_str::<serde_json::Value>(json).unwrap();
        
        // Verify field types in JSON
        assert!(json_value["symbol"].is_string());
        assert!(json_value["crossMode"].is_boolean());
        assert!(json_value["delevPercentage"].is_number());
        assert!(json_value["openSize"].is_string());
        assert!(json_value["leverage"].is_string());
        assert!(json_value["side"].is_string());
        assert!(json_value["openTime"].is_number());
        assert!(json_value["currentTimestamp"].is_number());
        assert!(json_value["riskLimit"].is_number());
        assert!(json_value["realLeverage"].is_number());
        assert!(json_value["riskLimitLevel"].is_number());
        
        // Verify deserialization works
        let position: Position = serde_json::from_str(json).unwrap();
        assert_eq!(position.symbol, "XBTUSDTM");
        assert_eq!(position.cross_mode, false);
        assert_eq!(position.open_time, 1234567890000);
        assert_eq!(position.risk_limit, 200000);
    }

    #[test]
    fn test_camel_case_conversion() {
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
            "crossMargin": 0.0,
            "isolatedMargin": 5000.0,
            "availableBalance": 10000.0
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        
        // Verify camelCase fields are properly converted to snake_case
        assert_eq!(position.cross_mode, false);
        assert_eq!(position.open_size, "1000");
        assert_eq!(position.unrealized_pnl, "50");
        assert_eq!(position.unrealized_cost, "5000");
        assert_eq!(position.unrealized_roi, "0.01");
        assert_eq!(position.unrealized_pnl_pct, "0.01");
        assert_eq!(position.current_qty, "1000");
        assert_eq!(position.current_cost, "50000");
        assert_eq!(position.current_comm, "25");
        assert_eq!(position.realized_cost, "0");
        assert_eq!(position.realized_pnl, "50");
        assert_eq!(position.realized_roi, "0.001");
        assert_eq!(position.realized_comm, "12.5");
        assert_eq!(position.open_time, 1234567890000);
        assert_eq!(position.current_timestamp, 1234567900000);
        assert_eq!(position.auto_deposit_status, AutoDepositStatus::On);
        assert_eq!(position.real_leverage, 9.8);
        assert_eq!(position.maintenance_margin, "2500");
        assert_eq!(position.risk_limit_level, 2);
        assert_eq!(position.settle_currency, "USDT");
        assert_eq!(position.mark_price, "50050");
        assert_eq!(position.position_margin, "5000");
        assert_eq!(position.cross_margin, 0.0);
        assert_eq!(position.isolated_margin, 5000.0);
        assert_eq!(position.available_balance, 10000.0);
    }

    #[test]
    fn test_optional_user_id_field() {
        // Test with user_id present
        let json_with_user = r#"{
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
            "userId": "test_user_123",
            "crossMargin": 0.0,
            "isolatedMargin": 5000.0,
            "availableBalance": 10000.0
        }"#;

        let position: Position = serde_json::from_str(json_with_user).unwrap();
        assert_eq!(position.user_id, Some("test_user_123".to_string()));

        // Test without user_id (should work with existing tests)
        let json_without_user = r#"{
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

        let position_without: Position = serde_json::from_str(json_without_user).unwrap();
        assert!(position_without.user_id.is_none());
    }

    #[test]
    fn test_leverage_variations() {
        let leverages = ["1", "5", "10", "20", "50", "100"];

        for leverage in leverages.iter() {
            let json = format!(r#"{{
                "symbol": "XBTUSDTM",
                "crossMode": false,
                "delevPercentage": 0.1,
                "openSize": "1000",
                "value": "50000",
                "leverage": "{}",
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
                "crossMargin": 0.0,
                "isolatedMargin": 5000.0,
                "availableBalance": 10000.0
            }}"#, leverage);

            let position: Position = serde_json::from_str(&json).unwrap();
            assert_eq!(position.leverage, *leverage);
        }
    }

    #[test]
    fn test_position_fields_validation() {
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
            "crossMargin": 0.0,
            "isolatedMargin": 5000.0,
            "availableBalance": 10000.0
        }"#;

        let position: Position = serde_json::from_str(json).unwrap();
        
        // Validate numeric field ranges
        assert!(position.deleverage_percentage > 0.0);
        assert!(position.risk_limit > 0);
        assert!(position.real_leverage > 0.0);
        assert!(position.open_time > 0);
        assert!(position.current_timestamp > 0);
        assert!(position.risk_limit_level > 0);
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(GET_POSITION_ENDPOINT, "/api/v1/position");
    }
}
