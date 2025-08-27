use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting non-tradable assets
const ASSET_NON_TRADABLE_ASSETS_ENDPOINT: &str = "api/v5/asset/non-tradable-assets";

/// Request parameters for getting non-tradable assets
#[derive(Debug, Clone, Serialize)]
pub struct GetNonTradableAssetsRequest {
    /// Single currency or multiple currencies (no more than 20) separated with comma, e.g. BTC or BTC,ETH
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Non-tradable asset information
#[derive(Debug, Clone, Deserialize)]
pub struct NonTradableAsset {
    /// Currency, e.g. CELT
    pub ccy: String,

    /// Chinese name of currency. There is no related name when it is not shown.
    pub name: String,

    /// Logo link of currency
    #[serde(rename = "logoLink")]
    pub logo_link: String,

    /// Withdrawable balance
    pub bal: String,

    /// Availability to withdraw to chain
    #[serde(rename = "canWd")]
    pub can_wd: bool,

    /// Chain for withdrawal
    pub chain: String,

    /// Minimum withdrawal amount of currency in a single transaction
    #[serde(rename = "minWd")]
    pub min_wd: String,

    /// Whether all assets in this currency must be withdrawn at one time
    #[serde(rename = "wdAll")]
    pub wd_all: bool,

    /// Fixed withdrawal fee
    pub fee: String,

    /// Fixed withdrawal fee unit, e.g. USDT
    #[serde(rename = "feeCcy")]
    pub fee_ccy: String,

    /// Burning fee rate, e.g "0.05" represents "5%"
    #[serde(rename = "burningFeeRate")]
    pub burning_fee_rate: String,

    /// Last 6 digits of contract address
    #[serde(rename = "ctAddr")]
    pub ct_addr: String,

    /// Withdrawal precision, indicating the number of digits after the decimal point
    #[serde(rename = "wdTickSz")]
    pub wd_tick_sz: String,

    /// Whether tag/memo information is required for withdrawal
    #[serde(rename = "needTag")]
    pub need_tag: bool,
}

impl RestClient {
    /// Get non-tradable assets
    ///
    /// Retrieve the funding account balances of all the assets and the amount that is available or on hold.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-non-tradable-assets)
    ///
    /// Rate limit: 6 requests per second
    ///
    /// # Arguments
    /// * `request` - The non-tradable assets request parameters
    ///
    /// # Returns
    /// A result containing the list of non-tradable assets
    pub async fn get_non_tradable_assets(
        &self,
        request: GetNonTradableAssetsRequest,
    ) -> RestResult<NonTradableAsset> {
        self.send_get_request(
            ASSET_NON_TRADABLE_ASSETS_ENDPOINT,
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
    fn test_get_non_tradable_assets_request_serialization() {
        let request = GetNonTradableAssetsRequest {
            ccy: Some("CELT,AIRDROP".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"CELT,AIRDROP\""));
    }

    #[test]
    fn test_get_non_tradable_assets_request_empty() {
        let request = GetNonTradableAssetsRequest { ccy: None };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_non_tradable_asset_deserialization() {
        let asset_json = json!({
            "ccy": "CELT",
            "name": "Celestia",
            "logoLink": "https://example.com/celt.png",
            "bal": "100.5",
            "canWd": true,
            "chain": "TIA-Celestia",
            "minWd": "0.1",
            "wdAll": false,
            "fee": "0.5",
            "feeCcy": "TIA",
            "burningFeeRate": "0",
            "ctAddr": "abc123",
            "wdTickSz": "0.1",
            "needTag": false
        });

        let asset: NonTradableAsset = serde_json::from_value(asset_json).unwrap();
        assert_eq!(asset.ccy, "CELT");
        assert_eq!(asset.name, "Celestia");
        assert!(asset.can_wd);
        assert!(!asset.wd_all);
        assert!(!asset.need_tag);
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "CELT",
                    "name": "Celestia",
                    "logoLink": "https://example.com/celt.png",
                    "bal": "100.5",
                    "canWd": true,
                    "chain": "TIA-Celestia",
                    "minWd": "0.1",
                    "wdAll": false,
                    "fee": "0.5",
                    "feeCcy": "TIA",
                    "burningFeeRate": "0",
                    "ctAddr": "abc123",
                    "wdTickSz": "0.1",
                    "needTag": false
                }
            ]
        });

        let response: ApiResponse<NonTradableAsset> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].ccy, "CELT");
    }
}
