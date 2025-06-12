/// Example demonstrating Crypto.com error code usage
/// 
/// This example shows how to use the Crypto.com error codes in real applications.
/// The error types can be used to handle API responses and provide meaningful
/// error messages to users or for logging purposes.

#[cfg(test)]
mod example {
    use crate::cryptocom::{ApiError, ErrorResponse, Errors, RestResult};

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
        let result = process_api_response(40101, "Not authenticated, or key/signature incorrect".to_string());
        match result {
            Err(Errors::ApiError(ApiError::Unauthorized)) => {
                // Handle authentication error
                println!("Authentication failed - check API credentials");
            }
            _ => panic!("Expected authentication error"),
        }

        // Example 3: Rate limit error
        let result = process_api_response(42901, "Requests have exceeded rate limits".to_string());
        match result {
            Err(Errors::ApiError(ApiError::TooManyRequests)) => {
                // Handle rate limit error
                println!("Rate limit exceeded - implement backoff strategy");
            }
            _ => panic!("Expected rate limit error"),
        }

        // Example 4: Invalid order error
        let result = process_api_response(213, "Invalid order quantity".to_string());
        match result {
            Err(Errors::ApiError(ApiError::InvalidOrderQuantity)) => {
                // Handle validation error
                println!("Order validation failed - check order parameters");
            }
            _ => panic!("Expected order validation error"),
        }

        // Example 5: Unknown error code (falls back to UnmappedApiError)
        let result = process_api_response(99999, "Unknown error".to_string());
        match result {
            Err(Errors::ApiError(ApiError::UnmappedApiError { code, message })) => {
                println!("Unknown error code {}: {}", code, message);
                assert_eq!(code, 99999);
                assert_eq!(message, "Unknown error");
            }
            _ => panic!("Expected unmapped error"),
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
}