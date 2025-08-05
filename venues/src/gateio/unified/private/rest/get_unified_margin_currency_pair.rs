use super::{RestClient, unified_margin::UnifiedMarginCurrencyPair};

const MARGIN_UNI_CURRENCY_PAIRS_ENDPOINT: &str = "/margin/uni/currency_pairs";

impl RestClient {
    /// Get a specific unified margin currency pair
    ///
    /// This endpoint returns details for a specific unified margin currency pair.
    ///
    /// # API Documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-unified-margin-currency-pair>
    pub async fn get_unified_margin_currency_pair(
        &self,
        currency_pair: &str,
    ) -> crate::gateio::unified::RestResult<UnifiedMarginCurrencyPair> {
        let endpoint = format!("{}/{}", MARGIN_UNI_CURRENCY_PAIRS_ENDPOINT, currency_pair);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_margin_uni_currency_pairs_endpoint() {
        assert_eq!(
            MARGIN_UNI_CURRENCY_PAIRS_ENDPOINT,
            "/margin/uni/currency_pairs"
        );
    }

    #[test]
    fn test_endpoint_formatting() {
        let currency_pair = "BTC_USDT";
        let endpoint = format!("{}/{}", MARGIN_UNI_CURRENCY_PAIRS_ENDPOINT, currency_pair);
        assert_eq!(endpoint, "/margin/uni/currency_pairs/BTC_USDT");
    }

    #[test]
    fn test_unified_margin_currency_pair_deserialization() {
        let json = r#"{
            "currency_pair": "ETH_USDT",
            "base": "ETH",
            "quote": "USDT",
            "leverage": "3",
            "min_amount": "0.01",
            "max_amount": "1000",
            "price_precision": 2,
            "amount_precision": 3,
            "trade_status": "tradable",
            "sell_start": 1640995200,
            "buy_start": 1640995200
        }"#;

        let pair: UnifiedMarginCurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.currency_pair, "ETH_USDT");
        assert_eq!(pair.base, "ETH");
        assert_eq!(pair.quote, "USDT");
        assert_eq!(pair.leverage, "3");
    }
}
