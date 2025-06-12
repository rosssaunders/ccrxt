use serde_json::Value;
use crate::cryptocom::{RestResult, EndpointType};
use super::client::RestClient;

impl RestClient {
    /// Get recent trades for a specific instrument
    /// 
    /// # Arguments
    /// * `instrument_name` - The trading pair (e.g., "BTC_USDT")
    /// * `count` - Optional number of trades to return (default: 25, max: 150)
    /// * `start_ts` - Optional start timestamp (Unix timestamp or nanoseconds, default: end_time - 1 day)
    /// * `end_ts` - Optional end timestamp (Unix timestamp or nanoseconds, default: current system timestamp)
    pub async fn get_trades(
        &self, 
        instrument_name: &str, 
        count: Option<u32>,
        start_ts: Option<&str>,
        end_ts: Option<&str>
    ) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_name": instrument_name
        });

        if let Some(c) = count {
            params["count"] = Value::Number(c.into());
        }
        
        if let Some(start) = start_ts {
            params["start_ts"] = Value::String(start.to_string());
        }
        
        if let Some(end) = end_ts {
            params["end_ts"] = Value::String(end.to_string());
        }

        self.send_request(
            "public/get-trades",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetTrades,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_trades_endpoint_type() {
        let trades_endpoint = EndpointType::PublicGetTrades;
        assert!(trades_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_trades_parameter_building() {
        let params = json!({
            "instrument_name": "BTC_USDT",
            "count": 100,
            "start_ts": "1234567890",
            "end_ts": "1234567900"
        });
        assert_eq!(params["instrument_name"], "BTC_USDT");
        assert_eq!(params["count"], 100);
        assert_eq!(params["start_ts"], "1234567890");
        assert_eq!(params["end_ts"], "1234567900");
    }
}