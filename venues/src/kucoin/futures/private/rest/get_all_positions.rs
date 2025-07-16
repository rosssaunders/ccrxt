use serde::Serialize;

use super::Position;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for get all positions
pub const GET_ALL_POSITIONS_ENDPOINT: &str = "/api/v1/positions";

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAllPositionsRequest;

pub type GetAllPositionsResponse = Vec<Position>;

impl super::RestClient {
    /// Get all positions
    pub async fn get_all_positions(
        &self,
        _request: GetAllPositionsRequest,
    ) -> Result<(RestResponse<GetAllPositionsResponse>, ResponseHeaders)> {
        self.get(GET_ALL_POSITIONS_ENDPOINT, None).await
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
}
