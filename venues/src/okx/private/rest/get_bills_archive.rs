use serde::Serialize;

use super::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

const ACCOUNT_BILLS_ARCHIVE_ENDPOINT: &str = "api/v5/account/bills-archive";

/// Request to get bills archive
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBillsArchiveRequest {
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,

    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Margin mode
    /// "isolated", "cross"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,

    /// Contract type
    /// "linear", "inverse"
    /// Only applicable to FUTURES/SWAP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct_type: Option<String>,

    /// Bill type
    /// Same values as bills endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Bill subtype
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,

    /// Pagination of data to return records earlier than the requested billId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested billId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Filter with a begin timestamp. Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// Filter with an end timestamp. Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Number of results per request. Maximum is 100. Default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Bill archive details (reusing the Bill struct from get_bills)
pub use super::get_bills::Bill as BillArchive;

impl RestClient {
    /// Get bills archive
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-bills-details-last-3-months)
    ///
    /// # Arguments
    /// * `request` - The get bills archive request
    ///
    /// # Returns
    /// A result containing the bills archive or an error
    pub async fn get_bills_archive(
        &self,
        request: &GetBillsArchiveRequest,
    ) -> RestResult<BillArchive> {
        self.send_get_request(
            ACCOUNT_BILLS_ARCHIVE_ENDPOINT,
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
    fn test_get_bills_archive_request_serialization() {
        let request = GetBillsArchiveRequest {
            inst_type: Some(InstrumentType::Spot),
            ccy: Some("BTC".to_string()),
            mgn_mode: Some("cross".to_string()),
            ct_type: None,
            r#type: Some("2".to_string()),
            sub_type: None,
            after: None,
            before: None,
            begin: Some("1597026383085".to_string()),
            end: Some("1597026383086".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SPOT"));
        assert!(serialized.contains("ccy=BTC"));
        assert!(serialized.contains("mgnMode=cross"));
        assert!(serialized.contains("type=2"));
        assert!(serialized.contains("begin=1597026383085"));
        assert!(serialized.contains("end=1597026383086"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_get_bills_archive_minimal_request() {
        let request = GetBillsArchiveRequest {
            inst_type: None,
            ccy: None,
            mgn_mode: None,
            ct_type: None,
            r#type: None,
            sub_type: None,
            after: None,
            before: None,
            begin: None,
            end: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }
}
