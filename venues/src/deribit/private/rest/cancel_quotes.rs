use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::deribit::{
    Currency, CurrencyPair, EndpointType, InstrumentKind, JsonRpcResult, RestResult,
};

/// REST API endpoint constant
const CANCEL_QUOTES_ENDPOINT: &str = "private/cancel_quotes";

/// Cancel type for cancel quotes endpoint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CancelType {
    /// Cancel quotes within a Delta range
    Delta,
    /// Cancel quotes by a specific Quote Set identifier
    QuoteSetId,
    /// Cancel all quotes associated with a particular instrument
    Instrument,
    /// Cancel all quotes for a certain instrument kind
    InstrumentKind,
    /// Cancel all quotes in a specified currency
    Currency,
    /// Cancel all quotes in a specified currency pair
    CurrencyPair,
    /// Cancel all quotes
    All,
}

/// Request parameters for cancel quotes endpoint
#[derive(Debug, Clone, Serialize)]
pub struct CancelQuotesRequest {
    /// Type of cancel criteria (required)
    pub cancel_type: CancelType,

    /// When detailed is set to true output format is changed (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,

    /// Whether or not to reject incoming quotes for 1 second after cancelling (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_quotes: Option<bool>,

    /// Min delta to cancel by delta (for cancel_type: delta)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_delta: Option<f64>,

    /// Max delta to cancel by delta (for cancel_type: delta)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_delta: Option<f64>,

    /// Unique identifier for the Quote set (for cancel_type: quote_set_id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_set_id: Option<String>,

    /// Instrument name (for cancel_type: instrument)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,

    /// Instrument kind (for cancel_type: instrument_kind)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<InstrumentKind>,

    /// The currency symbol (for cancel_type: currency)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The currency pair symbol (for cancel_type: currency_pair)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<CurrencyPair>,
}

/// Response for cancel quotes endpoint
pub type CancelQuotesResponse = JsonRpcResult<i64>;

impl RestClient {
    /// Cancel quotes based on the provided type
    ///
    /// This method cancels quotes based on the provided type. `delta` cancels quotes within a Delta
    /// range defined by `min_delta` and `max_delta`. `quote_set_id` cancels quotes by a
    /// specific Quote Set identifier. `instrument` cancels all quotes associated with a
    /// particular instrument. `kind` cancels all quotes for a certain instrument kind.
    /// `currency` cancels all quotes in a specified currency. `currency_pair` cancels
    /// all quotes in a specified currency pair. `all` cancels all quotes.
    ///
    /// This endpoint requires trade:read_write scope.
    ///
    /// [docs]: https://docs.deribit.com/v2/#private-cancel_quotes
    ///
    /// Rate limit: Matching engine endpoint (tier-based rate limiting)
    /// Scope: trade:read_write
    ///
    /// # Arguments
    /// * `cancel_type` - Type of cancel criteria
    /// * `detailed` - Optional flag for detailed output format
    /// * `freeze_quotes` - Optional flag to reject incoming quotes for 1 second after cancelling
    /// * `min_delta` - Min delta to cancel by delta (for cancel_type: delta)
    /// * `max_delta` - Max delta to cancel by delta (for cancel_type: delta)
    /// * `quote_set_id` - Unique identifier for the Quote set (for cancel_type: quote_set_id)
    /// * `instrument_name` - Instrument name (for cancel_type: instrument)
    /// * `kind` - Instrument kind (for cancel_type: instrument_kind)
    /// * `currency` - The currency symbol (for cancel_type: currency)
    /// * `currency_pair` - The currency pair symbol (for cancel_type: currency_pair)
    ///
    /// # Returns
    /// Result with total number of successfully cancelled quotes
    pub async fn cancel_quotes(
        &self,
        request: CancelQuotesRequest,
    ) -> RestResult<CancelQuotesResponse> {
        self.send_signed_request(
            CANCEL_QUOTES_ENDPOINT,
            &request,
            EndpointType::MatchingEngine,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
/// REST API endpoint constant
    use serde_json::{Value, json};

    use super::*;
    use crate::deribit::AccountTier;

    
    use crate::deribit::private::rest::credentials::Credentials;
    use rest::secrets::SecretString;


    #[test]
    fn test_cancel_type_serialization() {
        let delta = CancelType::Delta;
        let quote_set_id = CancelType::QuoteSetId;
        let instrument = CancelType::Instrument;
        let instrument_kind = CancelType::InstrumentKind;
        let currency = CancelType::Currency;
        let currency_pair = CancelType::CurrencyPair;
        let all = CancelType::All;

        assert_eq!(serde_json::to_string(&delta).unwrap(), "\"delta\"");
        assert_eq!(
            serde_json::to_string(&quote_set_id).unwrap(),
            "\"quote_set_id\""
        );
        assert_eq!(
            serde_json::to_string(&instrument).unwrap(),
            "\"instrument\""
        );
        assert_eq!(
            serde_json::to_string(&instrument_kind).unwrap(),
            "\"instrument_kind\""
        );
        assert_eq!(serde_json::to_string(&currency).unwrap(), "\"currency\"");
        assert_eq!(
            serde_json::to_string(&currency_pair).unwrap(),
            "\"currency_pair\""
        );
        assert_eq!(serde_json::to_string(&all).unwrap(), "\"all\"");

        let delta_from_json: CancelType = serde_json::from_str("\"delta\"").unwrap();
        let quote_set_id_from_json: CancelType = serde_json::from_str("\"quote_set_id\"").unwrap();
        let instrument_from_json: CancelType = serde_json::from_str("\"instrument\"").unwrap();
        let instrument_kind_from_json: CancelType =
            serde_json::from_str("\"instrument_kind\"").unwrap();
        let currency_from_json: CancelType = serde_json::from_str("\"currency\"").unwrap();
        let currency_pair_from_json: CancelType =
            serde_json::from_str("\"currency_pair\"").unwrap();
        let all_from_json: CancelType = serde_json::from_str("\"all\"").unwrap();

        assert_eq!(delta_from_json, CancelType::Delta);
        assert_eq!(quote_set_id_from_json, CancelType::QuoteSetId);
        assert_eq!(instrument_from_json, CancelType::Instrument);
        assert_eq!(instrument_kind_from_json, CancelType::InstrumentKind);
        assert_eq!(currency_from_json, CancelType::Currency);
        assert_eq!(currency_pair_from_json, CancelType::CurrencyPair);
        assert_eq!(all_from_json, CancelType::All);
    }

    #[test]
    fn test_request_serialization_delta() {
        let request = CancelQuotesRequest {
            cancel_type: CancelType::Delta,
            detailed: Some(true),
            freeze_quotes: Some(false),
            min_delta: Some(0.1),
            max_delta: Some(0.9),
            quote_set_id: None,
            instrument_name: None,
            kind: None,
            currency: None,
            currency_pair: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("cancel_type").unwrap(), "delta");
        assert_eq!(json_value.get("detailed").unwrap(), true);
        assert_eq!(json_value.get("freeze_quotes").unwrap(), false);
        assert_eq!(json_value.get("min_delta").unwrap(), 0.1);
        assert_eq!(json_value.get("max_delta").unwrap(), 0.9);
        assert!(json_value.get("quote_set_id").is_none());
        assert!(json_value.get("instrument_name").is_none());
        assert!(json_value.get("kind").is_none());
        assert!(json_value.get("currency").is_none());
        assert!(json_value.get("currency_pair").is_none());
    }

    #[test]
    fn test_request_serialization_quote_set_id() {
        let request = CancelQuotesRequest {
            cancel_type: CancelType::QuoteSetId,
            detailed: None,
            freeze_quotes: None,
            min_delta: None,
            max_delta: None,
            quote_set_id: Some("test_quote_set_123".to_string()),
            instrument_name: None,
            kind: None,
            currency: None,
            currency_pair: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("cancel_type").unwrap(), "quote_set_id");
        assert_eq!(
            json_value.get("quote_set_id").unwrap(),
            "test_quote_set_123"
        );
        assert!(json_value.get("detailed").is_none());
        assert!(json_value.get("freeze_quotes").is_none());
        assert!(json_value.get("min_delta").is_none());
        assert!(json_value.get("max_delta").is_none());
    }

    #[test]
    fn test_request_serialization_instrument() {
        let request = CancelQuotesRequest {
            cancel_type: CancelType::Instrument,
            detailed: None,
            freeze_quotes: None,
            min_delta: None,
            max_delta: None,
            quote_set_id: None,
            instrument_name: Some("BTC-PERPETUAL".to_string()),
            kind: None,
            currency: None,
            currency_pair: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("cancel_type").unwrap(), "instrument");
        assert_eq!(json_value.get("instrument_name").unwrap(), "BTC-PERPETUAL");
    }

    #[test]
    fn test_request_serialization_instrument_kind() {
        let request = CancelQuotesRequest {
            cancel_type: CancelType::InstrumentKind,
            detailed: None,
            freeze_quotes: None,
            min_delta: None,
            max_delta: None,
            quote_set_id: None,
            instrument_name: None,
            kind: Some(InstrumentKind::Option),
            currency: None,
            currency_pair: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("cancel_type").unwrap(), "instrument_kind");
        assert_eq!(json_value.get("kind").unwrap(), "option");
    }

    #[test]
    fn test_request_serialization_currency() {
        let request = CancelQuotesRequest {
            cancel_type: CancelType::Currency,
            detailed: None,
            freeze_quotes: None,
            min_delta: None,
            max_delta: None,
            quote_set_id: None,
            instrument_name: None,
            kind: None,
            currency: Some(Currency::BTC),
            currency_pair: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("cancel_type").unwrap(), "currency");
        assert_eq!(json_value.get("currency").unwrap(), "BTC");
    }

    #[test]
    fn test_request_serialization_currency_pair() {
        let request = CancelQuotesRequest {
            cancel_type: CancelType::CurrencyPair,
            detailed: None,
            freeze_quotes: None,
            min_delta: None,
            max_delta: None,
            quote_set_id: None,
            instrument_name: None,
            kind: None,
            currency: None,
            currency_pair: Some(CurrencyPair::BtcUsdc),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("cancel_type").unwrap(), "currency_pair");
        assert_eq!(json_value.get("currency_pair").unwrap(), "btc_usdc");
    }

    #[test]
    fn test_request_serialization_all() {
        let request = CancelQuotesRequest {
            cancel_type: CancelType::All,
            detailed: Some(false),
            freeze_quotes: Some(true),
            min_delta: None,
            max_delta: None,
            quote_set_id: None,
            instrument_name: None,
            kind: None,
            currency: None,
            currency_pair: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();
        let json_value: Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(json_value.get("cancel_type").unwrap(), "all");
        assert_eq!(json_value.get("detailed").unwrap(), false);
        assert_eq!(json_value.get("freeze_quotes").unwrap(), true);
    }

    #[test]
    fn test_response_deserialization() {
        let response_json = json!({
            "id": 123,
            "jsonrpc": "2.0",
            "result": 5
        });

        let response: CancelQuotesResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 123);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 5);
    }

    #[test]
    fn test_response_deserialization_zero_cancelled() {
        let response_json = json!({
            "id": 456,
            "jsonrpc": "2.0",
            "result": 0
        });

        let response: CancelQuotesResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.id, 456);
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, 0);
    }

    #[tokio::test]
    async fn test_cancel_quotes_method_exists() {
        // Test that the method exists and compiles without needing to call it
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = crate::deribit::RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        // Test that we can get a function reference to the method
        let _ = RestClient::cancel_quotes;

        // Verify the client exists
        let _ = &rest_client;

        println!("cancel_quotes method is accessible and properly typed");
    }
}
