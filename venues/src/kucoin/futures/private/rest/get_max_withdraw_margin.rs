use serde::Serialize;

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for getting maximum withdrawable margin
const GET_MAX_WITHDRAW_MARGIN_ENDPOINT: &str = "/api/v1/margin/maxWithdrawMargin";

/// Request parameters for getting maximum withdrawable margin.
#[derive(Debug, Clone, Serialize)]
pub struct GetMaxWithdrawMarginRequest {
    /// Trading symbol (e.g., "XBTUSDTM"). Required parameter.
    pub symbol: String,
}

/// Response containing the maximum withdrawable margin amount.
pub type GetMaxWithdrawMarginResponse = String;

impl super::RestClient {
    /// Get Maximum Withdrawable Margin
    ///
    /// Get the maximum withdrawable margin amount for a given symbol.
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/positions/get-maximum-withdraw-margin)
    ///
    /// Rate limit: 9
    ///
    /// # Arguments
    /// * `request` - The maximum withdrawable margin request parameters
    ///
    /// # Returns
    /// Maximum withdrawable margin amount as a string
    pub async fn get_max_withdraw_margin(
        &self,
        request: GetMaxWithdrawMarginRequest,
    ) -> Result<(RestResponse<GetMaxWithdrawMarginResponse>, ResponseHeaders)> {
        self.get_with_request(GET_MAX_WITHDRAW_MARGIN_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_withdraw_margin_request_creation() {
        let request = GetMaxWithdrawMarginRequest {
            symbol: "XBTUSDTM".to_string(),
        };
        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_get_max_withdraw_margin_response_deserialization() {
        let json = r#""21.1135719252""#;
        let response: GetMaxWithdrawMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response, "21.1135719252");
    }

    #[test]
    fn test_request_serialization() {
        let request = GetMaxWithdrawMarginRequest {
            symbol: "ETHUSDTM".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDTM");
    }

    #[test]
    fn test_various_symbols() {
        let symbols = ["XBTUSDTM", "ETHUSDTM", "ADAUSDTM", "DOTUSDTM"];

        for symbol in symbols.iter() {
            let request = GetMaxWithdrawMarginRequest {
                symbol: symbol.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], *symbol);
        }
    }

    #[test]
    fn test_response_with_zero_margin() {
        let json = r#""0""#;
        let response: GetMaxWithdrawMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response, "0");
    }

    #[test]
    fn test_response_with_large_margin() {
        let json = r#""999999.123456789""#;
        let response: GetMaxWithdrawMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response, "999999.123456789");
    }

    #[test]
    fn test_response_with_small_margin() {
        let json = r#""0.00000001""#;
        let response: GetMaxWithdrawMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response, "0.00000001");
    }

    #[test]
    fn test_request_field_types() {
        let request = GetMaxWithdrawMarginRequest {
            symbol: "XBTUSDTM".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["symbol"].is_string());
    }

    #[test]
    fn test_response_various_decimal_places() {
        let margin_values = [
            "21.1",
            "21.12",
            "21.123",
            "21.1234",
            "21.12345",
            "21.123456",
            "21.1234567",
            "21.12345678",
        ];

        for margin_value in margin_values.iter() {
            let json = format!(r#""{}""#, margin_value);
            let response: GetMaxWithdrawMarginResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response, *margin_value);
        }
    }

    #[test]
    fn test_symbol_case_sensitivity() {
        let request = GetMaxWithdrawMarginRequest {
            symbol: "xbtusdtm".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "xbtusdtm");
    }

    #[test]
    fn test_long_symbol_names() {
        let request = GetMaxWithdrawMarginRequest {
            symbol: "VERYLONGSYMBOLNAMEFORTESTING".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "VERYLONGSYMBOLNAMEFORTESTING");
    }

    #[test]
    fn test_response_integer_string() {
        let json = r#""100""#;
        let response: GetMaxWithdrawMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response, "100");
    }

    #[test]
    fn test_response_negative_margin() {
        let json = r#""-5.123""#;
        let response: GetMaxWithdrawMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response, "-5.123");
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(
            GET_MAX_WITHDRAW_MARGIN_ENDPOINT,
            "/api/v1/margin/maxWithdrawMargin"
        );
    }

    #[test]
    fn test_request_completeness() {
        let request = GetMaxWithdrawMarginRequest {
            symbol: "XBTUSDTM".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();

        // Should have exactly 1 field
        assert_eq!(json.as_object().unwrap().len(), 1);
        assert!(json.get("symbol").is_some());
    }
}
