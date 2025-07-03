#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redeem_request_creation() {
        let request = RedeemRequest {
            currency: "BTC".to_string(),
            size: "0.05".to_string(),
            purchase_order_no: "123456".to_string(),
        };
        assert_eq!(request.currency, "BTC");
        assert_eq!(request.size, "0.05");
        assert_eq!(request.purchase_order_no, "123456");
    }
}
use serde::{Deserialize, Serialize};
use crate::kucoin::{ResponseHeaders, RestResponse, Result};
use super::RestClient;

/// Request for redeeming a loan order
#[derive(Debug, Clone, Serialize)]
pub struct RedeemRequest {
    /// Currency
    pub currency: String,
    /// Redemption amount
    pub size: String,
    /// Purchase order ID
    #[serde(rename = "purchaseOrderNo")]
    pub purchase_order_no: String,
}

/// Response for redeem request
#[derive(Debug, Clone, Deserialize)]
pub struct RedeemResponse {
    /// Redeem order ID
    #[serde(rename = "orderNo")]
    pub order_no: String,
}

impl RestClient {
    /// Redeem your loan order
    ///
    /// Reference: https://docs.kucoin.com/margin-credit#redeem
    pub async fn redeem(
        &self,
        request: RedeemRequest,
    ) -> Result<(RedeemResponse, ResponseHeaders)> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::kucoin::ApiError::JsonParsing(format!("Failed to serialize request: {}", e))
        })?;

        let (response, headers): (RestResponse<RedeemResponse>, ResponseHeaders) =
            self.post("/api/v3/redeem", &body).await?;

        Ok((response.data, headers))
    }
}
