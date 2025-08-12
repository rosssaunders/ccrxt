use serde::Serialize;

use super::Position;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for get all positions
const GET_ALL_POSITIONS_ENDPOINT: &str = "/api/v1/positions";

/// Request parameters for getting all positions.
///
/// This is intentionally an empty struct (not a unit struct) so that it
/// serializes to an empty JSON object `{}` rather than a string. The unit
/// struct form (`pub struct GetAllPositionsRequest;`) serializes as the
/// struct name which caused the associated test to fail.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAllPositionsRequest {}

/// Response type containing a list of all positions.
pub type GetAllPositionsResponse = Vec<Position>;

impl super::RestClient {
    /// Get Position List
    ///
    /// Get the position list of all positions for the current user.
    ///
    /// [docs]: https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-position-list
    ///
    /// Rate limit: 9
    ///
    /// # Arguments
    /// * `_request` - The get all positions request (empty)
    ///
    /// # Returns
    /// List of all positions for the current user
    pub async fn get_all_positions(
        &self,
        _request: GetAllPositionsRequest,
    ) -> Result<(RestResponse<GetAllPositionsResponse>, ResponseHeaders)> {
        self.get::<GetAllPositionsResponse, ()>(GET_ALL_POSITIONS_ENDPOINT, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kucoin::spot::{AutoDepositStatus, PositionSide};

    #[test]
    fn test_get_all_positions_request_default() {
        let request = GetAllPositionsRequest::default();
        // Just verify we can create a default instance
        let _ = request;
    }

    #[test]
    fn test_get_all_positions_response_deserialization() {
        let json = r#"[
            {
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
            }
        ]"#;

        let response: GetAllPositionsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        let position = &response[0];
        assert_eq!(position.symbol, "XBTUSDTM");
        assert_eq!(position.side, PositionSide::Long);
        assert_eq!(position.leverage, "10");
        assert_eq!(position.auto_deposit_status, AutoDepositStatus::On);
    }

    #[test]
    fn test_get_all_positions_response_deserialization_empty() {
        let json = r#"[]"#;
        let response: GetAllPositionsResponse = serde_json::from_str(json).unwrap();
        assert!(response.is_empty());
    }

    #[test]
    fn test_get_all_positions_endpoint() {
        assert_eq!(GET_ALL_POSITIONS_ENDPOINT, "/api/v1/positions");
    }

    #[test]
    fn test_multiple_positions_deserialization() {
        let json = r#"[
            {
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
            },
            {
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
            }
        ]"#;

        let response: GetAllPositionsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        // First position
        let btc_position = &response[0];
        assert_eq!(btc_position.symbol, "XBTUSDTM");
        assert_eq!(btc_position.side, PositionSide::Long);
        assert_eq!(btc_position.leverage, "10");

        // Second position
        let eth_position = &response[1];
        assert_eq!(eth_position.symbol, "ETHUSDTM");
        assert_eq!(eth_position.side, PositionSide::Short);
        assert_eq!(eth_position.leverage, "5");
    }

    #[test]
    fn test_request_serialization() {
        let request = GetAllPositionsRequest::default();
        let json = serde_json::to_value(&request).unwrap();

        // Should serialize to an empty object
        assert!(json.is_object());
        assert_eq!(json.as_object().unwrap().len(), 0);
    }

    #[test]
    fn test_position_fields_validation() {
        let json = r#"[
            {
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
            }
        ]"#;

        let response: GetAllPositionsResponse = serde_json::from_str(json).unwrap();
        let position = &response[0];

        // Validate key field types
        assert!(!position.cross_mode);
        assert!(position.deleverage_percentage > 0.0);
        assert!(position.risk_limit > 0);
        assert!(position.real_leverage > 0.0);
        assert!(position.open_time > 0);
        assert!(position.current_timestamp > 0);
    }

    #[test]
    fn test_side_variations() {
        let sides = [("long", PositionSide::Long), ("short", PositionSide::Short)];

        for (side_str, expected_side) in sides.iter() {
            let json = format!(
                r#"[
                {{
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
                }}
            ]"#,
                side_str
            );

            let response: GetAllPositionsResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response[0].side, *expected_side);
        }
    }

    #[test]
    fn test_auto_deposit_status_variations() {
        let statuses = [
            ("on", AutoDepositStatus::On),
            ("off", AutoDepositStatus::Off),
        ];

        for (status_str, expected_status) in statuses.iter() {
            let json = format!(
                r#"[
                {{
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
                }}
            ]"#,
                status_str
            );

            let response: GetAllPositionsResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response[0].auto_deposit_status, *expected_status);
        }
    }

    #[test]
    fn test_symbol_variations() {
        let symbols = ["XBTUSDTM", "ETHUSDTM", "ADAUSDTM", "DOTUSDTM"];

        for symbol in symbols.iter() {
            let json = format!(
                r#"[
                {{
                    "symbol": "{}",
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
                }}
            ]"#,
                symbol
            );

            let response: GetAllPositionsResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response[0].symbol, *symbol);
        }
    }

    #[test]
    fn test_cross_mode_variations() {
        let modes = [true, false];

        for cross_mode in modes.iter() {
            let json = format!(
                r#"[
                {{
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
                }}
            ]"#,
                cross_mode
            );

            let response: GetAllPositionsResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response[0].cross_mode, *cross_mode);
        }
    }
}
