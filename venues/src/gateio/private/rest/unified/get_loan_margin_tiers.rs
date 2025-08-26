use super::{RestClient, RestResult, discount_tiers::LoanMarginTier};

const LOAN_MARGIN_TIERS_ENDPOINT: &str = "/unified/loan_margin_tiers";

impl RestClient {
    /// Get loan margin tiers
    ///
    /// This endpoint returns loan margin tier information.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-loan-margin-tiers)
    pub async fn get_loan_margin_tiers(&self) -> RestResult<Vec<LoanMarginTier>> {
        self.get(LOAN_MARGIN_TIERS_ENDPOINT).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loan_margin_tier_deserialization() {
        let json = r#"{
            "currency": "USDT",
            "tier": 1,
            "margin_rate": "0.05",
            "min_amount": "0",
            "max_amount": "10000"
        }"#;

        let tier: LoanMarginTier = serde_json::from_str(json).unwrap();
        assert_eq!(tier.currency, "USDT");
        assert_eq!(tier.tier, 1);
        assert_eq!(tier.margin_rate, "0.05");
        assert_eq!(tier.min_amount, "0");
        assert_eq!(tier.max_amount, "10000");
    }

    #[test]
    fn test_loan_margin_tiers_array_deserialization() {
        let json = r#"[
            {
                "currency": "USDT",
                "tier": 1,
                "margin_rate": "0.05",
                "min_amount": "0",
                "max_amount": "10000"
            },
            {
                "currency": "USDT",
                "tier": 2,
                "margin_rate": "0.08",
                "min_amount": "10000",
                "max_amount": "100000"
            },
            {
                "currency": "USDT",
                "tier": 3,
                "margin_rate": "0.10",
                "min_amount": "100000",
                "max_amount": "1000000"
            }
        ]"#;

        let tiers: Vec<LoanMarginTier> = serde_json::from_str(json).unwrap();
        assert_eq!(tiers.len(), 3);
        assert_eq!(tiers[0].tier, 1);
        assert_eq!(tiers[1].tier, 2);
        assert_eq!(tiers[2].tier, 3);
        assert_eq!(tiers[0].margin_rate, "0.05");
        assert_eq!(tiers[1].margin_rate, "0.08");
        assert_eq!(tiers[2].margin_rate, "0.10");
    }

    #[test]
    fn test_loan_margin_tiers_endpoint() {
        assert_eq!(LOAN_MARGIN_TIERS_ENDPOINT, "/unified/loan_margin_tiers");
    }

    #[test]
    fn test_loan_margin_tier_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "BNB"];

        for currency in currencies {
            let json = format!(
                r#"{{
                "currency": "{}",
                "tier": 1,
                "margin_rate": "0.05",
                "min_amount": "0",
                "max_amount": "1000"
            }}"#,
                currency
            );

            let tier: LoanMarginTier = serde_json::from_str(&json).unwrap();
            assert_eq!(tier.currency, currency);
        }
    }
}
