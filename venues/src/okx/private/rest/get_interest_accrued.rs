use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

const ACCOUNT_INTEREST_ACCRUED_ENDPOINT: &str = "api/v5/account/interest-accrued";

/// Request to get interest accrued
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestAccruedRequest {
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,

    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Instrument ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,

    /// Margin mode
    /// "cross", "isolated"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,

    /// Pagination of data to return records earlier than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested ts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. Maximum is 100. Default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Interest accrued details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestAccrued {
    /// Instrument type
    pub inst_type: String,

    /// Currency
    pub ccy: String,

    /// Instrument ID
    pub inst_id: Option<String>,

    /// Margin mode
    pub mgn_mode: String,

    /// Interest amount
    pub interest: String,

    /// Interest rate
    pub interest_rate: String,

    /// Liability
    pub liab: String,

    /// Timestamp
    pub ts: String,
}

impl RestClient {
    /// Get interest accrued
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-interest-accrued-data
    ///
    /// # Arguments
    /// * `request` - The get interest accrued request
    ///
    /// # Returns
    /// A result containing the interest accrued or an error
    pub async fn get_interest_accrued(
        &self,
        request: &GetInterestAccruedRequest,
    ) -> RestResult<InterestAccrued> {
        self.send_request(
            ACCOUNT_INTEREST_ACCRUED_ENDPOINT,
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
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_interest_accrued_request_serialization() {
        let request = GetInterestAccruedRequest {
            inst_type: Some(InstrumentType::Spot),
            ccy: Some("USDT".to_string()),
            inst_id: Some("BTC-USDT".to_string()),
            mgn_mode: Some("cross".to_string()),
            after: None,
            before: None,
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SPOT"));
        assert!(serialized.contains("ccy=USDT"));
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("mgnMode=cross"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_get_interest_accrued_minimal_request() {
        let request = GetInterestAccruedRequest {
            inst_type: None,
            ccy: None,
            inst_id: None,
            mgn_mode: None,
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_interest_accrued_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SPOT",
                    "ccy": "USDT",
                    "instId": "BTC-USDT",
                    "mgnMode": "cross",
                    "interest": "0.123",
                    "interestRate": "0.0001",
                    "liab": "1000.5",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: OkxApiResponse<InterestAccrued> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let interest = &response.data[0];
        assert_eq!(interest.inst_type, "SPOT");
        assert_eq!(interest.ccy, "USDT");
        assert_eq!(interest.interest, "0.123");
        assert_eq!(interest.interest_rate, "0.0001");
    }
}
