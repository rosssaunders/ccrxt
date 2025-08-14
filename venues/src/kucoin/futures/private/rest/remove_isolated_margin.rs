use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const REMOVE_ISOLATED_MARGIN_ENDPOINT: &str = "/api/v1/margin/withdrawMargin";

/// Remove isolated margin request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveIsolatedMarginRequest {
    /// Symbol of the contract
    pub symbol: String,

    /// Amount of margin to withdraw
    pub withdraw_amount: String,
}

/// Remove isolated margin response
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveIsolatedMarginResponse {
    /// Withdrawn margin amount
    #[serde(rename = "data")]
    pub withdrawn_amount: String,
}

impl super::RestClient {
    /// Remove isolated margin from a position
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/positions/remove-isolated-margin)
    pub async fn remove_isolated_margin(
        &self,
        request: RemoveIsolatedMarginRequest,
    ) -> Result<(RestResponse<RemoveIsolatedMarginResponse>, ResponseHeaders)> {
        self.post_with_request(REMOVE_ISOLATED_MARGIN_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_isolated_margin_request_serialization() {
        let request = RemoveIsolatedMarginRequest {
            symbol: "XBTUSDTM".to_string(),
            withdraw_amount: "10.5".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("XBTUSDTM"));
        assert!(json.contains("withdrawAmount"));
        assert!(json.contains("10.5"));
    }

    #[test]
    fn test_remove_isolated_margin_response_deserialization() {
        let json = r#"{
            "data": "10.5"
        }"#;
        let response: RemoveIsolatedMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.withdrawn_amount, "10.5");
    }
}
