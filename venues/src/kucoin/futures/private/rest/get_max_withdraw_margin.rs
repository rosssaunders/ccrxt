use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Get max withdraw margin request
#[derive(Debug, Clone, Serialize)]
pub struct GetMaxWithdrawMarginRequest {
    /// Symbol of the contract
    pub symbol: String,
}

/// Response for getting maximum withdrawable margin
#[derive(Debug, Clone, Deserialize)]
pub struct GetMaxWithdrawMarginResponse {
    /// Maximum withdrawable margin amount
    #[serde(rename = "data")]
    pub max_withdraw_margin: String,
}

impl super::RestClient {
    /// Get maximum withdrawable margin for a position
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-max-withdraw-margin>
    pub async fn get_max_withdraw_margin(
        &self,
        request: GetMaxWithdrawMarginRequest,
    ) -> Result<(RestResponse<String>, ResponseHeaders)> {
        const GET_MAX_WITHDRAW_MARGIN_ENDPOINT: &str = "/api/v1/margin/maxWithdrawMargin";
        
        let mut params = std::collections::HashMap::new();
        params.insert("symbol".to_string(), request.symbol);
        
        self.get(GET_MAX_WITHDRAW_MARGIN_ENDPOINT, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_withdraw_margin_request_creation() {
        let request = GetMaxWithdrawMarginRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_get_max_withdraw_margin_response_deserialization() {
        let json = r#"21.1135719252"#;
        let response: String = serde_json::from_str(json).unwrap();
        assert_eq!(response, "21.1135719252");
    }
}