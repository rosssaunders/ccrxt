use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::PrivateRestClient as RestClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for Classic Portfolio Margin Account Information.
const PORTFOLIO_MARGIN_ENDPOINT: &str = "/fapi/v1/pmAccountInfo";

/// Request parameters for the Classic Portfolio Margin Account Information endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPortfolioMarginAccountRequest {
    /// Asset name (e.g., "BTC"). Required.
    /// This field is securely stored and expected as SecretString.
    pub asset: Cow<'static, str>,

    /// Receiving window (optional, in milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch). Required.
    pub timestamp: u64,
}

/// Response for Classic Portfolio Margin Account Information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioMarginAccountResponse {
    /// Asset name.
    pub asset: Cow<'static, str>,

    /// Maximum amount for transfer out.
    pub max_withdraw_amount: Cow<'static, str>,

    /// Maximum virtual amount for transfer out in USD.
    #[serde(rename = "maxWithdrawAmountUSD")]
    pub max_withdraw_amount_usd: Cow<'static, str>,
}

impl RestClient {
    /// Classic Portfolio Margin Account Information
    ///
    /// Get Classic Portfolio Margin current account information.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/portfolio-margin-endpoints/Classic-Portfolio-Margin-Account-Information)
    ///
    /// Rate limit: 5 requests per second
    ///
    /// # Arguments
    /// * `params` - The request parameters for portfolio margin account info
    ///
    /// # Returns
    /// Portfolio margin account information
    pub async fn get_portfolio_margin_account(
        &self,
        params: GetPortfolioMarginAccountRequest,
    ) -> RestResult<PortfolioMarginAccountResponse> {
        self.send_get_signed_request(PORTFOLIO_MARGIN_ENDPOINT, params, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_portfolio_margin_account_response_deserialization() {
        let json = r#"{
            "asset": "BTC",
            "maxWithdrawAmount": "27.43689636",
            "maxWithdrawAmountUSD": "1627523.32459208"
        }"#;
        let response: PortfolioMarginAccountResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.asset, "BTC");
        assert_eq!(response.max_withdraw_amount, "27.43689636");
        assert_eq!(response.max_withdraw_amount_usd, "1627523.32459208");
    }

    #[test]
    fn test_get_portfolio_margin_account_request_serialization() {
        let req = GetPortfolioMarginAccountRequest {
            asset: Cow::Borrowed("BTC"),
            recv_window: Some(5000),
            timestamp: 1627523000000,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTC"));
        assert!(json.contains("recvWindow"));
        assert!(json.contains("timestamp"));
    }
}
