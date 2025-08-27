use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for position builder graph
const ACCOUNT_POSITION_BUILDER_GRAPH_ENDPOINT: &str = "api/v5/account/position-builder-graph";

/// Request parameters for position builder graph
#[derive(Debug, Clone, Serialize)]
pub struct PositionBuilderGraphRequest {
    /// Underlying price, e.g. 50000
    #[serde(rename = "uly")]
    pub uly: String,

    /// Option price volatility, e.g. 0.2
    #[serde(rename = "vol")]
    pub vol: String,

    /// Days to expiry, e.g. 30
    #[serde(rename = "expTime")]
    pub exp_time: String,

    /// Include options, the default is false
    #[serde(rename = "inclRealPosAndEq", skip_serializing_if = "Option::is_none")]
    pub incl_real_pos_and_eq: Option<bool>,

    /// Option data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<Vec<serde_json::Value>>,

    /// Spot hedge data
    #[serde(rename = "spotHedge", skip_serializing_if = "Option::is_none")]
    pub spot_hedge: Option<Vec<serde_json::Value>>,
}

/// PnL graph point
#[derive(Debug, Clone, Deserialize)]
pub struct PnlGraphPoint {
    /// Underlying price
    pub px: String,

    /// PnL
    pub pnl: String,

    /// Delta
    pub delta: String,

    /// Gamma
    pub gamma: String,

    /// Theta
    pub theta: String,

    /// Vega
    pub vega: String,
}

/// Position builder graph response
#[derive(Debug, Clone, Deserialize)]
pub struct PositionBuilderGraphResponse {
    /// PnL graph data points
    #[serde(rename = "pnlGraph")]
    pub pnl_graph: Vec<PnlGraphPoint>,
}

impl RestClient {
    /// Position builder graph
    ///
    /// Retrieve position builder graph data.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-position-builder-graph)
    ///
    /// Rate limit: 2 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The position builder graph request parameters
    ///
    /// # Returns
    /// A result containing the position builder graph response
    pub async fn position_builder_graph(
        &self,
        request: PositionBuilderGraphRequest,
    ) -> RestResult<PositionBuilderGraphResponse> {
        self.send_post_request(
            ACCOUNT_POSITION_BUILDER_GRAPH_ENDPOINT,
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
    fn test_position_builder_graph_request_serialization() {
        let request = PositionBuilderGraphRequest {
            uly: "50000".to_string(),
            vol: "0.2".to_string(),
            exp_time: "30".to_string(),
            incl_real_pos_and_eq: Some(true),
            option: None,
            spot_hedge: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"uly\":\"50000\""));
        assert!(json.contains("\"vol\":\"0.2\""));
        assert!(json.contains("\"expTime\":\"30\""));
        assert!(json.contains("\"inclRealPosAndEq\":true"));
    }

    #[test]
    fn test_pnl_graph_point_deserialization() {
        let point_json = json!({
            "px": "48000",
            "pnl": "1000",
            "delta": "0.5",
            "gamma": "0.01",
            "theta": "-5",
            "vega": "20"
        });

        let point: PnlGraphPoint = serde_json::from_value(point_json).unwrap();
        assert_eq!(point.px, "48000");
        assert_eq!(point.pnl, "1000");
        assert_eq!(point.delta, "0.5");
        assert_eq!(point.vega, "20");
    }

    #[test]
    fn test_position_builder_graph_response_deserialization() {
        let response_json = json!({
            "pnlGraph": [
                {
                    "px": "48000",
                    "pnl": "1000",
                    "delta": "0.5",
                    "gamma": "0.01",
                    "theta": "-5",
                    "vega": "20"
                },
                {
                    "px": "52000",
                    "pnl": "-500",
                    "delta": "0.3",
                    "gamma": "0.008",
                    "theta": "-3",
                    "vega": "15"
                }
            ]
        });

        let response: PositionBuilderGraphResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.pnl_graph.len(), 2);
        assert_eq!(response.pnl_graph[0].px, "48000");
        assert_eq!(response.pnl_graph[1].pnl, "-500");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "pnlGraph": [
                        {
                            "px": "50000",
                            "pnl": "0",
                            "delta": "0.4",
                            "gamma": "0.01",
                            "theta": "-4",
                            "vega": "18"
                        }
                    ]
                }
            ]
        });

        let response: ApiResponse<PositionBuilderGraphResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].pnl_graph.len(), 1);
    }
}
