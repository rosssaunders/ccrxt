use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting maker instrument settings
const GET_MAKER_INSTRUMENT_SETTINGS_ENDPOINT: &str = "api/v5/rfq/maker-instrument-settings";

/// Maker instrument settings data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MakerInstrumentSetting {
    /// Type of instrument
    #[serde(rename = "instType")]
    pub inst_type: String,

    /// Receive all instruments or not under specific instType setting
    pub include_all: bool,

    /// Elements of the instType
    pub data: Vec<MakerInstrumentData>,
}

/// Individual instrument setting data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MakerInstrumentData {
    /// Instrument family. Required for FUTURES, OPTION and SWAP only
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,

    /// Instrument ID. Required for SPOT only
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,

    /// Max trade quantity for the product(s)
    #[serde(rename = "maxBlockSz")]
    pub max_block_sz: String,

    /// Price bands in unit of ticks, measured against mark price
    #[serde(rename = "makerPxBand")]
    pub maker_px_band: String,
}

impl RestClient {
    /// Get quote products
    ///
    /// Retrieve the products which makers want to quote and receive RFQs for, and the
    /// corresponding price and size limit.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-get-quote-products)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Returns
    /// Response containing maker instrument settings
    pub async fn get_maker_instrument_settings(&self) -> RestResult<MakerInstrumentSetting> {
        self.send_get_request(
            GET_MAKER_INSTRUMENT_SETTINGS_ENDPOINT,
            None::<&()>,
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_maker_instrument_data_deserialization() {
        let data_json = json!({
            "instFamily": "BTC-USD",
            "maxBlockSz": "100",
            "makerPxBand": "5"
        });

        let data: MakerInstrumentData = serde_json::from_value(data_json).unwrap();
        assert_eq!(data.inst_family.unwrap(), "BTC-USD");
        assert_eq!(data.max_block_sz, "100");
        assert_eq!(data.maker_px_band, "5");
    }

    #[test]
    fn test_maker_instrument_setting_deserialization() {
        let setting_json = json!({
            "instType": "FUTURES",
            "includeAll": false,
            "data": [
                {
                    "instFamily": "BTC-USD",
                    "maxBlockSz": "100",
                    "makerPxBand": "5"
                }
            ]
        });

        let setting: MakerInstrumentSetting = serde_json::from_value(setting_json).unwrap();
        assert_eq!(setting.inst_type, "FUTURES");
        assert!(!setting.include_all);
        assert_eq!(setting.data.len(), 1);
        assert_eq!(setting.data[0].inst_family.as_ref().unwrap(), "BTC-USD");
    }

    #[test]
    fn test_maker_instrument_setting_spot_deserialization() {
        let setting_json = json!({
            "instType": "SPOT",
            "includeAll": true,
            "data": [
                {
                    "instId": "BTC-USDT",
                    "maxBlockSz": "10",
                    "makerPxBand": "1"
                }
            ]
        });

        let setting: MakerInstrumentSetting = serde_json::from_value(setting_json).unwrap();
        assert_eq!(setting.inst_type, "SPOT");
        assert!(setting.include_all);
        assert_eq!(setting.data.len(), 1);
        assert_eq!(setting.data[0].inst_id.as_ref().unwrap(), "BTC-USDT");
    }

    #[test]
    fn test_get_maker_instrument_settings_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SWAP",
                    "includeAll": false,
                    "data": [
                        {
                            "instFamily": "ETH-USD",
                            "maxBlockSz": "50",
                            "makerPxBand": "3"
                        }
                    ]
                }
            ]
        });

        let response: ApiResponse<MakerInstrumentSetting> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].inst_type, "SWAP");
        assert!(!response.data[0].include_all);
    }
}
