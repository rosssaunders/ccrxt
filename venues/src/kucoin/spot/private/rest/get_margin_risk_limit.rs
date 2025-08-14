use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for margin risk limit
const GET_MARGIN_RISK_LIMIT_ENDPOINT: &str = "/api/v3/margin/currencies";

/// Request for getting margin risk limit info
#[derive(Debug, Clone, Serialize)]
pub struct GetMarginRiskLimitRequest {
    /// True for isolated, false for cross
    #[serde(rename = "isIsolated", skip_serializing_if = "Option::is_none")]
    pub is_isolated: Option<bool>,

    /// Currency (for cross margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Symbol (for isolated margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Margin risk limit info (cross or isolated).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginRiskLimitInfo {
    pub timestamp: Option<i64>,

    // Cross margin fields
    pub currency: Option<String>,

    pub borrow_max_amount: Option<String>,

    pub buy_max_amount: Option<String>,

    pub hold_max_amount: Option<String>,

    pub borrow_coefficient: Option<String>,

    pub margin_coefficient: Option<String>,

    pub precision: Option<i32>,

    pub borrow_min_amount: Option<String>,

    pub borrow_min_unit: Option<String>,

    pub borrow_enabled: Option<bool>,

    // Isolated margin fields
    pub symbol: Option<String>,

    pub base_max_borrow_amount: Option<String>,

    pub quote_max_borrow_amount: Option<String>,

    pub base_max_buy_amount: Option<String>,

    pub quote_max_buy_amount: Option<String>,

    pub base_max_hold_amount: Option<String>,

    pub quote_max_hold_amount: Option<String>,

    pub base_precision: Option<i32>,

    pub quote_precision: Option<i32>,

    pub base_borrow_min_amount: Option<String>,

    pub quote_borrow_min_amount: Option<String>,

    pub base_borrow_min_unit: Option<String>,
}

/// Response for getting margin risk limit info.
///
/// The API returns an array directly; use a transparent wrapper to map it.
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct GetMarginRiskLimitResponse {
    pub data: Vec<MarginRiskLimitInfo>,
}

impl RestClient {
    /// Get Margin Risk Limit
    ///
    /// Request configure and risk limit info of the margin via this endpoint.
    ///
    /// - [docs](https://www.kucoin.com/docs-new/rest/margin-trading/risk-limit/get-margin-risk-limit)
    ///
    /// Rate limit: weight 20 (Private)
    ///
    /// # Arguments
    /// * `request` - The query containing one of `isIsolated=true`+symbol or `isIsolated=false`+currency
    ///
    /// # Returns
    /// A list of risk limit infos for cross or isolated margin and response headers
    pub async fn get_margin_risk_limit(
        &self,
        request: GetMarginRiskLimitRequest,
    ) -> Result<(GetMarginRiskLimitResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<GetMarginRiskLimitResponse>, ResponseHeaders) = self
            .get_with_request(GET_MARGIN_RISK_LIMIT_ENDPOINT, &request)
            .await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(GET_MARGIN_RISK_LIMIT_ENDPOINT, "/api/v3/margin/currencies");
    }

    #[test]
    fn test_get_margin_risk_limit_request_creation() {
        let req = GetMarginRiskLimitRequest {
            is_isolated: Some(true),
            currency: None,
            symbol: Some("BTC-USDT".to_string()),
        };
        assert_eq!(req.is_isolated, Some(true));
        assert_eq!(req.symbol, Some("BTC-USDT".to_string()));
    }

    #[test]
    fn test_margin_risk_limit_info_fields() {
        let info = MarginRiskLimitInfo {
            timestamp: Some(1234567890),
            currency: Some("USDT".to_string()),
            borrow_max_amount: Some("1000".to_string()),
            buy_max_amount: None,
            hold_max_amount: None,
            borrow_coefficient: None,
            margin_coefficient: None,
            precision: Some(8),
            borrow_min_amount: None,
            borrow_min_unit: None,
            borrow_enabled: Some(true),
            symbol: None,
            base_max_borrow_amount: None,
            quote_max_borrow_amount: None,
            base_max_buy_amount: None,
            quote_max_buy_amount: None,
            base_max_hold_amount: None,
            quote_max_hold_amount: None,
            base_precision: None,
            quote_precision: None,
            base_borrow_min_amount: None,
            quote_borrow_min_amount: None,
            base_borrow_min_unit: None,
        };
        assert_eq!(info.currency, Some("USDT".to_string()));
        assert_eq!(info.borrow_enabled, Some(true));
    }

    #[test]
    fn test_response_deserialization_cross_sample() {
        let json = r#"{
            "code": "200000",
            "data": [
                {
                    "timestamp": 1729678659275,
                    "currency": "BTC",
                    "borrowMaxAmount": "75.15",
                    "buyMaxAmount": "217.12",
                    "holdMaxAmount": "217.12",
                    "borrowCoefficient": "1",
                    "marginCoefficient": "1",
                    "precision": 8,
                    "borrowMinAmount": "0.001",
                    "borrowMinUnit": "0.0001",
                    "borrowEnabled": true
                }
            ]
        }"#;

        let resp: RestResponse<GetMarginRiskLimitResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.code, "200000");
        assert_eq!(resp.data.data.len(), 1);
        let item = &resp.data.data[0];
        assert_eq!(item.currency.as_deref(), Some("BTC"));
        assert_eq!(item.precision, Some(8));
        assert_eq!(item.borrow_enabled, Some(true));
    }
}
