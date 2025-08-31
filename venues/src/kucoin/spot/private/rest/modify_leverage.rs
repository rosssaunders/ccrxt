use serde::Serialize;

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result, private_client::RestClient};

// Endpoint constant for Modify Leverage
const MODIFY_LEVERAGE_ENDPOINT: &str = "/api/v3/position/update-user-leverage";

/// Request for modifying leverage
#[derive(Debug, Clone, Serialize)]
pub struct ModifyLeverageRequest {
    pub symbol: Option<String>,

    pub is_isolated: Option<bool>,

    pub leverage: String,
}

impl RestClient {
    /// Modify Leverage
    ///
    /// This endpoint allows modifying the leverage multiplier for cross margin or isolated margin.
    ///
    /// - [docs](https://www.kucoin.com/docs-new/rest/margin-trading/debit/modify-leverage)
    ///
    /// Rate limit: weight 8 (Private)
    ///
    /// # Arguments
    /// * `request` - Leverage modification parameters
    ///
    /// # Returns
    /// Empty response body with headers; success indicated via HTTP status and code
    pub async fn modify_leverage(
        &self,
        request: ModifyLeverageRequest,
    ) -> Result<(String, ResponseHeaders)> {
        let (response, headers): (RestResponse<String>, ResponseHeaders) = self
            .post_with_request(MODIFY_LEVERAGE_ENDPOINT, &request)
            .await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_leverage_request_creation() {
        let request = ModifyLeverageRequest {
            symbol: Some("BTC-USDT".to_string()),
            is_isolated: Some(true),
            leverage: "5.0".to_string(),
        };

        assert_eq!(request.symbol, Some("BTC-USDT".to_string()));
        assert_eq!(request.is_isolated, Some(true));
        assert_eq!(request.leverage, "5.0");
    }
}
