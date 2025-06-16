use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::client::RestClient;
use crate::cryptocom::RestResult;

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

impl RestClient {
    /// Cancel OCO orders
    ///
    /// Cancels a contingency order (OCO) using the list ID.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 10 requests per second per user
    ///
    /// # Arguments
    /// * `request` - The OCO order cancellation request
    ///
    /// # Returns
    /// Response confirming the cancellation request
    pub async fn cancel_oco_order(&self, request: CancelOcoOrderRequest) -> crate::cryptocom::RestResult<serde_json::Value> {
        let params = serde_json::to_value(&request)?;
        self.send_signed_request("private/cancel-order-list", params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
