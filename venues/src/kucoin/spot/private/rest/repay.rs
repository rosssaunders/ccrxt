use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const REPAY_ENDPOINT: &str = "/api/v3/margin/repay";

/// Request for repay
#[derive(Debug, Clone, Serialize)]
pub struct RepayRequest {
    pub currency: String,

    pub size: String,

    pub symbol: Option<String>,

    pub is_isolated: Option<bool>,

    pub is_hf: Option<bool>,
}

/// Response data for repay operation
#[derive(Debug, Clone, Deserialize)]
pub struct RepayResponse {
    #[serde(rename = "orderNo")]
    pub order_no: String,
}

impl RestClient {
    /// Repay margin
    ///
    /// This API endpoint is used to initiate an application for cross or isolated margin repayment.
    pub async fn repay(&self, request: RepayRequest) -> Result<(RepayResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<RepayResponse>, ResponseHeaders) =
            self.post_with_request(REPAY_ENDPOINT, &request).await?;
        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repay_request_creation() {
        let request = RepayRequest {
            currency: "USDT".to_string(),
            size: "50.0".to_string(),
            symbol: None,
            is_isolated: Some(false),
            is_hf: Some(false),
        };

        assert_eq!(request.currency, "USDT");
        assert_eq!(request.size, "50.0");
        assert_eq!(request.symbol, None);
        assert_eq!(request.is_isolated, Some(false));
        assert_eq!(request.is_hf, Some(false));
    }
}
