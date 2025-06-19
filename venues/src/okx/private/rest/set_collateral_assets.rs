use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to set collateral assets
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCollateralAssetsRequest {
    /// Currency
    pub ccy: String,

    /// Collateral assets: true, false
    pub coll_assets: bool,
}

/// Response for set collateral assets
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCollateralAssetsResponse {
    /// Currency
    pub ccy: String,

    /// Collateral assets status
    pub coll_assets: bool,
}

impl RestClient {
    /// Set collateral assets
    ///
    /// # Arguments
    /// * `request` - The set collateral assets request
    ///
    /// # Returns
    /// A result containing the set collateral assets response or an error
    pub async fn set_collateral_assets(
        &self,
        request: &SetCollateralAssetsRequest,
    ) -> RestResult<OkxApiResponse<SetCollateralAssetsResponse>> {
        self.send_request(
            "api/v5/account/set-collateral-assets",
            reqwest::Method::POST,
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
    fn test_set_collateral_assets_request_serialization() {
        let request = SetCollateralAssetsRequest {
            ccy: "BTC".to_string(),
            coll_assets: true,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"BTC\""));
        assert!(json.contains("\"collAssets\":true"));
    }

    #[test]
    fn test_set_collateral_assets_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "collAssets": true
                }
            ]
        }"#;

        let response: OkxApiResponse<SetCollateralAssetsResponse> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = &response.data[0];
        assert_eq!(result.ccy, "BTC");
        assert!(result.coll_assets);
    }
}