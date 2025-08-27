use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for setting maker instrument settings
const SET_MAKER_INSTRUMENT_SETTINGS_ENDPOINT: &str = "api/v5/rfq/maker-instrument-settings";

/// Request parameters for setting maker instrument settings
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMakerInstrumentSettingsRequest {
    /// Type of instrument
    #[serde(rename = "instType")]
    pub inst_type: String,

    /// Receive all instruments or not under specific instType setting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,

    /// Elements of the instType
    pub data: Vec<MakerInstrumentDataRequest>,
}

/// Individual instrument setting request data
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MakerInstrumentDataRequest {
    /// Instrument family. Required for FUTURES, OPTION and SWAP only
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,

    /// Instrument ID. Required for SPOT only
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,

    /// Max trade quantity for the product(s)
    #[serde(rename = "maxBlockSz", skip_serializing_if = "Option::is_none")]
    pub max_block_sz: Option<String>,

    /// Price bands in unit of ticks, measured against mark price
    #[serde(rename = "makerPxBand", skip_serializing_if = "Option::is_none")]
    pub maker_px_band: Option<String>,
}

/// Response for setting maker instrument settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMakerInstrumentSettingsResponse {
    /// Result of the request
    pub result: bool,
}

impl RestClient {
    /// Set quote products
    ///
    /// Customize the products which makers want to quote and receive RFQs for, and the
    /// corresponding price and size limit.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-set-quote-products)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The maker instrument settings request parameters
    ///
    /// # Returns
    /// Response containing the result of the operation
    pub async fn set_maker_instrument_settings(
        &self,
        request: SetMakerInstrumentSettingsRequest,
    ) -> RestResult<SetMakerInstrumentSettingsResponse> {
        self.send_post_request(
            SET_MAKER_INSTRUMENT_SETTINGS_ENDPOINT,
            &request,
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
    fn test_set_maker_instrument_settings_response_deserialization() {
        let response_json = json!({
            "result": true
        });

        let response: SetMakerInstrumentSettingsResponse =
            serde_json::from_value(response_json).unwrap();
        assert!(response.result);
    }

    #[test]
    fn test_set_maker_instrument_settings_api_response() {
        let api_response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "result": true
                }
            ]
        });

        let api_response: ApiResponse<SetMakerInstrumentSettingsResponse> =
            serde_json::from_value(api_response_json).unwrap();
        assert_eq!(api_response.code, "0");
        assert_eq!(api_response.data.len(), 1);
        assert!(api_response.data[0].result);
    }
}
