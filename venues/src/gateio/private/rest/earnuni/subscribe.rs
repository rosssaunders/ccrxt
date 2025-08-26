use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const SUBSCRIBE_ENDPOINT: &str = "/earn/uni/subscribe";

/// Request to subscribe (lend) to an EarnUni product.
#[derive(Debug, Clone, Serialize)]
pub struct SubscribeRequest {
    /// Product identifier to subscribe to.
    pub product_id: String,

    /// Amount to subscribe.
    pub amount: String,
}

/// Response returned when creating a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeResponse {
    /// Order identifier created by the platform.
    pub order_id: String,

    /// Status string from the API.
    pub status: String,
}

impl RestClient {
    /// Subscribe (lend) to an EarnUni product.
    pub async fn subscribe_earnuni(&self, req: SubscribeRequest) -> RestResult<SubscribeResponse> {
        // verb-specific function (POST)
        self.send_post_request(SUBSCRIBE_ENDPOINT, Some(&req)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subscribe_request_serializes() {
        let r = SubscribeRequest {
            product_id: "p1".into(),
            amount: "1.23".into(),
        };
        let js = serde_json::to_string(&r).expect("serialize");
        assert!(js.contains("p1"));
        assert!(js.contains("1.23"));
    }

    #[test]
    fn subscribe_response_deserializes() {
        let json = r#"{"order_id":"o1","status":"ok"}"#;
        let s: SubscribeResponse = serde_json::from_str(json).expect("deserialize");
        assert_eq!(s.order_id, "o1");
        assert_eq!(s.status, "ok");
    }
}
