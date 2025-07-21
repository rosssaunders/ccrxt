use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const MARK_ENDPOINT: &str = "/eapi/v1/mark";

/// Request parameters for mark price
#[derive(Debug, Clone, Serialize, Default)]
pub struct MarkPriceRequest {
    /// Option trading pair, e.g BTC-200730-9000-C
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Option mark price and Greek information
#[derive(Debug, Clone, Deserialize)]
pub struct MarkPriceResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Mark price
    #[serde(rename = "markPrice")]
    pub mark_price: Decimal,

    /// Implied volatility Buy
    #[serde(rename = "bidIV")]
    pub bid_iv: Decimal,

    /// Implied volatility Sell
    #[serde(rename = "askIV")]
    pub ask_iv: Decimal,

    /// Implied volatility mark
    #[serde(rename = "markIV")]
    pub mark_iv: Decimal,

    /// Delta
    #[serde(rename = "delta")]
    pub delta: Decimal,

    /// Theta
    #[serde(rename = "theta")]
    pub theta: Decimal,

    /// Gamma
    #[serde(rename = "gamma")]
    pub gamma: Decimal,

    /// Vega
    #[serde(rename = "vega")]
    pub vega: Decimal,

    /// Current highest buy price
    #[serde(rename = "highPriceLimit")]
    pub high_price_limit: Decimal,

    /// Current lowest sell price
    #[serde(rename = "lowPriceLimit")]
    pub low_price_limit: Decimal,

    /// Risk free rate
    #[serde(rename = "riskFreeInterest")]
    pub risk_free_interest: Decimal,
}

impl RestClient {
    /// Get option mark price
    ///
    /// Returns option mark price and Greek info.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Option-Mark-Price)
    /// Method: GET /eapi/v1/mark
    /// Weight: 5
    /// Security: None
    pub async fn get_mark_price(
        &self,
        params: MarkPriceRequest,
    ) -> RestResult<Vec<MarkPriceResponse>> {
        if params.symbol.is_none() {
            self.send_public_request(MARK_ENDPOINT, reqwest::Method::GET, None::<()>, 5)
                .await
        } else {
            self.send_public_request(MARK_ENDPOINT, reqwest::Method::GET, Some(params), 5)
                .await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_mark_price_request_serialization_with_symbol() {
        let request = MarkPriceRequest {
            symbol: Some("BTC-240329-70000-C".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
    }

    #[test]
    fn test_mark_price_request_serialization_without_symbol() {
        let request = MarkPriceRequest { symbol: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(!serialized.contains("symbol"));
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_mark_price_request_serialization_default() {
        let request = MarkPriceRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(!serialized.contains("symbol"));
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_mark_price_request_serialization_various_symbols() {
        let symbols = vec![
            "BTC-240329-70000-C",
            "BTC-240329-70000-P",
            "ETH-240329-3000-C",
            "ETH-240329-3000-P",
            "BNB-240329-500-C",
        ];

        for symbol in symbols {
            let request = MarkPriceRequest {
                symbol: Some(symbol.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("symbol={}", symbol)));
        }
    }

    #[test]
    fn test_mark_price_request_clone() {
        let request = MarkPriceRequest {
            symbol: Some("BTC-240329-70000-C".to_string()),
        };

        let cloned = request.clone();
        assert_eq!(request.symbol, cloned.symbol);
    }

    #[test]
    fn test_mark_price_request_debug() {
        let request = MarkPriceRequest {
            symbol: Some("BTC-240329-70000-C".to_string()),
        };

        let debug_output = format!("{:?}", request);
        assert!(debug_output.contains("MarkPriceRequest"));
        assert!(debug_output.contains("BTC-240329-70000-C"));
    }

    #[test]
    fn test_mark_price_response_deserialization() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "markPrice": "7150.50000000",
            "bidIV": "0.65000000",
            "askIV": "0.70000000",
            "markIV": "0.67500000",
            "delta": "0.55000000",
            "theta": "-25.50000000",
            "gamma": "0.00012000",
            "vega": "45.75000000",
            "highPriceLimit": "7800.00000000",
            "lowPriceLimit": "6500.00000000",
            "riskFreeInterest": "0.05000000"
        }"#;

        let response: MarkPriceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-70000-C");
        assert_eq!(response.mark_price, dec!(7150.50000000));
        assert_eq!(response.bid_iv, dec!(0.65000000));
        assert_eq!(response.ask_iv, dec!(0.70000000));
        assert_eq!(response.mark_iv, dec!(0.67500000));
        assert_eq!(response.delta, dec!(0.55000000));
        assert_eq!(response.theta, dec!(-25.50000000));
        assert_eq!(response.gamma, dec!(0.00012000));
        assert_eq!(response.vega, dec!(45.75000000));
        assert_eq!(response.high_price_limit, dec!(7800.00000000));
        assert_eq!(response.low_price_limit, dec!(6500.00000000));
        assert_eq!(response.risk_free_interest, dec!(0.05000000));
    }

    #[test]
    fn test_mark_price_response_deserialization_high_precision() {
        let json = r#"{
            "symbol": "ETH-240329-3000-P",
            "markPrice": "3015.12345678",
            "bidIV": "0.65432109",
            "askIV": "0.70987654",
            "markIV": "0.67543210",
            "delta": "-0.45123456",
            "theta": "-12.87654321",
            "gamma": "0.00098765",
            "vega": "32.11111111",
            "highPriceLimit": "3300.98765432",
            "lowPriceLimit": "2700.01234567",
            "riskFreeInterest": "0.04567890"
        }"#;

        let response: MarkPriceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETH-240329-3000-P");
        assert_eq!(response.mark_price.to_string(), "3015.12345678");
        assert_eq!(response.bid_iv.to_string(), "0.65432109");
        assert_eq!(response.ask_iv.to_string(), "0.70987654");
        assert_eq!(response.mark_iv.to_string(), "0.67543210");
        assert_eq!(response.delta.to_string(), "-0.45123456");
        assert_eq!(response.theta.to_string(), "-12.87654321");
        assert_eq!(response.gamma.to_string(), "0.00098765");
        assert_eq!(response.vega.to_string(), "32.11111111");
        assert_eq!(response.high_price_limit.to_string(), "3300.98765432");
        assert_eq!(response.low_price_limit.to_string(), "2700.01234567");
        assert_eq!(response.risk_free_interest.to_string(), "0.04567890");
    }

    #[test]
    fn test_mark_price_response_deserialization_zero_values() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "markPrice": "0.00000000",
            "bidIV": "0.00000000",
            "askIV": "0.00000000",
            "markIV": "0.00000000",
            "delta": "0.00000000",
            "theta": "0.00000000",
            "gamma": "0.00000000",
            "vega": "0.00000000",
            "highPriceLimit": "0.00000000",
            "lowPriceLimit": "0.00000000",
            "riskFreeInterest": "0.00000000"
        }"#;

        let response: MarkPriceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-70000-C");
        assert_eq!(response.mark_price, dec!(0.00000000));
        assert_eq!(response.bid_iv, dec!(0.00000000));
        assert_eq!(response.ask_iv, dec!(0.00000000));
        assert_eq!(response.mark_iv, dec!(0.00000000));
        assert_eq!(response.delta, dec!(0.00000000));
        assert_eq!(response.theta, dec!(0.00000000));
        assert_eq!(response.gamma, dec!(0.00000000));
        assert_eq!(response.vega, dec!(0.00000000));
        assert_eq!(response.high_price_limit, dec!(0.00000000));
        assert_eq!(response.low_price_limit, dec!(0.00000000));
        assert_eq!(response.risk_free_interest, dec!(0.00000000));
    }

    #[test]
    fn test_mark_price_response_deserialization_negative_values() {
        let json = r#"{
            "symbol": "BTC-240329-70000-P",
            "markPrice": "500.12345678",
            "bidIV": "0.80000000",
            "askIV": "0.85000000",
            "markIV": "0.82500000",
            "delta": "-0.75000000",
            "theta": "-50.25000000",
            "gamma": "0.00008000",
            "vega": "30.50000000",
            "highPriceLimit": "1000.00000000",
            "lowPriceLimit": "100.00000000",
            "riskFreeInterest": "-0.01000000"
        }"#;

        let response: MarkPriceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-70000-P");
        assert_eq!(response.mark_price, dec!(500.12345678));
        assert_eq!(response.bid_iv, dec!(0.80000000));
        assert_eq!(response.ask_iv, dec!(0.85000000));
        assert_eq!(response.mark_iv, dec!(0.82500000));
        assert_eq!(response.delta, dec!(-0.75000000));
        assert_eq!(response.theta, dec!(-50.25000000));
        assert_eq!(response.gamma, dec!(0.00008000));
        assert_eq!(response.vega, dec!(30.50000000));
        assert_eq!(response.high_price_limit, dec!(1000.00000000));
        assert_eq!(response.low_price_limit, dec!(100.00000000));
        assert_eq!(response.risk_free_interest, dec!(-0.01000000));
    }

    #[test]
    fn test_mark_price_response_deserialization_very_small_values() {
        let json = r#"{
            "symbol": "ETH-240329-3000-C",
            "markPrice": "0.00000001",
            "bidIV": "0.00000001",
            "askIV": "0.00000001",
            "markIV": "0.00000001",
            "delta": "0.00000001",
            "theta": "-0.00000001",
            "gamma": "0.00000001",
            "vega": "0.00000001",
            "highPriceLimit": "0.00000001",
            "lowPriceLimit": "0.00000001",
            "riskFreeInterest": "0.00000001"
        }"#;

        let response: MarkPriceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETH-240329-3000-C");
        assert_eq!(response.mark_price, dec!(0.00000001));
        assert_eq!(response.bid_iv, dec!(0.00000001));
        assert_eq!(response.ask_iv, dec!(0.00000001));
        assert_eq!(response.mark_iv, dec!(0.00000001));
        assert_eq!(response.delta, dec!(0.00000001));
        assert_eq!(response.theta, dec!(-0.00000001));
        assert_eq!(response.gamma, dec!(0.00000001));
        assert_eq!(response.vega, dec!(0.00000001));
        assert_eq!(response.high_price_limit, dec!(0.00000001));
        assert_eq!(response.low_price_limit, dec!(0.00000001));
        assert_eq!(response.risk_free_interest, dec!(0.00000001));
    }

    #[test]
    fn test_mark_price_response_deserialization_large_values() {
        let json = r#"{
            "symbol": "BTC-240329-10000-C",
            "markPrice": "99999999.99999999",
            "bidIV": "9.99999999",
            "askIV": "9.99999999",
            "markIV": "9.99999999",
            "delta": "0.99999999",
            "theta": "-999.99999999",
            "gamma": "0.99999999",
            "vega": "9999.99999999",
            "highPriceLimit": "999999999.99999999",
            "lowPriceLimit": "0.00000001",
            "riskFreeInterest": "0.99999999"
        }"#;

        let response: MarkPriceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-240329-10000-C");
        assert_eq!(response.mark_price, dec!(99999999.99999999));
        assert_eq!(response.bid_iv, dec!(9.99999999));
        assert_eq!(response.ask_iv, dec!(9.99999999));
        assert_eq!(response.mark_iv, dec!(9.99999999));
        assert_eq!(response.delta, dec!(0.99999999));
        assert_eq!(response.theta, dec!(-999.99999999));
        assert_eq!(response.gamma, dec!(0.99999999));
        assert_eq!(response.vega, dec!(9999.99999999));
        assert_eq!(response.high_price_limit, dec!(999999999.99999999));
        assert_eq!(response.low_price_limit, dec!(0.00000001));
        assert_eq!(response.risk_free_interest, dec!(0.99999999));
    }

    #[test]
    fn test_mark_price_response_clone() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "markPrice": "7150.50000000",
            "bidIV": "0.65000000",
            "askIV": "0.70000000",
            "markIV": "0.67500000",
            "delta": "0.55000000",
            "theta": "-25.50000000",
            "gamma": "0.00012000",
            "vega": "45.75000000",
            "highPriceLimit": "7800.00000000",
            "lowPriceLimit": "6500.00000000",
            "riskFreeInterest": "0.05000000"
        }"#;

        let response: MarkPriceResponse = serde_json::from_str(json).unwrap();
        let cloned = response.clone();
        
        assert_eq!(response.symbol, cloned.symbol);
        assert_eq!(response.mark_price, cloned.mark_price);
        assert_eq!(response.bid_iv, cloned.bid_iv);
        assert_eq!(response.ask_iv, cloned.ask_iv);
        assert_eq!(response.mark_iv, cloned.mark_iv);
        assert_eq!(response.delta, cloned.delta);
        assert_eq!(response.theta, cloned.theta);
        assert_eq!(response.gamma, cloned.gamma);
        assert_eq!(response.vega, cloned.vega);
        assert_eq!(response.high_price_limit, cloned.high_price_limit);
        assert_eq!(response.low_price_limit, cloned.low_price_limit);
        assert_eq!(response.risk_free_interest, cloned.risk_free_interest);
    }

    #[test]
    fn test_mark_price_response_debug() {
        let json = r#"{
            "symbol": "BTC-240329-70000-C",
            "markPrice": "7150.50000000",
            "bidIV": "0.65000000",
            "askIV": "0.70000000",
            "markIV": "0.67500000",
            "delta": "0.55000000",
            "theta": "-25.50000000",
            "gamma": "0.00012000",
            "vega": "45.75000000",
            "highPriceLimit": "7800.00000000",
            "lowPriceLimit": "6500.00000000",
            "riskFreeInterest": "0.05000000"
        }"#;

        let response: MarkPriceResponse = serde_json::from_str(json).unwrap();
        let debug_output = format!("{:?}", response);
        
        assert!(debug_output.contains("MarkPriceResponse"));
        assert!(debug_output.contains("BTC-240329-70000-C"));
        assert!(debug_output.contains("7150.50000000"));
    }

    #[test]
    fn test_mark_price_response_deserialization_different_option_types() {
        let test_cases = vec![
            ("BTC-240329-70000-C", "Call option"),
            ("BTC-240329-70000-P", "Put option"),
            ("ETH-240329-3000-C", "ETH Call option"),
            ("ETH-240329-3000-P", "ETH Put option"),
            ("BNB-240329-500-C", "BNB Call option"),
            ("BNB-240329-500-P", "BNB Put option"),
        ];

        for (symbol, _description) in test_cases {
            let json = format!(r#"{{
                "symbol": "{}",
                "markPrice": "1000.00000000",
                "bidIV": "0.60000000",
                "askIV": "0.65000000",
                "markIV": "0.62500000",
                "delta": "0.50000000",
                "theta": "-20.00000000",
                "gamma": "0.00010000",
                "vega": "40.00000000",
                "highPriceLimit": "1500.00000000",
                "lowPriceLimit": "500.00000000",
                "riskFreeInterest": "0.04000000"
            }}"#, symbol);

            let response: MarkPriceResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.symbol, symbol);
            assert_eq!(response.mark_price, dec!(1000.00000000));
            assert_eq!(response.bid_iv, dec!(0.60000000));
            assert_eq!(response.ask_iv, dec!(0.65000000));
            assert_eq!(response.mark_iv, dec!(0.62500000));
            assert_eq!(response.delta, dec!(0.50000000));
            assert_eq!(response.theta, dec!(-20.00000000));
            assert_eq!(response.gamma, dec!(0.00010000));
            assert_eq!(response.vega, dec!(40.00000000));
            assert_eq!(response.high_price_limit, dec!(1500.00000000));
            assert_eq!(response.low_price_limit, dec!(500.00000000));
            assert_eq!(response.risk_free_interest, dec!(0.04000000));
        }
    }

    #[test]
    fn test_mark_price_response_deserialization_edge_case_symbols() {
        let edge_symbols = vec![
            "BTC-991231-999999-C",    // Far expiry, high strike
            "ETH-000101-1-P",         // Low strike
            "BNB-240229-100000-C",    // Leap year
        ];

        for symbol in edge_symbols {
            let json = format!(r#"{{
                "symbol": "{}",
                "markPrice": "100.00000000",
                "bidIV": "0.50000000",
                "askIV": "0.55000000",
                "markIV": "0.52500000",
                "delta": "0.25000000",
                "theta": "-10.00000000",
                "gamma": "0.00005000",
                "vega": "20.00000000",
                "highPriceLimit": "200.00000000",
                "lowPriceLimit": "50.00000000",
                "riskFreeInterest": "0.03000000"
            }}"#, symbol);

            let response: MarkPriceResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.symbol, symbol);
            assert_eq!(response.mark_price, dec!(100.00000000));
        }
    }
}
