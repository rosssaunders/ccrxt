use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting account asset valuation
const ASSET_VALUATION_ENDPOINT: &str = "api/v5/asset/asset-valuation";

/// Request parameters for getting account asset valuation
#[derive(Debug, Clone, Serialize)]
pub struct GetAssetValuationRequest {
    /// Asset valuation calculation unit
    /// BTC, USDT, USD, CNY, JP, KRW, RUB, EUR, VND, IDR, INR, PHP, THB, TRY, AUD, SGD, ARS, SAR, AED, IQD
    /// The default is the valuation in BTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Asset valuation details for each account
#[derive(Debug, Clone, Deserialize)]
pub struct AssetValuationDetails {
    /// Funding account
    pub funding: String,

    /// Trading account
    pub trading: String,

    /// Classic account (Deprecated)
    pub classic: String,

    /// Earn account
    pub earn: String,
}

/// Account asset valuation information
#[derive(Debug, Clone, Deserialize)]
pub struct AssetValuation {
    /// Valuation of total account assets
    #[serde(rename = "totalBal")]
    pub total_bal: String,

    /// Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: String,

    /// Asset valuation details for each account
    pub details: AssetValuationDetails,
}

impl RestClient {
    /// Get account asset valuation
    ///
    /// View account asset valuation
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-account-asset-valuation)
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The asset valuation request parameters
    ///
    /// # Returns
    /// A result containing the account asset valuation
    pub async fn get_asset_valuation(
        &self,
        request: GetAssetValuationRequest,
    ) -> RestResult<AssetValuation> {
        self.send_get_request(
            ASSET_VALUATION_ENDPOINT,
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
    fn test_get_asset_valuation_request_serialization() {
        let request = GetAssetValuationRequest {
            ccy: Some("USDT".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"USDT\""));
    }

    #[test]
    fn test_get_asset_valuation_request_empty() {
        let request = GetAssetValuationRequest { ccy: None };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_asset_valuation_deserialization() {
        let valuation_json = json!({
            "totalBal": "1000.123456",
            "ts": "1597026383085",
            "details": {
                "funding": "500.0",
                "trading": "400.0",
                "classic": "0.0",
                "earn": "100.123456"
            }
        });

        let valuation: AssetValuation = serde_json::from_value(valuation_json).unwrap();
        assert_eq!(valuation.total_bal, "1000.123456");
        assert_eq!(valuation.ts, "1597026383085");
        assert_eq!(valuation.details.funding, "500.0");
        assert_eq!(valuation.details.trading, "400.0");
        assert_eq!(valuation.details.earn, "100.123456");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "totalBal": "1000.123456",
                    "ts": "1597026383085",
                    "details": {
                        "funding": "500.0",
                        "trading": "400.0",
                        "classic": "0.0",
                        "earn": "100.123456"
                    }
                }
            ]
        });

        let response: ApiResponse<AssetValuation> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].total_bal, "1000.123456");
        assert_eq!(response.data[0].details.funding, "500.0");
    }
}
