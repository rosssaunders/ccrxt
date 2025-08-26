use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const LEND_RECORDS_ENDPOINT: &str = "/earn/uni/lend_records";

/// Request parameters for lending transaction records (/earn/uni/lend_records)
#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub struct LoanRecordsRequest {
    /// Currency name to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Page number (default: 1). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of items to return (1-100). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Start timestamp (unix seconds). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End timestamp (unix seconds). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Operation type: "lend" or "redeem". Optional.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Lending transaction record returned by the API.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct LoanRecord {
    /// Record type: lend or redeem
    #[serde(rename = "type")]
    pub r#type: String,

    /// Currency name
    pub currency: String,

    /// Current amount
    pub amount: String,

    /// Previous available amount
    #[serde(default)]
    pub last_wallet_amount: Option<String>,

    /// Previous lent amount
    #[serde(default)]
    pub last_lent_amount: Option<String>,

    /// Previous frozen amount
    #[serde(default)]
    pub last_frozen_amount: Option<String>,

    /// Created time (ms)
    pub create_time: i64,

    /// Optional record id
    #[serde(default)]
    pub id: Option<String>,
}

impl RestClient {
    /// Query lending transaction records
    /// Query lending transaction records (GET /earn/uni/lend_records)
    ///
    /// Gate.io docs: https://www.gate.io/docs/developers/apiv4/en/#query-lending-transaction-records
    pub async fn spot_get_loan_records(
        &self,
        params: LoanRecordsRequest,
    ) -> RestResult<Vec<LoanRecord>> {
        // GET with query parameters
        self.get_with_query(LEND_RECORDS_ENDPOINT, &params).await
    }

    /// Get a specific lending transaction record by ID.
    pub async fn spot_get_loan_record(&self, loan_record_id: &str) -> RestResult<LoanRecord> {
        let endpoint = format!("{}/{}", LEND_RECORDS_ENDPOINT, loan_record_id);
        // Gate.io docs for single record retrieval are under the general loan records section
        // See: https://www.gate.io/docs/developers/apiv4/en/#query-lending-transaction-records
        self.send_get_request(&endpoint, Option::<&()>::None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loan_records_request_serializes_optional_fields() {
        let r = LoanRecordsRequest {
            currency: Some("BTC".into()),
            page: Some(2),
            limit: Some(20),
            from: Some(1620000000),
            to: None,
            r#type: Some("lend".into()),
        };
        let qs = serde_urlencoded::to_string(&r).expect("serialize");
        assert!(qs.contains("currency=BTC"));
        assert!(qs.contains("page=2"));
        assert!(qs.contains("limit=20"));
        assert!(qs.contains("from=1620000000"));
        assert!(qs.contains("type=lend"));
    }

    #[test]
    fn loan_record_deserializes() {
        let json = r#"{"type":"lend","currency":"BTC","amount":"1","last_wallet_amount":"0.2","last_lent_amount":"0","last_frozen_amount":"0","create_time":1673247054000,"id":"abc"}"#;
        let lr: LoanRecord = serde_json::from_str(json).expect("deserialize");
        assert_eq!(lr.currency, "BTC");
        assert_eq!(lr.amount, "1");
        assert_eq!(lr.id.as_deref(), Some("abc"));
    }
}
