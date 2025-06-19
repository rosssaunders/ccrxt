use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to get collateral assets
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollateralAssetsRequest {
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Collateral asset information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralAsset {
    /// Currency
    pub ccy: String,

    /// Collateral rate
    pub collateral_rate: String,
}

impl RestClient {
    /// Get collateral assets
    ///
    /// # Arguments
    /// * `request` - The get collateral assets request
    ///
    /// # Returns
    /// A result containing the collateral assets or an error
    pub async fn get_collateral_assets(
        &self,
        request: &GetCollateralAssetsRequest,
    ) -> RestResult<OkxApiResponse<CollateralAsset>> {
        self.send_request(
            "api/v5/account/collateral-assets",
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
    fn test_get_collateral_assets_request_serialization() {
        let request = GetCollateralAssetsRequest {
            ccy: Some("BTC".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=BTC"));
    }

    #[test]
    fn test_collateral_asset_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "collateralRate": "0.95"
                }
            ]
        }"#;

        let response: OkxApiResponse<CollateralAsset> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let asset = &response.data[0];
        assert_eq!(asset.ccy, "BTC");
        assert_eq!(asset.collateral_rate, "0.95");
    }
}