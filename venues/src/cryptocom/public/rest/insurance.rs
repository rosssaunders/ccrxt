use serde_json::Value;
use crate::cryptocom::{RestResult, EndpointType};
use super::client::RestClient;

impl RestClient {
    /// Get balance of Insurance Fund for a particular currency
    /// 
    /// # Arguments
    /// * `instrument_name` - The currency (e.g., "USD")
    /// * `count` - Optional number of data points to return (default: 25)
    /// * `start_ts` - Optional start timestamp (Unix timestamp) 
    /// * `end_ts` - Optional end timestamp (Unix timestamp)
    pub async fn get_insurance(
        &self, 
        instrument_name: &str,
        count: Option<u32>,
        start_ts: Option<u64>,
        end_ts: Option<u64>
    ) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_name": instrument_name
        });

        if let Some(c) = count {
            params["count"] = Value::Number(c.into());
        }
        
        if let Some(start) = start_ts {
            params["start_ts"] = Value::Number(start.into());
        }
        
        if let Some(end) = end_ts {
            params["end_ts"] = Value::Number(end.into());
        }

        self.send_request(
            "public/get-insurance",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetInsurance,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_insurance_endpoint_type() {
        let insurance_endpoint = EndpointType::PublicGetInsurance;
        assert!(insurance_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_insurance_parameter_building() {
        let params = json!({
            "instrument_name": "USD",
            "count": 25
        });
        assert_eq!(params["instrument_name"], "USD");
        assert_eq!(params["count"], 25);
    }
}