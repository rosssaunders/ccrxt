use serde::Deserialize;

use super::{RestClient, RestResult};

const CURRENCIES_ENDPOINT: &str = "/loan/collateral/currencies";

/// Supported borrowing and collateral currency information.
///
/// See [docs](https://www.gate.io/docs/developers/apiv4/en/#query-supported-borrowing-and-collateral-currencies)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CollateralLoanCurrencyInfo {
    /// Currency code
    pub currency: String,

    /// Whether it can be borrowed
    pub borrowable: bool,

    /// Whether it can be used as collateral
    pub collateral: bool,
}

impl RestClient {
    /// Query supported borrowing and collateral currencies
    ///
    /// Returns a list of supported currencies for collateral loans.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-supported-borrowing-and-collateral-currencies)
    ///
    /// # Returns
    /// List of supported currency info
    pub async fn list_collateral_loan_currencies(
        &self,
    ) -> RestResult<Vec<CollateralLoanCurrencyInfo>> {
        self.send_get_request::<Vec<CollateralLoanCurrencyInfo>, ()>(CURRENCIES_ENDPOINT, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_response() {
        let json = r#"[
            { "currency": "BTC", "borrowable": true, "collateral": true },
            { "currency": "USDT", "borrowable": true, "collateral": false }
        ]"#;
        let resp: Vec<CollateralLoanCurrencyInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(resp[0].currency, "BTC");
        assert!(resp[0].borrowable);
        assert!(resp[0].collateral);
        assert_eq!(resp[1].currency, "USDT");
        assert!(resp[1].borrowable);
        assert!(!resp[1].collateral);
    }
}
