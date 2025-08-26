use super::{
    RestClient, RestResult,
    unified_margin::{UnifiedMarginCurrencyPair, UnifiedMarginCurrencyPairsRequest},
};

const MARGIN_UNI_CURRENCY_PAIRS_ENDPOINT: &str = "/margin/uni/currency_pairs";

impl RestClient {
    /// Get unified margin currency pairs
    ///
    /// This endpoint returns currency pairs available for unified margin trading.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-unified-margin-currency-pairs)
    pub async fn get_unified_margin_currency_pairs(
        &self,
        params: UnifiedMarginCurrencyPairsRequest,
    ) -> RestResult<Vec<UnifiedMarginCurrencyPair>> {
        self.get_with_query(MARGIN_UNI_CURRENCY_PAIRS_ENDPOINT, &params)
            .await
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
    fn test_unified_margin_currency_pairs_request_serialization() {
        let request = UnifiedMarginCurrencyPairsRequest {
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_unified_margin_currency_pairs_request_default() {
        let request = UnifiedMarginCurrencyPairsRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        assert!(!json.as_object().unwrap().contains_key("currency_pair"));
    }

    #[test]
    fn test_unified_margin_currency_pair_deserialization() {
        let json = r#"{
            "currency_pair": "BTC_USDT",
            "base": "BTC",
            "quote": "USDT",
            "leverage": "5",
            "min_amount": "0.001",
            "max_amount": "100",
            "price_precision": 2,
            "amount_precision": 4,
            "trade_status": "tradable",
            "sell_start": 1640995200,
            "buy_start": 1640995200
        }"#;

        let pair: UnifiedMarginCurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.currency_pair, "BTC_USDT");
        assert_eq!(pair.base, "BTC");
        assert_eq!(pair.quote, "USDT");
        assert_eq!(pair.leverage, "5");
        assert_eq!(pair.min_amount, "0.001");
        assert_eq!(pair.max_amount, "100");
        assert_eq!(pair.price_precision, 2);
        assert_eq!(pair.amount_precision, 4);
        assert_eq!(pair.trade_status, "tradable");
    }
}
