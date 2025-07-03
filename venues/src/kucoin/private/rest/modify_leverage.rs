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
use crate::kucoin::{ResponseHeaders, RestResponse, Result};
use super::RestClient;
use serde::{Deserialize, Serialize};

/// Request for modifying leverage
#[derive(Debug, Clone, Serialize)]
pub struct ModifyLeverageRequest {
    pub symbol: Option<String>,
    pub is_isolated: Option<bool>,
    pub leverage: String,
}

impl RestClient {
    /// Modify leverage
    ///
    /// This endpoint allows modifying the leverage multiplier for cross margin or isolated margin.
    pub async fn modify_leverage(
        &self,
        request: ModifyLeverageRequest,
    ) -> Result<(String, ResponseHeaders)> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::kucoin::ApiError::JsonParsing(format!("Failed to serialize request: {}", e))
        })?;
        let (response, headers): (RestResponse<String>, ResponseHeaders) = self
            .post("/api/v3/position/update-user-leverage", &body)
            .await?;
        Ok((response.data, headers))
    }
}
