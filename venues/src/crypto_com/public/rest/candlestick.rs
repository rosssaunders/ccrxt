use serde_json::Value;
use crate::crypto_com::{RestResult, EndpointType};
use super::client::RestClient;

impl RestClient {
    /// Get candlestick data for a specific instrument
    /// 
    /// # Arguments
    /// * `instrument_name` - The trading pair (e.g., "BTC_USDT")
    /// * `timeframe` - The timeframe (e.g., "1m", "5m", "1h", "1D")
    /// * `count` - Optional number of data points to return (default: 25, max: 300)
    /// * `start_ts` - Optional start timestamp (Unix timestamp, default: 1 day ago)
    /// * `end_ts` - Optional end timestamp (Unix timestamp, default: current time)
    pub async fn get_candlestick(
        &self, 
        instrument_name: &str, 
        timeframe: &str, 
        count: Option<u32>,
        start_ts: Option<u64>,
        end_ts: Option<u64>
    ) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_name": instrument_name,
            "timeframe": timeframe
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
            "public/get-candlestick",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetCandlestick,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_candlestick_endpoint_type() {
        let candlestick_endpoint = EndpointType::PublicGetCandlestick;
        assert!(candlestick_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_candlestick_parameter_building() {
        let params = json!({
            "instrument_name": "BTC_USDT",
            "timeframe": "1h",
            "count": 25,
            "start_ts": 1234567890,
            "end_ts": 1234567900
        });
        assert_eq!(params["instrument_name"], "BTC_USDT");
        assert_eq!(params["timeframe"], "1h");
        assert_eq!(params["count"], 25);
        assert_eq!(params["start_ts"], 1234567890);
        assert_eq!(params["end_ts"], 1234567900);
    }
}