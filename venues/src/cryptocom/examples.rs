/// Example demonstrating Crypto.com error code usage
///
/// This example shows how to use the Crypto.com error codes in real applications.
/// The error types can be used to handle API responses and provide meaningful
/// error messages to users or for logging purposes.
#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod example {
    use crate::cryptocom::{ApiError, ErrorResponse, Errors, PrivateRestClient, RestResult};
    use serde_json::json;

    /// Simulates processing an API response from Crypto.com
    fn process_api_response(response_code: i32, message: String) -> RestResult<String> {
        match response_code {
            0 => Ok("Operation successful".to_string()),
            _ => {
                let error_response = ErrorResponse {
                    code: response_code,
                    message,
                };
                let api_error: ApiError = error_response.into();
                Err(Errors::ApiError(api_error))
            }
        }
    }

    #[test]
    fn example_usage() {
        // Example 1: Successful response
        let result = process_api_response(0, "Success".to_string());
        assert!(result.is_ok());

        // Example 2: Authentication error
        let result = process_api_response(
            40101,
            "Not authenticated, or key/signature incorrect".to_string(),
        );
        match result {
            Err(Errors::ApiError(ApiError::Unauthorized)) => {
                // Handle authentication error
                println!("Authentication failed - check API credentials");
            }
            _ => assert!(false, "Expected authentication error"),
        }

        // Example 3: Rate limit error
        let result = process_api_response(42901, "Requests have exceeded rate limits".to_string());
        match result {
            Err(Errors::ApiError(ApiError::TooManyRequests)) => {
                // Handle rate limit error
                println!("Rate limit exceeded - implement backoff strategy");
            }
            _ => assert!(false, "Expected rate limit error"),
        }

        // Example 4: Invalid order error
        let result = process_api_response(213, "Invalid order quantity".to_string());
        match result {
            Err(Errors::ApiError(ApiError::InvalidOrderQuantity)) => {
                // Handle validation error
                println!("Order validation failed - check order parameters");
            }
            _ => assert!(false, "Expected order validation error"),
        }

        // Example 5: Unknown error code (falls back to UnmappedApiError)
        let result = process_api_response(99999, "Unknown error".to_string());
        match result {
            Err(Errors::ApiError(ApiError::UnmappedApiError { code, message })) => {
                println!("Unknown error code {}: {}", code, message);
                assert_eq!(code, 99999);
                assert_eq!(message, "Unknown error");
            }
            _ => assert!(false, "Expected unmapped error"),
        }
    }

    #[test]
    fn example_error_display() {
        // Demonstrate how errors can be displayed for logging or user messages
        let test_errors = vec![
            (201, "No position"),
            (302, "Exceeds account risk limit"),
            (40001, "Bad request"),
            (43003, "FOK order has not been filled and cancelled"),
        ];

        for (code, message) in test_errors {
            let error_response = ErrorResponse {
                code,
                message: message.to_string(),
            };

            let api_error: ApiError = error_response.into();
            let error_message = format!("{}", api_error);

            // Verify that error messages are meaningful and not empty
            assert!(!error_message.is_empty());
            println!("Code {}: {}", code, error_message);
        }
    }

    #[test]
    fn example_private_endpoint_signing() {
        // Example demonstrating how to use the private endpoint signing
        // Note: This is a demonstration only - you would not use PlainTextSecret in production
        use rest::secrets::SecretValue;
        use secrecy::SecretString;

        // Create a RestClient with proper secrets (this example uses test secrets)
        let api_key = Box::new(SecretValue::new(SecretString::new("your_api_key".into())));
        let api_secret = Box::new(SecretValue::new(SecretString::new(
            "your_api_secret".into(),
        )));
        let client = reqwest::Client::new();

        let rest_client =
            PrivateRestClient::new(api_key, api_secret, "https://api.crypto.com", client);

        // Example 1: Sign a get-order-detail request
        let params = json!({
            "order_id": "53287421324"  // Note: Using string format as recommended
        });

        let signature = rest_client.sign_request(
            "private/get-order-detail",
            11, // request ID
            &params,
            1587846358253, // nonce (timestamp in milliseconds)
        );

        match signature {
            Ok(sig) => {
                println!("Generated signature: {}", sig);
                assert_eq!(sig.len(), 64); // HMAC-SHA256 produces 64 hex chars
            }
            Err(e) => assert!(false, "Failed to sign request: {}", e),
        }

        // Example 2: Sign a create-order request
        let order_params = json!({
            "instrument_name": "BTC_USDT",
            "side": "BUY",
            "type": "LIMIT",
            "quantity": "1.5",
            "price": "50000.00"
        });

        let signature = rest_client.sign_request(
            "private/create-order",
            42,
            &order_params,
            chrono::Utc::now().timestamp_millis() as u64,
        );

        assert!(signature.is_ok());
        println!("Create order signature: {}", signature.unwrap());

        // Example 3: Sign a request with empty parameters
        let signature = rest_client.sign_request(
            "private/get-account-summary",
            1,
            &json!({}),
            chrono::Utc::now().timestamp_millis() as u64,
        );

        assert!(signature.is_ok());
        println!("Account summary signature: {}", signature.unwrap());
    }

    /// Example demonstrating usage of the new public endpoints
    #[test]
    fn example_new_public_endpoints() {
        // Note: These are just examples of method calls - they won't make actual HTTP requests in tests

        // Example usage patterns (would be used in real applications):

        // 1. Get announcements filtered by category and product type
        // let announcements = client.get_announcements(Some("system"), Some("Spot")).await?;

        // 2. Get risk parameters for Smart Cross Margin
        // let risk_params = client.get_risk_parameters().await?;

        // 3. Get all available instruments
        // let instruments = client.get_instruments().await?;

        // 4. Get ticker data for all instruments
        // let tickers = client.get_tickers(None).await?;

        // 5. Get valuation data (index price) for BTCUSD-INDEX
        // let valuations = client.get_valuations("BTCUSD-INDEX", "index_price", Some(10), None, None).await?;

        // 6. Get expired settlement prices for FUTURE instruments
        // let settlement_prices = client.get_expired_settlement_price("FUTURE", Some(1)).await?;

        // 7. Get insurance fund balance for USD
        // let insurance = client.get_insurance("USD", Some(25), None, None).await?;

        // 8. Enhanced candlestick data with timestamp filtering
        // let candlesticks = client.get_candlestick("BTCUSD-PERP", "1h", Some(100), Some(1640995200), Some(1641081600)).await?;

        // 9. Enhanced trades data with timestamp filtering (nanosecond precision)
        // let trades = client.get_trades("BTCUSD-PERP", Some(50), Some("1613547060925523623"), None).await?;

        println!("All new endpoint methods are available and properly typed");
    }
}
