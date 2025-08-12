use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentState, InstrumentType, RestResult};

const PUBLIC_INSTRUMENTS_ENDPOINT: &str = "api/v5/public/instruments";
/// Request parameters for getting instruments
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsRequest {
    /// Instrument type (required)
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,

    /// Underlying (for SWAP/FUTURES/OPTION)
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Instrument family (for FUTURES/SWAP/OPTION)
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,

    /// Instrument ID
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Individual instrument details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,

    /// Instrument ID (e.g., "BTC-USDT")
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Underlying (e.g., "BTC-USD")
    #[serde(rename = "uly")]
    pub underlying: Option<String>,

    /// Instrument family (e.g., "BTC-USD")
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,

    /// Category (e.g., "1")
    pub category: String,

    /// Base currency (e.g., "BTC")
    #[serde(rename = "baseCcy")]
    pub base_ccy: String,

    /// Quote currency (e.g., "USDT")
    #[serde(rename = "quoteCcy")]
    pub quote_ccy: String,

    /// Settlement currency (e.g., "BTC")
    #[serde(rename = "settleCcy")]
    pub settle_ccy: String,

    /// Contract value (for derivatives)
    #[serde(rename = "ctVal")]
    pub ct_val: Option<String>,

    /// Contract multiplier (for derivatives)
    #[serde(rename = "ctMult")]
    pub ct_mult: Option<String>,

    /// Contract value currency (for derivatives)
    #[serde(rename = "ctValCcy")]
    pub ct_val_ccy: Option<String>,

    /// Option type ("C" for call, "P" for put)
    #[serde(rename = "optType")]
    pub opt_type: Option<String>,

    /// Strike price (for options)
    #[serde(rename = "stk")]
    pub strike: Option<String>,

    /// Listing time (Unix timestamp in milliseconds)
    #[serde(rename = "listTime")]
    pub list_time: String,

    /// End time of call auction (Unix timestamp in milliseconds)
    #[serde(rename = "auctionEndTime")]
    pub auction_end_time: Option<String>,

    /// Continuous trading switch time (Unix timestamp in milliseconds)
    #[serde(rename = "contTdSwTime")]
    pub cont_td_sw_time: Option<String>,

    /// Open type ("fix_price", "pre_quote", "call_auction")
    #[serde(rename = "openType")]
    pub open_type: Option<String>,

    /// Expiry time (Unix timestamp in milliseconds)
    #[serde(rename = "expTime")]
    pub exp_time: Option<String>,

    /// Leverage (for margin trading)
    pub lever: Option<String>,

    /// Tick size
    #[serde(rename = "tickSz")]
    pub tick_sz: String,

    /// Lot size
    #[serde(rename = "lotSz")]
    pub lot_sz: String,

    /// Minimum order size
    #[serde(rename = "minSz")]
    pub min_sz: String,

    /// Contract type ("linear" or "inverse")
    #[serde(rename = "ctType")]
    pub ct_type: Option<String>,

    /// Alias ("this_week", "next_week", "quarter", "next_quarter")
    pub alias: Option<String>,

    /// Instrument state
    pub state: InstrumentState,

    /// Trading rule types ("normal", "pre_market")
    #[serde(rename = "ruleType")]
    pub rule_type: Option<String>,

    /// Maximum order quantity of a single limit order
    #[serde(rename = "maxLmtSz")]
    pub max_lmt_sz: Option<String>,

    /// Maximum order quantity of a single market order
    #[serde(rename = "maxMktSz")]
    pub max_mkt_sz: Option<String>,

    /// Max USD amount for a single limit order
    #[serde(rename = "maxLmtAmt")]
    pub max_lmt_amt: Option<String>,

    /// Max USD amount for a single market order
    #[serde(rename = "maxMktAmt")]
    pub max_mkt_amt: Option<String>,

    /// Maximum order quantity of a single TWAP order
    #[serde(rename = "maxTwapSz")]
    pub max_twap_sz: Option<String>,

    /// Maximum order quantity of a single iceberg order
    #[serde(rename = "maxIcebergSz")]
    pub max_iceberg_sz: Option<String>,

    /// Maximum order quantity of a single trigger order
    #[serde(rename = "maxTriggerSz")]
    pub max_trigger_sz: Option<String>,

    /// Maximum order quantity of a single stop market order
    #[serde(rename = "maxStopSz")]
    pub max_stop_sz: Option<String>,

    /// Whether daily settlement for expiry feature is enabled
    #[serde(rename = "futureSettlement")]
    pub future_settlement: Option<bool>,
}

impl RestClient {
    /// Get instruments
    ///
    /// Retrieve a list of instruments with open contracts.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-instruments
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The instruments request parameters
    ///
    /// # Returns
    /// Response containing the list of available instruments
    pub async fn get_instruments(&self, request: GetInstrumentsRequest) -> RestResult<Instrument> {
        self.send_get_request(
            PUBLIC_INSTRUMENTS_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_instruments_request_structure() {
        let request = GetInstrumentsRequest {
            inst_type: InstrumentType::Spot,
            underlying: None,
            inst_family: None,
            inst_id: Some("BTC-USDT".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("SPOT")
        );
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USDT")
        );
        // Verify that null fields are not serialized
        assert!(serialized.get("instFamily").is_none());
        assert!(serialized.get("uly").is_none());
    }

    #[test]
    fn test_instrument_structure() {
        let instrument_json = json!({
            "instType": "SPOT",
            "instId": "BTC-USDT",
            "baseCcy": "BTC",
            "quoteCcy": "USDT",
            "settleCcy": "USDT",
            "category": "1",
            "tickSz": "0.1",
            "lotSz": "0.00000001",
            "minSz": "0.00001",
            "state": "live",
            "listTime": "1606276800000"
        });

        let instrument: Instrument = serde_json::from_value(instrument_json).unwrap();
        assert_eq!(instrument.inst_type, InstrumentType::Spot);
        assert_eq!(instrument.inst_id, "BTC-USDT");
        assert_eq!(instrument.base_ccy, "BTC");
        assert_eq!(instrument.quote_ccy, "USDT");
        assert_eq!(instrument.state, InstrumentState::Live);
        assert_eq!(instrument.inst_family, None);
    }

    #[test]
    fn test_get_instruments_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SPOT",
                    "instId": "BTC-USDT",
                    "baseCcy": "BTC",
                    "quoteCcy": "USDT",
                    "settleCcy": "USDT",
                    "category": "1",
                    "tickSz": "0.1",
                    "lotSz": "0.00000001",
                    "minSz": "0.00001",
                    "state": "live",
                    "listTime": "1606276800000"
                }
            ]
        });

        let response: OkxApiResponse<Instrument> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USDT");
    }

    #[test]
    fn test_instrument_structure_with_all_fields() {
        let instrument_json = json!({
            "instType": "FUTURES",
            "instId": "BTC-USD-240329",
            "uly": "BTC-USD",
            "instFamily": "BTC-USD",
            "category": "1",
            "baseCcy": "BTC",
            "quoteCcy": "USD",
            "settleCcy": "BTC",
            "ctVal": "100",
            "ctMult": "1",
            "ctValCcy": "USD",
            "optType": "",
            "stk": "",
            "listTime": "1606276800000",
            "auctionEndTime": "",
            "contTdSwTime": "",
            "openType": "",
            "expTime": "1711699200000",
            "lever": "125",
            "tickSz": "0.1",
            "lotSz": "1",
            "minSz": "1",
            "ctType": "linear",
            "alias": "quarter",
            "state": "live",
            "ruleType": "normal",
            "maxLmtSz": "10000",
            "maxMktSz": "1000",
            "maxLmtAmt": "1000000",
            "maxMktAmt": "100000",
            "maxTwapSz": "10000",
            "maxIcebergSz": "10000",
            "maxTriggerSz": "10000",
            "maxStopSz": "1000",
            "futureSettlement": true
        });

        let instrument: Instrument = serde_json::from_value(instrument_json).unwrap();
        assert_eq!(instrument.inst_type, InstrumentType::Futures);
        assert_eq!(instrument.inst_id, "BTC-USD-240329");
        assert_eq!(instrument.underlying, Some("BTC-USD".to_string()));
        assert_eq!(instrument.inst_family, Some("BTC-USD".to_string()));
        assert_eq!(instrument.settle_ccy, "BTC");
        assert_eq!(instrument.ct_val, Some("100".to_string()));
        assert_eq!(instrument.ct_mult, Some("1".to_string()));
        assert_eq!(instrument.ct_val_ccy, Some("USD".to_string()));
        assert_eq!(instrument.ct_type, Some("linear".to_string()));
        assert_eq!(instrument.alias, Some("quarter".to_string()));
        assert_eq!(instrument.state, InstrumentState::Live);
        assert_eq!(instrument.rule_type, Some("normal".to_string()));
        assert_eq!(instrument.max_lmt_sz, Some("10000".to_string()));
        assert_eq!(instrument.max_mkt_sz, Some("1000".to_string()));
        assert_eq!(instrument.max_lmt_amt, Some("1000000".to_string()));
        assert_eq!(instrument.max_mkt_amt, Some("100000".to_string()));
        assert_eq!(instrument.max_twap_sz, Some("10000".to_string()));
        assert_eq!(instrument.max_iceberg_sz, Some("10000".to_string()));
        assert_eq!(instrument.max_trigger_sz, Some("10000".to_string()));
        assert_eq!(instrument.max_stop_sz, Some("1000".to_string()));
        assert_eq!(instrument.future_settlement, Some(true));
    }

    #[test]
    fn test_instrument_serialization_roundtrip() {
        let original = GetInstrumentsRequest {
            inst_type: InstrumentType::Swap,
            underlying: Some("BTC-USD".to_string()),
            inst_family: Some("BTC-USD".to_string()),
            inst_id: None,
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetInstrumentsRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_type, deserialized.inst_type);
        assert_eq!(original.underlying, deserialized.underlying);
        assert_eq!(original.inst_family, deserialized.inst_family);
        assert_eq!(original.inst_id, deserialized.inst_id);
    }

    #[test]
    fn test_get_instruments_request_serialization_variations() {
        // Test SPOT request (should only include instType)
        let spot_request = GetInstrumentsRequest {
            inst_type: InstrumentType::Spot,
            underlying: None,
            inst_family: None,
            inst_id: None,
        };
        let spot_serialized = serde_json::to_value(&spot_request).unwrap();
        assert_eq!(
            spot_serialized.get("instType").and_then(|v| v.as_str()),
            Some("SPOT")
        );
        assert!(spot_serialized.get("instFamily").is_none());
        assert!(spot_serialized.get("uly").is_none());
        assert!(spot_serialized.get("instId").is_none());

        // Test FUTURES request with instFamily (should include instType and instFamily)
        let futures_request = GetInstrumentsRequest {
            inst_type: InstrumentType::Futures,
            underlying: None,
            inst_family: Some("BTC-USD".to_string()),
            inst_id: None,
        };
        let futures_serialized = serde_json::to_value(&futures_request).unwrap();
        assert_eq!(
            futures_serialized.get("instType").and_then(|v| v.as_str()),
            Some("FUTURES")
        );
        assert_eq!(
            futures_serialized
                .get("instFamily")
                .and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert!(futures_serialized.get("uly").is_none());
        assert!(futures_serialized.get("instId").is_none());
    }
}
