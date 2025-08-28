use serde::{Deserialize, Serialize};

use crate::cryptocom::{ApiResult, PrivateRestClient as RestClient, RestResult};

/// Endpoint path for the cancel-oco-order API
const CANCEL_OCO_ORDER_ENDPOINT: &str = "exchange/v1/private/cancel-order-list";

/// Request for canceling OCO orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOcoOrderRequest {
    /// Contingency type (must be OCO)
    pub contingency_type: crate::cryptocom::ContingencyType,

    /// List ID of the OCO order to cancel
    pub list_id: String,

    /// Instrument name
    pub instrument_name: String,
}

/// Result data for canceling OCO orders
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOcoOrderResult {
    /// The list ID of the canceled OCO order
    pub list_id: String,
}

/// Response wrapper for endpoint
pub type CancelOcoOrderResponse = ApiResult<CancelOcoOrderResult>;

impl RestClient {
    /// Cancel OCO orders
    ///
    /// Cancels a contingency order (OCO) using the list ID.
    ///
    /// [docs](https://exchange-docs.crypto.com/derivatives/index.html)
    ///
    /// Rate limit: 10 requests per second per user
    ///
    /// # Arguments
    /// * `request` - The OCO order cancellation request
    ///
    /// # Returns
    /// Response confirming the cancellation request
    pub async fn cancel_oco_order(
        &self,
        request: CancelOcoOrderRequest,
    ) -> RestResult<CancelOcoOrderResponse> {
        self.send_signed_request(CANCEL_OCO_ORDER_ENDPOINT, request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_cancel_oco_order_request_structure() {
        let request_json = json!({
            "contingency_type": "OCO",
            "list_id": "4421958062479290999",
            "instrument_name": "BTCUSD-PERP"
        });
        let request: CancelOcoOrderRequest = serde_json::from_value(request_json).unwrap();
        assert_eq!(
            request.contingency_type,
            crate::cryptocom::enums::ContingencyType::Oco
        );
        assert_eq!(request.list_id, "4421958062479290999");
        assert_eq!(request.instrument_name, "BTCUSD-PERP");
    }
}
