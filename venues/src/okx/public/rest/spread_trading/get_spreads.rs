use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const GET_SPREADS_ENDPOINT: &str = "/api/v5/sprd/spreads";

/// Request parameters for the get spreads request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpreadsRequest {
    /// Base currency, e.g. BTC
    #[serde(rename = "baseCcy", skip_serializing_if = "Option::is_none")]
    pub base_ccy: Option<String>,

    /// Spread ID
    #[serde(rename = "sprdId", skip_serializing_if = "Option::is_none")]
    pub sprd_id: Option<String>,

    /// Spread state
    /// live: Trading
    /// suspend: Suspended
    /// preopen: Preopen
    /// expired: Expired
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Response data for the get spreads request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadData {
    /// Base currency, e.g. BTC
    #[serde(rename = "baseCcy")]
    pub base_ccy: String,

    /// Spread ID
    #[serde(rename = "sprdId")]
    pub sprd_id: String,

    /// Spread type (linear or inverse)
    #[serde(rename = "sprdType")]
    pub sprd_type: String,

    /// Spread state
    /// live: Trading
    /// suspend: Suspended
    /// preopen: Preopen
    /// expired: Expired
    #[serde(rename = "state")]
    pub state: String,

    /// Size currency
    #[serde(rename = "szCcy")]
    pub sz_ccy: String,

    /// Quote currency
    #[serde(rename = "quoteCcy")]
    pub quote_ccy: String,

    /// Minimum size
    #[serde(rename = "minSz")]
    pub min_sz: String,

    /// Tick size
    #[serde(rename = "tickSz")]
    pub tick_sz: String,

    /// Lot size
    #[serde(rename = "lotSz")]
    pub lot_sz: String,

    /// Listing time, Unix timestamp format in milliseconds
    #[serde(rename = "listTime")]
    pub list_time: String,

    /// Expiry time, Unix timestamp format in milliseconds
    #[serde(rename = "expTime")]
    pub exp_time: String,

    /// Update time, Unix timestamp format in milliseconds
    #[serde(rename = "uTime")]
    pub u_time: String,

    /// Spread legs
    #[serde(rename = "legs")]
    pub legs: Vec<SpreadLeg>,
}

/// Spread leg information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadLeg {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Side
    /// buy: Buy (long)
    /// sell: Sell (short)
    #[serde(rename = "side")]
    pub side: String,
}

impl RestClient {
    /// Get spreads
    ///
    /// Retrieve all available spreads for trading
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-get-spreads-public)
    pub async fn get_spreads(&self, request: Option<GetSpreadsRequest>) -> RestResult<SpreadData> {
        self.send_get_request(
            GET_SPREADS_ENDPOINT,
            request.as_ref(),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_spreads_request_serialization() {
        let request = GetSpreadsRequest {
            base_ccy: Some("BTC".to_string()),
            sprd_id: Some("BTC-USDT_BTC-USDT-SWAP".to_string()),
            state: Some("live".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetSpreadsRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_spreads_request_minimal() {
        let request = GetSpreadsRequest {
            base_ccy: None,
            sprd_id: None,
            state: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(serialized, "{}");
    }

    #[test]
    fn test_spread_data_serialization() {
        let spread_leg = SpreadLeg {
            inst_id: "BTC-USDT".to_string(),
            side: "buy".to_string(),
        };

        let spread_data = SpreadData {
            base_ccy: "BTC".to_string(),
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            sprd_type: "linear".to_string(),
            state: "live".to_string(),
            sz_ccy: "BTC".to_string(),
            quote_ccy: "USDT".to_string(),
            min_sz: "0.01".to_string(),
            tick_sz: "0.1".to_string(),
            lot_sz: "0.01".to_string(),
            list_time: "1597026383085".to_string(),
            exp_time: "1597026383085".to_string(),
            u_time: "1597026383085".to_string(),
            legs: vec![spread_leg],
        };

        let serialized = serde_json::to_string(&spread_data).unwrap();
        let deserialized: SpreadData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(spread_data, deserialized);
    }

    #[test]
    fn test_spread_data_deserialization_from_api() {
        let json_response = r#"{
            "baseCcy": "BTC",
            "sprdId": "BTC-USDT_BTC-USDT-SWAP",
            "sprdType": "linear",
            "state": "live",
            "szCcy": "BTC",
            "quoteCcy": "USDT",
            "minSz": "0.01",
            "tickSz": "0.1",
            "lotSz": "0.01",
            "listTime": "1597026383085",
            "expTime": "1597026383085",
            "uTime": "1597026383085",
            "legs": [
                {
                    "instId": "BTC-USDT",
                    "side": "buy"
                },
                {
                    "instId": "BTC-USDT-SWAP", 
                    "side": "sell"
                }
            ]
        }"#;

        let spread_data: SpreadData = serde_json::from_str(json_response).unwrap();
        assert_eq!(spread_data.base_ccy, "BTC");
        assert_eq!(spread_data.sprd_id, "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(spread_data.sprd_type, "linear");
        assert_eq!(spread_data.state, "live");
        assert_eq!(spread_data.sz_ccy, "BTC");
        assert_eq!(spread_data.quote_ccy, "USDT");
        assert_eq!(spread_data.legs.len(), 2);
        assert_eq!(spread_data.legs[0].inst_id, "BTC-USDT");
        assert_eq!(spread_data.legs[0].side, "buy");
        assert_eq!(spread_data.legs[1].inst_id, "BTC-USDT-SWAP");
        assert_eq!(spread_data.legs[1].side, "sell");
    }

    #[test]
    fn test_spread_states() {
        let states = vec!["live", "suspend", "preopen", "expired"];

        for state in states {
            let request = GetSpreadsRequest {
                base_ccy: None,
                sprd_id: None,
                state: Some(state.to_string()),
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"state\":\"{}\"", state)));
        }
    }
}
