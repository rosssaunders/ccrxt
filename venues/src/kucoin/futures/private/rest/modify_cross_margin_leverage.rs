use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Modify cross margin leverage request
#[derive(Debug, Clone, Serialize)]
pub struct ModifyCrossMarginLeverageRequest {
    /// Symbol of the contract
    pub symbol: String,
    /// New leverage value
    pub leverage: String,
}

/// Modify cross margin leverage response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyCrossMarginLeverageResponse {
    /// Success flag
    #[serde(rename = "data")]
    pub success: bool,
}

impl super::RestClient {
    /// Modify cross margin leverage for a symbol
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/positions/modify-cross-margin-leverage>
    pub async fn modify_cross_margin_leverage(
        &self,
        request: ModifyCrossMarginLeverageRequest,
    ) -> Result<(RestResponse<bool>, ResponseHeaders)> {
        const MODIFY_CROSS_MARGIN_LEVERAGE_ENDPOINT: &str = "/api/v2/changeCrossUserLeverage";
        self.post(MODIFY_CROSS_MARGIN_LEVERAGE_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_cross_margin_leverage_request_serialization() {
        let request = ModifyCrossMarginLeverageRequest {
            symbol: "XBTUSDTM".to_string(),
            leverage: "10".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("XBTUSDTM"));
        assert!(json.contains("leverage"));
        assert!(json.contains("10"));
    }

    #[test]
    fn test_modify_cross_margin_leverage_response_deserialization() {
        let json = r#"true"#;
        let response: bool = serde_json::from_str(json).unwrap();
        assert_eq!(response, true);
    }
}
