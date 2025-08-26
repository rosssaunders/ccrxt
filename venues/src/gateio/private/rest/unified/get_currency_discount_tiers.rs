use super::{RestClient, RestResult, discount_tiers::CurrencyDiscountTier};

const CURRENCY_DISCOUNT_TIERS_ENDPOINT: &str = "/unified/currency_discount_tiers";

impl RestClient {
    /// Get currency discount tiers
    ///
    /// This endpoint returns discount tier information for currencies.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-currency-discount-tiers)
    pub async fn get_currency_discount_tiers(&self) -> RestResult<Vec<CurrencyDiscountTier>> {
        self.get(CURRENCY_DISCOUNT_TIERS_ENDPOINT).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_discount_tier_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "tier": 1,
            "discount_rate": "0.95",
            "min_amount": "0",
            "max_amount": "10"
        }"#;

        let tier: CurrencyDiscountTier = serde_json::from_str(json).unwrap();
        assert_eq!(tier.currency, "BTC");
        assert_eq!(tier.tier, 1);
        assert_eq!(tier.discount_rate, "0.95");
        assert_eq!(tier.min_amount, "0");
        assert_eq!(tier.max_amount, "10");
    }

    #[test]
    fn test_currency_discount_tiers_array_deserialization() {
        let json = r#"[
            {
                "currency": "BTC",
                "tier": 1,
                "discount_rate": "0.95",
                "min_amount": "0",
                "max_amount": "10"
            },
            {
                "currency": "BTC",
                "tier": 2,
                "discount_rate": "0.90",
                "min_amount": "10",
                "max_amount": "100"
            }
        ]"#;

        let tiers: Vec<CurrencyDiscountTier> = serde_json::from_str(json).unwrap();
        assert_eq!(tiers.len(), 2);
        assert_eq!(tiers[0].tier, 1);
        assert_eq!(tiers[1].tier, 2);
        assert_eq!(tiers[0].discount_rate, "0.95");
        assert_eq!(tiers[1].discount_rate, "0.90");
    }

    #[test]
    fn test_currency_discount_tiers_endpoint() {
        assert_eq!(
            CURRENCY_DISCOUNT_TIERS_ENDPOINT,
            "/unified/currency_discount_tiers"
        );
    }
}
