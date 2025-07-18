use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Get cross margin leverage request
#[derive(Debug, Clone, Serialize)]
pub struct GetCrossMarginLeverageRequest {
    /// Symbol of the contract
    pub symbol: String,
}

/// Cross margin leverage response
#[derive(Debug, Clone, Deserialize)]
pub struct GetCrossMarginLeverageResponse {
    /// Contract symbol
    pub symbol: String,
    /// Current leverage value
    pub leverage: String,
}

impl super::RestClient {
    /// Get current cross margin leverage for a symbol
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-cross-margin-leverage>
    pub async fn get_cross_margin_leverage(
        &self,
        request: GetCrossMarginLeverageRequest,
    ) -> Result<(
        RestResponse<GetCrossMarginLeverageResponse>,
        ResponseHeaders,
    )> {
        const GET_CROSS_MARGIN_LEVERAGE_ENDPOINT: &str = "/api/v2/getCrossUserLeverage";
        self.get(GET_CROSS_MARGIN_LEVERAGE_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cross_margin_leverage_request_creation() {
        let request = GetCrossMarginLeverageRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_get_cross_margin_leverage_response_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "leverage": "3"
        }"#;

        let response: GetCrossMarginLeverageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "XBTUSDTM");
        assert_eq!(response.leverage, "3");
    }
}
