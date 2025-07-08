use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::{ResponseHeaders, RestResponse, Result};

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

/// Margin risk limit info (cross or isolated)
#[derive(Debug, Clone, Deserialize)]
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

/// Response for getting margin risk limit info
#[derive(Debug, Clone, Deserialize)]
pub struct GetMarginRiskLimitResponse {
    pub data: Vec<MarginRiskLimitInfo>,
}

impl RestClient {
    /// Get margin risk limit info (cross or isolated)
    pub async fn get_margin_risk_limit(
        &self,
        request: GetMarginRiskLimitRequest,
    ) -> Result<(GetMarginRiskLimitResponse, ResponseHeaders)> {
        let mut params = HashMap::new();
        if let Some(is_isolated) = request.is_isolated {
            params.insert("isIsolated".to_string(), is_isolated.to_string());
        }
        if let Some(currency) = request.currency {
            params.insert("currency".to_string(), currency);
        }
        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        let (response, headers): (RestResponse<GetMarginRiskLimitResponse>, ResponseHeaders) = self
            .get(
                "/api/v3/margin/currencies",
                if params.is_empty() {
                    None
                } else {
                    Some(params)
                },
            )
            .await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
