use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};
use serde::{Deserialize, Serialize};

/// Request parameters for getting instrument tick bands
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentTickBandsRequest {
    /// Instrument type (required)
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    /// Instrument family (optional, only applicable to OPTION)
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Tick size band information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickBand {
    /// Minimum price while placing an order
    #[serde(rename = "minPx")]
    pub min_px: String,
    /// Maximum price while placing an order
    #[serde(rename = "maxPx")]
    pub max_px: String,
    /// Tick size, e.g. 0.0001
    #[serde(rename = "tickSz")]
    pub tick_sz: String,
}

/// Response for getting instrument tick bands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInstrumentTickBandsResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Tick band data
    pub data: Vec<InstrumentTickBandData>,
}

/// Individual instrument tick band data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentTickBandData {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Instrument family
    #[serde(rename = "instFamily")]
    pub inst_family: String,
    /// Tick size band
    #[serde(rename = "tickBand")]
    pub tick_band: Vec<TickBand>,
}

impl RestClient {
    /// Get option tick bands
    ///
    /// Get option tick bands information
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-option-tick-bands
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The tick bands request parameters
    ///
    /// # Returns
    /// Response containing the tick band information
    pub async fn get_instrument_tick_bands(
        &self,
        request: GetInstrumentTickBandsRequest,
    ) -> RestResult<GetInstrumentTickBandsResponse> {
        self.send_request(
            "api/v5/public/instrument-tick-bands",
            reqwest::Method::GET,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_instrument_tick_bands_request_structure() {
        let request = GetInstrumentTickBandsRequest {
            inst_type: InstrumentType::Option,
            inst_family: Some("BTC-USD".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("OPTION")
        );
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
    }

    #[test]
    fn test_get_instrument_tick_bands_request_minimal() {
        let request = GetInstrumentTickBandsRequest {
            inst_type: InstrumentType::Option,
            inst_family: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("OPTION")
        );
        assert!(serialized.get("instFamily").is_none());
    }

    #[test]
    fn test_tick_band_structure() {
        let tick_band_json = json!({
            "minPx": "0.0001",
            "maxPx": "0.01",
            "tickSz": "0.0001"
        });

        let tick_band: TickBand = serde_json::from_value(tick_band_json).unwrap();
        assert_eq!(tick_band.min_px, "0.0001");
        assert_eq!(tick_band.max_px, "0.01");
        assert_eq!(tick_band.tick_sz, "0.0001");
    }

    #[test]
    fn test_get_instrument_tick_bands_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "OPTION",
                    "instFamily": "BTC-USD",
                    "tickBand": [
                        {
                            "minPx": "0.0001",
                            "maxPx": "0.01",
                            "tickSz": "0.0001"
                        },
                        {
                            "minPx": "0.01",
                            "maxPx": "0.1",
                            "tickSz": "0.0005"
                        }
                    ]
                }
            ]
        });

        let response: GetInstrumentTickBandsResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        
        let data = response.data.first().unwrap();
        assert_eq!(data.inst_type, "OPTION");
        assert_eq!(data.inst_family, "BTC-USD");
        assert_eq!(data.tick_band.len(), 2);
        assert_eq!(data.tick_band[0].min_px, "0.0001");
        assert_eq!(data.tick_band[0].max_px, "0.01");
        assert_eq!(data.tick_band[0].tick_sz, "0.0001");
        assert_eq!(data.tick_band[1].min_px, "0.01");
        assert_eq!(data.tick_band[1].max_px, "0.1");
        assert_eq!(data.tick_band[1].tick_sz, "0.0005");
    }

    #[test]
    fn test_instrument_tick_band_data_structure() {
        let data_json = json!({
            "instType": "OPTION",
            "instFamily": "ETH-USD",
            "tickBand": [
                {
                    "minPx": "0.0001",
                    "maxPx": "0.005",
                    "tickSz": "0.0001"
                }
            ]
        });

        let data: InstrumentTickBandData = serde_json::from_value(data_json).unwrap();
        assert_eq!(data.inst_type, "OPTION");
        assert_eq!(data.inst_family, "ETH-USD");
        assert_eq!(data.tick_band.len(), 1);
        assert_eq!(data.tick_band[0].min_px, "0.0001");
        assert_eq!(data.tick_band[0].max_px, "0.005");
        assert_eq!(data.tick_band[0].tick_sz, "0.0001");
    }

    #[test]
    fn test_request_serialization_roundtrip() {
        let original = GetInstrumentTickBandsRequest {
            inst_type: InstrumentType::Option,
            inst_family: Some("BTC-USD".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetInstrumentTickBandsRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_type, deserialized.inst_type);
        assert_eq!(original.inst_family, deserialized.inst_family);
    }
}