use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const RATE_ENDPOINT: &str = "/earn/uni/rate";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RateInfo {
    pub currency: String,

    /// Current annualized rate (string decimal)
    pub annualized_rate: Option<String>,

    /// Hourly rate if returned
    #[serde(default)]
    pub hourly_rate: Option<String>,

    /// timestamp ms
    #[serde(default)]
    pub timestamp: Option<i64>,
}

impl RestClient {
    /// GET /earn/uni/rate/{currency}
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#currency-estimated-annualized-interest-rate)
    pub async fn get_earnuni_rate(&self, currency: &str) -> RestResult<RateInfo> {
        let endpoint = format!("{}/{}", RATE_ENDPOINT, currency);
        self.send_get_request(&endpoint, Option::<&()>::None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_info_deserializes() {
        let json = r#"{"currency":"BTC","annualized_rate":"0.05","hourly_rate":"0.0001","timestamp":1673247054000}"#;
        let r: RateInfo = serde_json::from_str(json).expect("deserialize");
        assert_eq!(r.currency, "BTC");
        assert_eq!(r.hourly_rate.as_deref(), Some("0.0001"));
    }
}
