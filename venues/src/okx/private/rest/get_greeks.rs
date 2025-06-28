use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to get account Greeks
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGreeksRequest {
    /// Currency, only applicable to OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Greeks information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Greeks {
    /// Timestamp
    pub ts: String,

    /// Delta BS
    pub delta_bs: String,

    /// Delta PA
    pub delta_pa: String,

    /// Gamma BS
    pub gamma_bs: String,

    /// Gamma PA
    pub gamma_pa: String,

    /// Theta BS
    pub theta_bs: String,

    /// Theta PA
    pub theta_pa: String,

    /// Vega BS
    pub vega_bs: String,

    /// Vega PA
    pub vega_pa: String,

    /// Currency
    pub ccy: String,
}

impl RestClient {
    /// Get account Greeks
    ///
    /// # Arguments
    /// * `request` - The get Greeks request
    ///
    /// # Returns
    /// A result containing the Greeks information or an error
    pub async fn get_greeks(
        &self,
        request: &GetGreeksRequest,
    ) -> RestResult<OkxApiResponse<Greeks>> {
        self.send_request(
            "api/v5/account/greeks",
            reqwest::Method::GET,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_greeks_request_serialization() {
        let request = GetGreeksRequest {
            ccy: Some("BTC".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=BTC"));
    }

    #[test]
    fn test_greeks_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ts": "1597026383085",
                    "deltaBs": "0.5",
                    "deltaPa": "0.48",
                    "gammaBs": "0.01",
                    "gammaPa": "0.009",
                    "thetaBs": "-0.2",
                    "thetaPa": "-0.19",
                    "vegaBs": "0.15",
                    "vegaPa": "0.14",
                    "ccy": "BTC"
                }
            ]
        }"#;

        let response: OkxApiResponse<Greeks> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let greeks = &response.data[0];
        assert_eq!(greeks.ts, "1597026383085");
        assert_eq!(greeks.delta_bs, "0.5");
        assert_eq!(greeks.delta_pa, "0.48");
        assert_eq!(greeks.gamma_bs, "0.01");
        assert_eq!(greeks.ccy, "BTC");
    }
}
