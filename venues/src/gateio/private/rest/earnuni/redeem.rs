use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const REDEEM_ENDPOINT: &str = "/earn/uni/redeem";

/// Request to redeem (withdraw) from an EarnUni order.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemRequest {
    /// Order ID to redeem from.
    pub order_id: String,

    /// Amount to redeem.
    pub amount: String,
}

/// Response returned by a redeem request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedeemResponse {
    /// Order identifier.
    pub order_id: String,

    /// Status string.
    pub status: String,
}

impl RestClient {
    /// Redeem amount from an EarnUni order.
    pub async fn redeem_earnuni(&self, req: RedeemRequest) -> RestResult<RedeemResponse> {
        // verb-specific function (POST)
        self.send_post_request(REDEEM_ENDPOINT, Some(&req)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redeem_request_serializes() {
        let r = RedeemRequest {
            order_id: "o1".into(),
            amount: "0.5".into(),
        };
        let js = serde_json::to_string(&r).expect("serialize");
        assert!(js.contains("o1"));
        assert!(js.contains("0.5"));
    }

    #[test]
    fn redeem_response_deserializes() {
        let json = r#"{"order_id":"o1","status":"done"}"#;
        let s: RedeemResponse = serde_json::from_str(json).expect("deserialize");
        assert_eq!(s.order_id, "o1");
        assert_eq!(s.status, "done");
    }
}
