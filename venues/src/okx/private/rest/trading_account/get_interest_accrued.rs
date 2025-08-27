use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting interest accrued data
const ACCOUNT_INTEREST_ACCRUED_ENDPOINT: &str = "api/v5/account/interest-accrued";

/// Request parameters for getting interest accrued data
#[derive(Debug, Clone, Serialize)]
pub struct GetInterestAccruedRequest {
    /// Instrument type
    /// MARGIN: Spot and margin
    /// SWAP: Perpetual swap
    /// FUTURES: Futures
    /// OPTION: Option
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,

    /// Currency, e.g. BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Instrument ID, e.g. BTC-USDT
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,

    /// Margin mode
    /// cross: cross margin
    /// isolated: isolated margin
    #[serde(rename = "mgnMode", skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,

    /// Pagination of data to return records earlier than the requested timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Interest accrued data
#[derive(Debug, Clone, Deserialize)]
pub struct InterestAccrued {
    /// Instrument type
    #[serde(rename = "type")]
    pub inst_type: String,

    /// Instrument ID, e.g. BTC-USDT
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Margin mode
    #[serde(rename = "mgnMode")]
    pub mgn_mode: String,

    /// Currency, e.g. BTC
    pub ccy: String,

    /// Interest accrued
    pub interest: String,

    /// Interest accrued time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "interestTime")]
    pub interest_time: String,

    /// Loan amount
    pub loan: String,

    /// Data generation time, Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: String,
}

impl RestClient {
    /// Get interest accrued data (trading account)
    ///
    /// Get interest accrued data for trading account.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-interest-accrued)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The interest accrued request parameters
    ///
    /// # Returns
    /// A result containing the interest accrued data
    pub async fn get_trading_interest_accrued(
        &self,
        request: GetInterestAccruedRequest,
    ) -> RestResult<InterestAccrued> {
        self.send_get_request(
            ACCOUNT_INTEREST_ACCRUED_ENDPOINT,
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
    fn test_get_interest_accrued_request_serialization() {
        let request = GetInterestAccruedRequest {
            inst_type: Some("MARGIN".to_string()),
            ccy: Some("BTC".to_string()),
            inst_id: Some("BTC-USDT".to_string()),
            mgn_mode: Some("cross".to_string()),
            after: None,
            before: None,
            limit: Some("50".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"type\":\"MARGIN\""));
        assert!(json.contains("\"ccy\":\"BTC\""));
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        assert!(json.contains("\"limit\":\"50\""));
    }

    #[test]
    fn test_interest_accrued_deserialization() {
        let interest_json = json!({
            "type": "MARGIN",
            "instId": "BTC-USDT",
            "mgnMode": "cross",
            "ccy": "BTC",
            "interest": "0.00012345",
            "interestTime": "1597026383085",
            "loan": "0.1",
            "ts": "1597026383085"
        });

        let interest: InterestAccrued = serde_json::from_value(interest_json).unwrap();
        assert_eq!(interest.inst_type, "MARGIN");
        assert_eq!(interest.inst_id, "BTC-USDT");
        assert_eq!(interest.mgn_mode, "cross");
        assert_eq!(interest.ccy, "BTC");
        assert_eq!(interest.interest, "0.00012345");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "type": "MARGIN",
                    "instId": "BTC-USDT",
                    "mgnMode": "cross",
                    "ccy": "BTC",
                    "interest": "0.00012345",
                    "interestTime": "1597026383085",
                    "loan": "0.1",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: ApiResponse<InterestAccrued> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].interest, "0.00012345");
    }
}
