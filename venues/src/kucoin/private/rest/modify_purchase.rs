use serde::{Deserialize, Serialize};
use crate::kucoin::{ResponseHeaders, RestResponse, Result};
use super::RestClient;

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
    /// Update the interest rates of subscription orders
    ///
    /// Reference: https://docs.kucoin.com/margin-credit#modify-purchase
    pub async fn modify_purchase(
        &self,
        request: ModifyPurchaseRequest,
    ) -> Result<(String, ResponseHeaders)> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::kucoin::ApiError::JsonParsing(format!("Failed to serialize request: {}", e))
        })?;

        let (response, headers): (RestResponse<String>, ResponseHeaders) =
            self.post("/api/v3/lend/purchase/update", &body).await?;

        Ok((response.data, headers))
    }
}
