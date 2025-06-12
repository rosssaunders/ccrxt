#[cfg(test)]
mod integration_tests {
    use crate::cryptocom::{ApiError, ErrorResponse, Errors, RestClient, PublicRestClient};

    #[test]
    fn test_crypto_com_module_exports() {
        // Test that we can use the exported types
        let error_response = ErrorResponse {
            code: 201,
            message: "No position".to_string(),
        };
        
        let api_error: ApiError = error_response.into();
        let _errors = Errors::ApiError(api_error);
        
        // Test that we can construct various error types
        let _http_error = Errors::Error("Test error".to_string());
        let _invalid_key = Errors::InvalidApiKey();
        
        // Test that we can access both the private and public RestClient types
        // We can't easily construct them in test without proper dependencies,
        // but we can verify the types are accessible
        let _private_client_type_check = std::marker::PhantomData::<RestClient>;
        let _public_client_type_check = std::marker::PhantomData::<PublicRestClient>;
    }

    #[test]
    fn test_crypto_com_error_conversion_examples() {
        // Test some key error codes that would be commonly encountered
        let test_cases = vec![
            (0, "Success"),
            (204, "Duplicate client order id"),
            (308, "Invalid price"),
            (40101, "Not authenticated, or key/signature incorrect"),
            (42901, "Requests have exceeded rate limits"),
        ];

        for (code, message) in test_cases {
            let error_response = ErrorResponse {
                code,
                message: message.to_string(),
            };
            
            let api_error: ApiError = error_response.into();
            let error_string = format!("{}", api_error);
            
            // Verify that error messages are meaningful
            assert!(error_string.len() > 0);
            assert!(!error_string.contains("UnmappedApiError"), 
                   "Error code {} should be properly mapped", code);
        }
    }
}