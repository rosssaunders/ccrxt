use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const INTEREST_STATUS_ENDPOINT: &str = "/earn/uni/interest_status"; // append /{currency}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InterestStatus {
    pub currency: String,

    /// Whether interest has been enabled for this currency
    pub enabled: bool,

    /// Optional reason or message
    #[serde(default)]
    pub message: Option<String>,
}

impl RestClient {
    /// GET /earn/uni/interest_status/{currency}
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-currency-interest-compounding-status)
    pub async fn get_earnuni_interest_status(&self, currency: &str) -> RestResult<InterestStatus> {
        let endpoint = format!("{}/{}", INTEREST_STATUS_ENDPOINT, currency);
        self.send_get_request(&endpoint, Option::<&()>::None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interest_status_deserializes() {
        let json = r#"{"currency":"BTC","enabled":true,"message":"ok"}"#;
        let s: InterestStatus = serde_json::from_str(json).expect("deserialize");
        assert!(s.enabled);
        assert_eq!(s.message.as_deref(), Some("ok"));
    }
}
