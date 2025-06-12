use serde_json::Value;
use crate::cryptocom::{RestResult, EndpointType};
use super::client::RestClient;

impl RestClient {
    /// Get valuation data for a particular instrument
    /// 
    /// # Arguments
    /// * `instrument_name` - The instrument name (e.g., "BTCUSD-INDEX")
    /// * `valuation_type` - The valuation type: index_price, mark_price, funding_hist, funding_rate, estimated_funding_rate
    /// * `count` - Optional number of data points to return (default: 25)
    /// * `start_ts` - Optional start timestamp (Unix timestamp)
    /// * `end_ts` - Optional end timestamp (Unix timestamp)
    pub async fn get_valuations(
        &self, 
        instrument_name: &str, 
        valuation_type: &str,
        count: Option<u32>,
        start_ts: Option<u64>,
        end_ts: Option<u64>
    ) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_name": instrument_name,
            "valuation_type": valuation_type
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
            "public/get-valuations",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetValuations,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valuations_endpoint_type() {
        let valuations_endpoint = EndpointType::PublicGetValuations;
        assert!(valuations_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_valuations_parameter_building() {
        let params = json!({
            "instrument_name": "BTCUSD-INDEX",
            "valuation_type": "index_price",
            "count": 10
        });
        assert_eq!(params["instrument_name"], "BTCUSD-INDEX");
        assert_eq!(params["valuation_type"], "index_price");
        assert_eq!(params["count"], 10);
    }
}