use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for position builder
const ACCOUNT_POSITION_BUILDER_ENDPOINT: &str = "api/v5/account/position-builder";

/// Spot hedge data for position builder
#[derive(Debug, Clone, Serialize)]
pub struct SpotHedge {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// Spot hedge size
    pub sz: String,

    /// The price for calculating the profit and loss of the position
    pub px: String,
}

/// Option data for position builder
#[derive(Debug, Clone, Serialize)]
pub struct OptionData {
    /// Instrument ID, e.g. BTC-USD-221216-4000-C
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Option side
    /// buy, sell
    pub side: String,

    /// Option size
    pub sz: String,

    /// The price for calculating the profit and loss of the position
    pub px: String,
}

/// Request parameters for position builder
#[derive(Debug, Clone, Serialize)]
pub struct PositionBuilderRequest {
    /// Include options, the default is false
    #[serde(rename = "inclRealPosAndEq", skip_serializing_if = "Option::is_none")]
    pub incl_real_pos_and_eq: Option<bool>,

    /// Spot hedge data
    #[serde(rename = "spotHedge", skip_serializing_if = "Option::is_none")]
    pub spot_hedge: Option<Vec<SpotHedge>>,

    /// Option data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<Vec<OptionData>>,
}

/// Greeks data
#[derive(Debug, Clone, Deserialize)]
pub struct Greeks {
    /// Delta BS
    #[serde(rename = "deltaBS")]
    pub delta_bs: String,

    /// Delta PA
    #[serde(rename = "deltaPA")]
    pub delta_pa: String,

    /// Gamma BS
    #[serde(rename = "gammaBS")]
    pub gamma_bs: String,

    /// Gamma PA  
    #[serde(rename = "gammaPA")]
    pub gamma_pa: String,

    /// Theta BS
    #[serde(rename = "thetaBS")]
    pub theta_bs: String,

    /// Theta PA
    #[serde(rename = "thetaPA")]
    pub theta_pa: String,

    /// Vega BS
    #[serde(rename = "vegaBS")]
    pub vega_bs: String,

    /// Vega PA
    #[serde(rename = "vegaPA")]
    pub vega_pa: String,
}

/// Position builder response
#[derive(Debug, Clone, Deserialize)]
pub struct PositionBuilderResponse {
    /// Greeks
    pub greeks: Greeks,

    /// The estimated profit and loss under different underlying price scenarios
    #[serde(rename = "pnlGraph")]
    pub pnl_graph: Vec<serde_json::Value>,
}

impl RestClient {
    /// Position builder
    ///
    /// Calculates portfolio margin information for simulated position adjustments in derivatives trading.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-position-builder)
    ///
    /// Rate limit: 2 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The position builder request parameters
    ///
    /// # Returns
    /// A result containing the position builder response
    pub async fn position_builder(
        &self,
        request: PositionBuilderRequest,
    ) -> RestResult<PositionBuilderResponse> {
        self.send_post_request(
            ACCOUNT_POSITION_BUILDER_ENDPOINT,
            Some(&request),
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
    fn test_position_builder_request_serialization() {
        let spot_hedge = vec![SpotHedge {
            ccy: "BTC".to_string(),
            sz: "0.5".to_string(),
            px: "50000".to_string(),
        }];

        let option_data = vec![OptionData {
            inst_id: "BTC-USD-221216-4000-C".to_string(),
            side: "buy".to_string(),
            sz: "10".to_string(),
            px: "1000".to_string(),
        }];

        let request = PositionBuilderRequest {
            incl_real_pos_and_eq: Some(true),
            spot_hedge: Some(spot_hedge),
            option: Some(option_data),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"inclRealPosAndEq\":true"));
        assert!(json.contains("\"ccy\":\"BTC\""));
        assert!(json.contains("\"instId\":\"BTC-USD-221216-4000-C\""));
    }

    #[test]
    fn test_position_builder_response_deserialization() {
        let response_json = json!({
            "greeks": {
                "deltaBS": "0.5",
                "deltaPA": "0.4",
                "gammaBS": "0.1",
                "gammaPA": "0.1",
                "thetaBS": "-10",
                "thetaPA": "-9",
                "vegaBS": "20",
                "vegaPA": "21"
            },
            "pnlGraph": []
        });

        let response: PositionBuilderResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.greeks.delta_bs, "0.5");
        assert_eq!(response.greeks.vega_bs, "20");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "greeks": {
                        "deltaBS": "0.5",
                        "deltaPA": "0.4",
                        "gammaBS": "0.1",
                        "gammaPA": "0.1",
                        "thetaBS": "-10",
                        "thetaPA": "-9",
                        "vegaBS": "20",
                        "vegaPA": "21"
                    },
                    "pnlGraph": []
                }
            ]
        });

        let response: ApiResponse<PositionBuilderResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].greeks.delta_bs, "0.5");
    }
}
