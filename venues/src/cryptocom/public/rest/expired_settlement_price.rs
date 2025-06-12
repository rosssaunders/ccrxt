use serde_json::Value;
use crate::cryptocom::{RestResult, EndpointType};
use super::client::RestClient;

impl RestClient {
    /// Get settlement price of expired instruments
    /// 
    /// # Arguments
    /// * `instrument_type` - The instrument type (e.g., "FUTURE")
    /// * `page` - Optional page number (default: 1)
    pub async fn get_expired_settlement_price(&self, instrument_type: &str, page: Option<u32>) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_type": instrument_type
        });

        if let Some(p) = page {
            params["page"] = Value::Number(p.into());
        }

        self.send_request(
            "public/get-expired-settlement-price",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetExpiredSettlementPrice,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_expired_settlement_price_endpoint_type() {
        let expired_settlement_endpoint = EndpointType::PublicGetExpiredSettlementPrice;
        assert!(expired_settlement_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_expired_settlement_price_parameter_building() {
        let params = json!({
            "instrument_type": "FUTURE",
            "page": 1
        });
        assert_eq!(params["instrument_type"], "FUTURE");
        assert_eq!(params["page"], 1);
    }
}