use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{AutoDepositStatus, ResponseHeaders, RestResponse, Result};

/// Endpoint URL for Add Margin
const ADD_MARGIN_ENDPOINT: &str = "/api/v1/position/margin/deposit-margin";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarginRequest {
    pub symbol: String,
    pub margin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_no: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMarginResponse {
    pub id: String,
    pub symbol: String,
    pub auto_deposit_status: AutoDepositStatus,
    pub margin: String,
    pub risk_limit: i64,
    pub realized_roi: String,
    pub cross_mode: bool,
    #[serde(rename = "delevPercentage")]
    pub deleverage_percentage: f64,
    pub open_size: String,
    pub value: String,
    pub available_balance: f64,
}

impl super::RestClient {
    /// Add margin to position
    pub async fn add_margin(
        &self,
        request: AddMarginRequest,
    ) -> Result<(RestResponse<AddMarginResponse>, ResponseHeaders)> {
        let endpoint = ADD_MARGIN_ENDPOINT;
        self.post(endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_margin_request_serialization() {
        let request = AddMarginRequest {
            symbol: "XBTUSDTM".to_string(),
            margin: "100.5".to_string(),
            biz_no: Some("unique-biz-no-123".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"symbol":"XBTUSDTM","margin":"100.5","bizNo":"unique-biz-no-123"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_add_margin_request_serialization_without_biz_no() {
        let request = AddMarginRequest {
            symbol: "ETHUSDTM".to_string(),
            margin: "50.25".to_string(),
            biz_no: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{"symbol":"ETHUSDTM","margin":"50.25"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_add_margin_response_deserialization() {
        let json = r#"{
            "id": "5e8c8c2f1a3b4a001c5d8e31",
            "symbol": "XBTUSDTM",
            "autoDepositStatus": "on",
            "margin": "100.5",
            "riskLimit": 200000,
            "realizedRoi": "0.0025",
            "crossMode": false,
            "delevPercentage": 0.1,
            "openSize": "1000",
            "value": "10500.25",
            "availableBalance": 5000.5
        }"#;

        let response: AddMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "5e8c8c2f1a3b4a001c5d8e31");
        assert_eq!(response.symbol, "XBTUSDTM");
        assert_eq!(response.auto_deposit_status, AutoDepositStatus::On);
        assert_eq!(response.margin, "100.5");
        assert_eq!(response.risk_limit, 200000);
        assert_eq!(response.realized_roi, "0.0025");
        assert_eq!(response.cross_mode, false);
        assert_eq!(response.deleverage_percentage, 0.1);
        assert_eq!(response.open_size, "1000");
        assert_eq!(response.value, "10500.25");
        assert_eq!(response.available_balance, 5000.5);
    }

    #[test]
    fn test_add_margin_endpoint() {
        assert_eq!(ADD_MARGIN_ENDPOINT, "/api/v1/position/margin/deposit-margin");
    }
}
