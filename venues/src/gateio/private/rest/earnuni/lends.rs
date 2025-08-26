use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const LENDS_ENDPOINT: &str = "/earn/uni/lends";

/// Request to create a lend (POST /earn/uni/lends)
#[derive(Debug, Clone, Serialize)]
pub struct CreateLendRequest {
    pub currency: String,
    pub amount: String,
    /// Optional target rate per hour (string decimal)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// Optional flag to use best available rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_best_rate: Option<bool>,
}

/// Response for a created lend or an active lend entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LendInfo {
    pub order_id: Option<String>,
    pub currency: String,
    pub amount: String,
    pub rate: Option<String>,
    pub status: Option<String>,
    /// timestamp in ms
    pub create_time: Option<i64>,
}

impl RestClient {
    /// Create a lending order (POST /earn/uni/lends)
    ///
    /// Gate.io docs: https://www.gate.io/docs/developers/apiv4/en/#create-lending-or-redemption
    pub async fn create_earnuni_lend(&self, req: CreateLendRequest) -> RestResult<LendInfo> {
        self.send_post_request(LENDS_ENDPOINT, Some(&req)).await
    }

    /// List current lends (GET /earn/uni/lends)
    ///
    /// Gate.io docs: https://www.gate.io/docs/developers/apiv4/en/#query-user-s-lending-order-list
    pub async fn list_earnuni_lends(&self) -> RestResult<Vec<LendInfo>> {
        self.send_get_request(LENDS_ENDPOINT, Option::<&()>::None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_lend_request_serializes() {
        let r = CreateLendRequest {
            currency: "BTC".into(),
            amount: "1".into(),
            rate: Some("0.0001".into()),
            use_best_rate: Some(false),
        };
        let js = serde_json::to_string(&r).expect("serialize");
        assert!(js.contains("BTC"));
        assert!(js.contains("0.0001"));
    }

    #[test]
    fn lend_info_deserializes() {
        let json = r#"{"order_id":"123","currency":"BTC","amount":"1","rate":"0.0001","status":"open","create_time":1673247054000}"#;
        let li: LendInfo = serde_json::from_str(json).expect("deserialize");
        assert_eq!(li.currency, "BTC");
        assert_eq!(li.order_id.as_deref(), Some("123"));
        assert_eq!(li.status.as_deref(), Some("open"));
    }
}
