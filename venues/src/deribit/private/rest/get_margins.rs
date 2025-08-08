use serde::{Deserialize, Serialize};

use crate::deribit::{EndpointType, JsonRpcResult, RestResult};

/// REST API endpoint constant
const GET_MARGINS_ENDPOINT: &str = "private/get_margins";

/// Request parameters for the `/private/get_margins` endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMarginsRequest {
    /// Instrument name (e.g., "BTC-PERPETUAL").
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// Order size requested. For perpetual/inverse futures, this is in USD units; for options and linear futures, the base currency coin.
    #[serde(rename = "amount")]
    pub amount: f64,

    /// Price at which to calculate margin.
    #[serde(rename = "price")]
    pub price: f64,
}

/// Margin requirements returned by `/private/get_margins`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMarginsResult {
    /// Margin required to open a buy (long) position at this price and size.
    #[serde(rename = "buy")]
    pub buy: f64,

    /// Maximum allowed order price for a future. Orders above this are clamped to this max.
    #[serde(rename = "max_price")]
    pub max_price: f64,

    /// Minimum allowed order price for a future. Orders below this are clamped to this min.
    #[serde(rename = "min_price")]
    pub min_price: f64,

    /// Margin required to open a sell (short) position at this price and size.
    #[serde(rename = "sell")]
    pub sell: f64,
}

/// JSON-RPC standard response structure for `/private/get_margins`.
pub type GetMarginsResponse = JsonRpcResult<GetMarginsResult>;

/// Implementation for calling the endpoint from the REST client.
impl super::client::RestClient {
    /// Fetches margin requirements for a given instrument, amount, and price.
    ///
    /// [Official Deribit Docs](https://docs.deribit.com/v2/#private-get_margins)
    pub async fn get_margins(&self, request: GetMarginsRequest) -> RestResult<GetMarginsResponse> {
        self.send_signed_request(GET_MARGINS_ENDPOINT, &request, EndpointType::MatchingEngine)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    /// REST API endpoint constant
    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = GetMarginsRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            amount: 100.0,
            price: 25000.0,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("instrument_name"));
        assert!(json.contains("amount"));
        assert!(json.contains("price"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"
        {
            "jsonrpc": "2.0",
            "id": 42,
            "result": {
                "buy": 10.0,
                "max_price": 50000.0,
                "min_price": 21000.0,
                "sell": 9.0
            }
        }
        "#;
        let resp: GetMarginsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 42);
        assert_eq!(resp.result.buy, 10.0);
        assert_eq!(resp.result.sell, 9.0);
    }
}
