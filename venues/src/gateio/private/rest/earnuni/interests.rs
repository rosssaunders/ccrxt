use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const INTERESTS_ENDPOINT: &str = "/earn/uni/interests"; // append /{currency}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InterestInfo {
    pub currency: String,

    /// Current daily interest (string decimal)
    #[serde(default)]
    pub daily_interest: Option<String>,

    /// Annualized APY if provided
    #[serde(default)]
    pub apy: Option<String>,

    /// Timestamp of the rate (ms)
    #[serde(default)]
    pub timestamp: Option<i64>,
}

impl RestClient {
    /// GET /earn/uni/interests/{currency}
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-total-interest-income-for-specified-currency)
    pub async fn get_earnuni_interests(&self, currency: &str) -> RestResult<InterestInfo> {
        let endpoint = format!("{}/{}", INTERESTS_ENDPOINT, currency);
        self.send_get_request(&endpoint, Option::<&()>::None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interest_info_deserializes() {
        let json = r#"{"currency":"BTC","daily_interest":"0.0001","apy":"0.0365","timestamp":1673247054000}"#;
        let i: InterestInfo = serde_json::from_str(json).expect("deserialize");
        assert_eq!(i.currency, "BTC");
        assert_eq!(i.daily_interest.as_deref(), Some("0.0001"));
        assert_eq!(i.apy.as_deref(), Some("0.0365"));
    }
}
