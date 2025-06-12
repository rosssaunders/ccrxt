use serde_json::Value;
use crate::cryptocom::{RestResult, EndpointType};
use super::client::RestClient;

impl RestClient {
    /// Get ticker information for one or all instruments
    /// 
    /// # Arguments
    /// * `instrument_name` - Optional specific instrument name. If None, returns all tickers.
    pub async fn get_ticker(&self, instrument_name: Option<&str>) -> RestResult<Value> {
        let params = if let Some(instrument) = instrument_name {
            Some(serde_json::json!({
                "instrument_name": instrument
            }))
        } else {
            None
        };

        self.send_request(
            "public/get-tickers",
            reqwest::Method::GET,
            params.as_ref(),
            EndpointType::PublicGetTickers,
        ).await
    }

    /// Get ticker information for one or all instruments
    /// 
    /// # Arguments
    /// * `instrument_name` - Optional specific instrument name. If None, returns all tickers.
    pub async fn get_tickers(&self, instrument_name: Option<&str>) -> RestResult<Value> {
        let params = if let Some(instrument) = instrument_name {
            Some(serde_json::json!({
                "instrument_name": instrument
            }))
        } else {
            None
        };

        self.send_request(
            "public/get-tickers",
            reqwest::Method::GET,
            params.as_ref(),
            EndpointType::PublicGetTickers,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_ticker_endpoint_types() {
        let ticker_endpoint = EndpointType::PublicGetTicker;
        let tickers_endpoint = EndpointType::PublicGetTickers;
        
        assert!(ticker_endpoint.rate_limit().max_requests > 0);
        assert!(tickers_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_ticker_parameter_building() {
        let params = json!({
            "instrument_name": "BTC_USDT"
        });
        assert_eq!(params["instrument_name"], "BTC_USDT");
    }
}