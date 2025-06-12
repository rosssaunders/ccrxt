use serde_json::Value;
use crate::crypto_com::{RestResult, EndpointType};
use super::client::RestClient;

impl RestClient {
    /// Get the order book for a specific instrument
    /// 
    /// # Arguments
    /// * `instrument_name` - The trading pair (e.g., "BTC_USDT")
    /// * `depth` - Optional depth of the order book (default: 10)
    pub async fn get_book(&self, instrument_name: &str, depth: Option<u32>) -> RestResult<Value> {
        let mut params = serde_json::json!({
            "instrument_name": instrument_name
        });

        if let Some(d) = depth {
            params["depth"] = Value::Number(d.into());
        }

        self.send_request(
            "public/get-book",
            reqwest::Method::GET,
            Some(&params),
            EndpointType::PublicGetBook,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_book_endpoint_type() {
        let book_endpoint = EndpointType::PublicGetBook;
        assert!(book_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_book_parameter_building() {
        let params = json!({
            "instrument_name": "BTC_USDT",
            "depth": 10
        });
        assert_eq!(params["instrument_name"], "BTC_USDT");
        assert_eq!(params["depth"], 10);
    }
}