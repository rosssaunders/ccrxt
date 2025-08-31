use serde::Serialize;

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result, private_client::RestClient};

const MODIFY_PURCHASE_ENDPOINT: &str = "/api/v3/lend/purchase/update";

/// Request for modifying a purchase order
#[derive(Debug, Clone, Serialize)]
pub struct ModifyPurchaseRequest {
    /// Currency
    pub currency: String,

    /// Modified purchase interest rate
    #[serde(rename = "interestRate")]
    pub interest_rate: String,

    /// Purchase order ID
    #[serde(rename = "purchaseOrderNo")]
    pub purchase_order_no: String,
}

impl RestClient {
    /// Modify Purchase
    ///
    /// Update the interest rates of subscription orders, which will take effect at the beginning of the next hour.
    ///
    /// - [docs](https://www.kucoin.com/docs-new/rest/margin-trading/credit/modify-purchase)
    ///
    /// Rate limit: weight 10 (Private)
    ///
    /// # Arguments
    /// * `request` - Currency, purchase order ID, and new interest rate
    ///
    /// # Returns
    /// Empty response body on success and response headers
    pub async fn modify_purchase(
        &self,
        request: ModifyPurchaseRequest,
    ) -> Result<(String, ResponseHeaders)> {
        let (response, headers): (RestResponse<String>, ResponseHeaders) = self
            .post_with_request(MODIFY_PURCHASE_ENDPOINT, &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(MODIFY_PURCHASE_ENDPOINT, "/api/v3/lend/purchase/update");
    }

    #[test]
    fn test_request_serialization() {
        let req = ModifyPurchaseRequest {
            currency: "BTC".to_string(),
            interest_rate: "0.09".to_string(),
            purchase_order_no: "abc123".to_string(),
        };
        let val = serde_json::to_value(&req).unwrap();
        assert_eq!(val["currency"], "BTC");
        assert_eq!(val["interestRate"], "0.09");
        assert_eq!(val["purchaseOrderNo"], "abc123");
    }
}
