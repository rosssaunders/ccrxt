use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const CURRENCIES_ENDPOINT: &str = "/earn/uni/currencies";
const CURRENCY_ENDPOINT: &str = "/earn/uni/currencies"; // append /{currency}

/// Currency metadata returned by the Earn/Uni endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CurrencyInfo {
    pub currency: String,
    pub name: Option<String>,
    /// Display precision for the currency
    pub precision: Option<i32>,
    /// Minimum withdraw amount (string because many amounts are decimals)
    #[serde(rename = "min_withdraw_amount", default)]
    pub min_withdraw_amount: Option<String>,
    /// Whether the currency is currently enabled for Earn
    #[serde(default)]
    pub enabled: Option<bool>,
}

impl RestClient {
    /// GET /earn/uni/currencies
    ///
    /// Gate.io docs: https://www.gate.io/docs/developers/apiv4/en/#query-lending-currency-list
    pub async fn list_earnuni_currencies(&self) -> RestResult<Vec<CurrencyInfo>> {
        self.send_get_request(CURRENCIES_ENDPOINT, Option::<&()>::None)
            .await
    }

    /// GET /earn/uni/currencies/{currency}
    ///
    /// Gate.io docs: https://www.gate.io/docs/developers/apiv4/en/#query-single-lending-currency-details
    pub async fn get_earnuni_currency(&self, currency: &str) -> RestResult<CurrencyInfo> {
        let endpoint = format!("{}/{}", CURRENCY_ENDPOINT, currency);
        self.send_get_request(&endpoint, Option::<&()>::None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_deserializes() {
        let json = r#"[{"currency":"BTC","name":"Bitcoin","precision":8,"min_withdraw_amount":"0.001","enabled":true}]"#;
        let v: Vec<CurrencyInfo> = serde_json::from_str(json).expect("deserialize");
        assert_eq!(v[0].currency, "BTC");
        assert_eq!(v[0].name.as_deref(), Some("Bitcoin"));
        assert_eq!(v[0].precision, Some(8));
        assert_eq!(v[0].min_withdraw_amount.as_deref(), Some("0.001"));
        assert_eq!(v[0].enabled, Some(true));
    }
}
