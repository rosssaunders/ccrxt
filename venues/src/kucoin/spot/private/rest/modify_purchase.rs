use serde::Serialize;

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const MODIFY_PURCHASE_ENDPOINT: &str = "/api/v1/otc/loan/purchase";

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
            crate::kucoin::spot::ApiError::JsonParsing(format!(
                "Failed to serialize request: {}",
                e
            ))
        })?;

        let (response, headers): (RestResponse<String>, ResponseHeaders) =
            self.post(MODIFY_PURCHASE_ENDPOINT, &body).await?;

        Ok((response.data, headers))
    }
}
