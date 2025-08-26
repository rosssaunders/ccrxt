use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const INTEREST_RECORDS_ENDPOINT: &str = "/earn/uni/interest_records";

/// Optional request parameters for listing interest records (pagination/time range)
#[derive(Debug, Clone, Serialize, Default)]
pub struct InterestRecordsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InterestRecord {
    pub id: Option<String>,
    pub currency: String,
    pub amount: String,
    /// timestamp in ms
    pub create_time: i64,
    /// optional description
    #[serde(default)]
    pub note: Option<String>,
}

impl RestClient {
    /// GET /earn/uni/interest_records
    ///
    /// Gate.io docs: https://www.gate.io/docs/developers/apiv4/en/#query-user-dividend-records
    pub async fn list_earnuni_interest_records(
        &self,
        params: InterestRecordsRequest,
    ) -> RestResult<Vec<InterestRecord>> {
        self.get_with_query(INTEREST_RECORDS_ENDPOINT, &params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interest_record_deserializes() {
        let json = r#"[{"id":"1","currency":"BTC","amount":"0.01","create_time":1673247054000}]"#;
        let v: Vec<InterestRecord> = serde_json::from_str(json).expect("deserialize");
        assert_eq!(v[0].currency, "BTC");
        assert_eq!(v[0].id.as_deref(), Some("1"));
    }
}
