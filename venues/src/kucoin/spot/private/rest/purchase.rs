use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const PURCHASE_ENDPOINT: &str = "/api/v3/purchase";

/// Request for purchasing/lending credit
#[derive(Debug, Clone, Serialize)]
pub struct PurchaseRequest {
    /// Currency
    pub currency: String,
    /// Purchase amount
    pub size: String,
    /// Purchase interest rate
    #[serde(rename = "interestRate")]
    pub interest_rate: String,
}

/// Response for purchase request
#[derive(Debug, Clone, Deserialize)]
pub struct PurchaseResponse {
    /// Purchase order ID
    #[serde(rename = "orderNo")]
    pub order_no: String,
}

impl RestClient {
    /// Invest credit in the market and earn interest
    ///
    /// Reference: https://docs.kucoin.com/margin-credit#purchase
    pub async fn purchase(
        &self,
        request: PurchaseRequest,
    ) -> Result<(PurchaseResponse, ResponseHeaders)> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::kucoin::spot::ApiError::JsonParsing(format!(
                "Failed to serialize request: {}",
                e
            ))
        })?;

        let (response, headers): (RestResponse<PurchaseResponse>, ResponseHeaders) =
            self.post(PURCHASE_ENDPOINT, &body).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purchase_request_creation() {
        let request = PurchaseRequest {
            currency: "BTC".to_string(),
            size: "0.1".to_string(),
            interest_rate: "0.005".to_string(),
        };
        assert_eq!(request.currency, "BTC");
        assert_eq!(request.size, "0.1");
        assert_eq!(request.interest_rate, "0.005");
    }
}
