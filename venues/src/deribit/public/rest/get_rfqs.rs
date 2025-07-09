//! Implements the /public/get_rfqs endpoint for Deribit.
//!
//! Retrieves active RFQs for instruments in given currency.

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{EndpointType, JsonRpcResult, RestResult, enums::InstrumentKind};

const RFQS_ENDPOINT: &str = "public/get_rfqs";

/// Request parameters for the get_rfqs endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetRfqsRequest {
    /// The currency symbol.
    #[serde(rename = "currency")]
    pub currency: String,

    /// Instrument kind, if not provided instruments of all kinds are considered.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<InstrumentKind>,
}

/// Represents a single RFQ entry.
#[derive(Debug, Clone, Deserialize)]
pub struct RfqEntry {
    /// It represents the requested order size. For perpetual and inverse futures the amount is in USD units. For options and linear futures and it is the underlying base currency coin.
    #[serde(rename = "amount")]
    pub amount: f64,

    /// Unique instrument identifier.
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,

    /// The timestamp of last RFQ (milliseconds since the Unix epoch).
    #[serde(rename = "last_rfq_timestamp")]
    pub last_rfq_timestamp: u64,

    /// Side - buy or sell.
    #[serde(rename = "side")]
    pub side: String,

    /// Volume traded since last RFQ.
    #[serde(rename = "traded_volume")]
    pub traded_volume: f64,
}

/// Response for the get_rfqs endpoint.
pub type GetRfqsResponse = JsonRpcResult<Vec<RfqEntry>>;

impl RestClient {
    /// Calls the /public/get_rfqs endpoint.
    ///
    /// Retrieves active RFQs for instruments in given currency.
    ///
    /// [Official API docs](https://docs.deribit.com/#public-get_rfqs)
    pub async fn get_rfqs(&self, params: GetRfqsRequest) -> RestResult<GetRfqsResponse> {
        self.send_request(
            RFQS_ENDPOINT,
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
    use crate::deribit::enums::InstrumentKind;

    #[test]
    fn test_serialize_request() {
        let req = GetRfqsRequest {
            currency: "BTC".to_string(),
            kind: Some(InstrumentKind::Future),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("future"));
    }

    #[test]
    fn test_deserialize_response() {
        let data = r#"{
            "id": 25,
            "jsonrpc": "2.0",
            "result": [
                {
                    "amount": 1000,
                    "instrument_name": "BTC-PERPETUAL",
                    "last_rfq_timestamp": 1680310800000,
                    "side": "buy",
                    "traded_volume": 50
                }
            ]
        }"#;
        let resp: GetRfqsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.id, 25);
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.result.len(), 1);
        let rfq = &resp.result[0];
        assert_eq!(rfq.amount, 1000.0);
        assert_eq!(rfq.instrument_name, "BTC-PERPETUAL");
        assert_eq!(rfq.side, "buy");
        assert_eq!(rfq.last_rfq_timestamp, 1680310800000);
        assert_eq!(rfq.traded_volume, 50.0);
    }
}
