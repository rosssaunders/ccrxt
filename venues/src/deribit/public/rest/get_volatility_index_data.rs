use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, Resolution, RestResult};

const VOLATILITY_INDEX_DATA_ENDPOINT: &str = "public/get_volatility_index_data";

/// Request parameters for the get_volatility_index_data endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetVolatilityIndexDataRequest {
    /// The currency symbol (e.g., BTC, ETH, USDC, USDT, EURR).
    #[serde(rename = "currency")]
    pub currency: Cow<'static, str>,

    /// The earliest timestamp to return result from (milliseconds since the UNIX epoch).
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: u64,

    /// The most recent timestamp to return result from (milliseconds since the UNIX epoch).
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: u64,

    /// Time resolution given in full seconds or keyword 1D (only some specific resolutions are supported).
    #[serde(rename = "resolution")]
    pub resolution: Resolution,
}

/// The result object for get_volatility_index_data.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GetVolatilityIndexDataResult {
    /// Array of volatility index data points.
    /// Candles as an array of arrays with 5 values each.
    /// The inner values correspond to the timestamp in
    ///
    /// ms
    /// open
    /// high
    /// low
    /// close
    ///
    /// values of the volatility index correspondingly.
    #[serde(rename = "data")]
    pub data: Vec<[f64; 5]>,

    /// Continuation token for pagination (null if no more data).
    #[serde(rename = "continuation")]
    pub continuation: Option<String>,
}

impl GetVolatilityIndexDataResult {
    /// Returns an iterator over the timestamps (ms) of the data points.
    pub fn timestamps(&self) -> impl Iterator<Item = f64> + '_ {
        self.data.iter().map(|arr| arr[0])
    }

    /// Returns an iterator over the open values.
    pub fn opens(&self) -> impl Iterator<Item = f64> + '_ {
        self.data.iter().map(|arr| arr[1])
    }

    /// Returns an iterator over the high values.
    pub fn highs(&self) -> impl Iterator<Item = f64> + '_ {
        self.data.iter().map(|arr| arr[2])
    }

    /// Returns an iterator over the low values.
    pub fn lows(&self) -> impl Iterator<Item = f64> + '_ {
        self.data.iter().map(|arr| arr[3])
    }

    /// Returns an iterator over the close values.
    pub fn closes(&self) -> impl Iterator<Item = f64> + '_ {
        self.data.iter().map(|arr| arr[4])
    }

    /// Returns an iterator over all values as tuples: (timestamp, open, high, low, close)
    pub fn as_tuples(&self) -> impl Iterator<Item = (f64, f64, f64, f64, f64)> + '_ {
        self.data
            .iter()
            .map(|arr| (arr[0], arr[1], arr[2], arr[3], arr[4]))
    }
}

/// Response for public/get_volatility_index_data endpoint following Deribit JSON-RPC 2.0 format.
pub type GetVolatilityIndexDataResponse = JsonRpcResult<GetVolatilityIndexDataResult>;

impl RestClient {
    /// Calls the /public/get_volatility_index_data endpoint.
    ///
    /// Retrieves volatility index data for a given index name.
    ///
    /// [docs](https://docs.deribit.com/#public-get_volatility_index_data)
    pub async fn get_volatility_index_data(
        &self,
        params: GetVolatilityIndexDataRequest,
    ) -> RestResult<GetVolatilityIndexDataResponse> {
        self.send_post_request(
            VOLATILITY_INDEX_DATA_ENDPOINT,
            Some(&params),
            EndpointType::NonMatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = GetVolatilityIndexDataRequest {
            currency: Cow::Borrowed("BTC"),
            start_timestamp: 1680307200000,
            end_timestamp: 1680310800000,
            resolution: Resolution::OneHour,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("3600"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 21,
            "jsonrpc": "2.0",
            "result": {
                "data": [
                [1598019300000,0.210084879,0.212860821,0.210084879,0.212860821],
                [1598019360000,0.212869011,0.212987527,0.212869011,0.212987527],
                [1598019420000,0.212987723,0.212992597,0.212987723,0.212992597]
                ],
                "continuation": null
        }
        }"#;

        let resp: GetVolatilityIndexDataResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 21);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.data.len(), 3);
        assert_eq!(resp.result.data[0][0], 1598019300000.0);
        assert!((resp.result.data[0][1] - 0.210084879).abs() < 1e-8);
        assert!((resp.result.data[0][4] - 0.212860821).abs() < 1e-8);
        assert_eq!(resp.result.continuation, None);
    }
}
