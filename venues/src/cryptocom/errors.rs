use thiserror::Error;
use serde::Deserialize;
use std::fmt;

/// Represents all possible errors that can occur when interacting with the Crypto.com API
#[derive(Debug)]
pub enum Errors {
    /// Invalid API key or signature
    InvalidApiKey(),
    
    /// Http error occurred while making a request
    /// This variant is used to represent errors that are not specific to the Crypto.com API,
    /// such as network issues or HTTP errors.
    /// It can be used to wrap any error that occurs during the request process.
    /// This variant is not used for errors returned by the Crypto.com API itself.
    HttpError(reqwest::Error),

    /// An error returned by the Crypto.com API
    ApiError(ApiError),
    
    /// A general error with a descriptive message
    Error(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::InvalidApiKey() => write!(f, "Invalid API key or signature"),
            Errors::HttpError(err) => write!(f, "HTTP error: {}", err),
            Errors::ApiError(err) => write!(f, "API error: {}", err),
            Errors::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for Errors {}

impl From<reqwest::Error> for Errors {
    fn from(err: reqwest::Error) -> Self {
        Errors::HttpError(err)
    }
}

/// Represents an error response from the Crypto.com API.
/// 
/// This is public as it is used by API responses.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
}

/// Crypto.com API error codes as documented in their REST API specification
/// https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#response-and-reason-codes
#[derive(Error, Debug, Clone, Deserialize)]
pub enum ApiError {
    // Success
    #[error("Success")]
    Success,

    // 2xx - Position and Account Related Errors
    #[error("No position")]
    NoPosition,

    #[error("Account is suspended")]
    AccountIsSuspended,

    #[error("Accounts do not match")]
    AccountsDoNotMatch,

    #[error("Duplicate client order id")]
    DuplicateClientOrderId,

    #[error("Duplicate order id")]
    DuplicateOrderId,

    #[error("Instrument has expired")]
    InstrumentExpired,

    #[error("No mark price")]
    NoMarkPrice,

    #[error("Instrument is not tradable")]
    InstrumentNotTradable,

    #[error("Instrument is invalid")]
    InvalidInstrument,

    #[error("Account is invalid")]
    InvalidAccount,

    #[error("Currency is invalid")]
    InvalidCurrency,

    #[error("Invalid order id")]
    InvalidOrderId,

    #[error("Invalid order quantity")]
    InvalidOrderQuantity,

    #[error("Invalid settlement currency")]
    InvalidSettleCurrency,

    #[error("Invalid fee currency")]
    InvalidFeeCurrency,

    #[error("Invalid position quantity")]
    InvalidPositionQuantity,

    #[error("Invalid open quantity")]
    InvalidOpenQuantity,

    #[error("Invalid order_type")]
    InvalidOrderType,

    #[error("Invalid exec_inst")]
    InvalidExecInst,

    #[error("Invalid side")]
    InvalidSide,

    #[error("Invalid time_in_force")]
    InvalidTimeInForce,

    #[error("Stale mark price")]
    StaleMarkPrice,

    #[error("No client order id")]
    NoClientOrderId,

    #[error("Rejected by matching engine")]
    RejectedByMatchingEngine,

    #[error("Exceeds maximum entry leverage")]
    ExceedMaximumEntryLeverage,

    #[error("Invalid leverage")]
    InvalidLeverage,

    #[error("Invalid slippage")]
    InvalidSlippage,

    #[error("Invalid floor price")]
    InvalidFloorPrice,

    #[error("Invalid ref price")]
    InvalidRefPrice,

    #[error("Invalid ref price type")]
    InvalidTriggerType,

    // 3xx - Risk and Margin Related Errors  
    #[error("Account is in margin call")]
    AccountIsInMarginCall,

    #[error("Exceeds account risk limit")]
    ExceedsAccountRiskLimit,

    #[error("Exceeds position risk limit")]
    ExceedsPositionRiskLimit,

    #[error("Order will lead to immediate liquidation")]
    OrderWillLeadToImmediateLiquidation,

    #[error("Order will trigger margin call")]
    OrderWillTriggerMarginCall,

    #[error("Insufficient available balance")]
    InsufficientAvailableBalance,

    #[error("Invalid order status")]
    InvalidOrderStatus,

    #[error("Invalid price")]
    InvalidPrice,

    #[error("Market is not open")]
    MarketIsNotOpen,

    #[error("Order price beyond liquidation price")]
    OrderPriceBeyondLiquidationPrice,

    #[error("Position is in liquidation")]
    PositionIsInLiquidation,

    #[error("Order price is greater than the limit up price")]
    OrderPriceGreaterThanLimitUpPrice,

    #[error("Order price is less than the limit down price")]
    OrderPriceLessThanLimitDownPrice,

    #[error("Exceeds max order size")]
    ExceedsMaxOrderSize,

    #[error("Far away limit price")]
    FarAwayLimitPrice,

    #[error("No active order")]
    NoActiveOrder,

    #[error("Position does not exist")]
    PositionNoExist,

    #[error("Exceeds max allowed orders")]
    ExceedsMaxAllowedOrders,

    #[error("Exceeds max position size")]
    ExceedsMaxPositionSize,

    #[error("Exceeds initial margin")]
    ExceedsInitialMargin,

    #[error("Exceeds maximum available balance")]
    ExceedsMaxAvailableBalance,

    // 4xx - Account and User Related Errors
    #[error("Account does not exist")]
    AccountDoesNotExist,

    #[error("Account is not active")]
    AccountIsNotActive,

    #[error("Margin unit does not exist")]
    MarginUnitDoesNotExist,

    #[error("Margin unit is suspended")]
    MarginUnitIsSuspended,

    #[error("Invalid user")]
    InvalidUser,

    #[error("User is not active")]
    UserIsNotActive,

    #[error("User does not have derivative access")]
    UserNoDerivAccess,

    #[error("Account does not have derivative access")]
    AccountNoDerivAccess,

    #[error("Below Min. Order Size")]
    BelowMinOrderSize,

    // 5xx - Margin and Calculation Related Errors
    #[error("Exceeds maximum effective leverage")]
    ExceedMaximumEffectiveLeverage,

    #[error("Invalid collateral price")]
    InvalidCollateralPrice,

    #[error("Invalid margin calculation")]
    InvalidMarginCalc,

    #[error("Exceed allowed slippage")]
    ExceedAllowedSlippage,

    // Withdrawal Related
    #[error("If create-withdrawal call quantity > max_withdrawal_balance in user-balance api")]
    MaxAmountViolated,

    // HTTP Status Related Errors
    #[error("Bad request")]
    BadRequest,

    #[error("Method not found")]
    MethodNotFound,

    #[error("Invalid request")]
    InvalidRequest,

    #[error("Required argument is blank or missing")]
    MissingOrInvalidArgument,

    #[error("Invalid date")]
    InvalidDate,

    #[error("Duplicate request received")]
    DuplicateRequest,

    #[error("Not authenticated, or key/signature incorrect")]
    Unauthorized,

    #[error("Nonce value differs by more than 60 seconds")]
    InvalidNonce,

    #[error("IP address not whitelisted")]
    IpIllegal,

    #[error("Disallowed based on user tier")]
    UserTierInvalid,

    #[error("Session subscription limit has been exceeded")]
    ExceedMaxSubscriptions,

    #[error("Not found")]
    NotFound,

    #[error("Request has timed out")]
    RequestTimeout,

    #[error("Requests have exceeded rate limits")]
    TooManyRequests,

    #[error("FOK order has not been filled and cancelled")]
    FillOrKill,

    #[error("IOC order has not been filled and cancelled")]
    ImmediateOrCancel,

    #[error("Rejected POST_ONLY create-order request (normally happened when exec_inst contains POST_ONLY but time_in_force is NOT GOOD_TILL_CANCEL)")]
    PostOnlyRej,

    #[error("Canceled due to Self Trade Prevention")]
    SelfTradePrevention,

    #[error("If create-withdrawal call breaching credit line check")]
    DwCreditLineNotMaintained,

    #[error("Internal error")]
    ErrInternal,

    /// Unmapped API error - for error codes not explicitly handled
    #[error("API error (code: {code}): {message}")]
    UnmappedApiError { code: i32, message: String },
}

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        match err.code {
            0 => ApiError::Success,
            201 => ApiError::NoPosition,
            202 => ApiError::AccountIsSuspended,
            203 => ApiError::AccountsDoNotMatch,
            204 => ApiError::DuplicateClientOrderId,
            205 => ApiError::DuplicateOrderId,
            206 => ApiError::InstrumentExpired,
            207 => ApiError::NoMarkPrice,
            208 => ApiError::InstrumentNotTradable,
            209 => ApiError::InvalidInstrument,
            210 => ApiError::InvalidAccount,
            211 => ApiError::InvalidCurrency,
            212 => ApiError::InvalidOrderId,
            213 => ApiError::InvalidOrderQuantity,
            214 => ApiError::InvalidSettleCurrency,
            215 => ApiError::InvalidFeeCurrency,
            216 => ApiError::InvalidPositionQuantity,
            217 => ApiError::InvalidOpenQuantity,
            218 => ApiError::InvalidOrderType,
            219 => ApiError::InvalidExecInst,
            220 => ApiError::InvalidSide,
            221 => ApiError::InvalidTimeInForce,
            222 => ApiError::StaleMarkPrice,
            223 => ApiError::NoClientOrderId,
            224 => ApiError::RejectedByMatchingEngine,
            225 => ApiError::ExceedMaximumEntryLeverage,
            226 => ApiError::InvalidLeverage,
            227 => ApiError::InvalidSlippage,
            228 => ApiError::InvalidFloorPrice,
            229 => ApiError::InvalidRefPrice,
            230 => ApiError::InvalidTriggerType,
            301 => ApiError::AccountIsInMarginCall,
            302 => ApiError::ExceedsAccountRiskLimit,
            303 => ApiError::ExceedsPositionRiskLimit,
            304 => ApiError::OrderWillLeadToImmediateLiquidation,
            305 => ApiError::OrderWillTriggerMarginCall,
            306 => ApiError::InsufficientAvailableBalance,
            307 => ApiError::InvalidOrderStatus,
            308 => ApiError::InvalidPrice,
            309 => ApiError::MarketIsNotOpen,
            310 => ApiError::OrderPriceBeyondLiquidationPrice,
            311 => ApiError::PositionIsInLiquidation,
            312 => ApiError::OrderPriceGreaterThanLimitUpPrice,
            313 => ApiError::OrderPriceLessThanLimitDownPrice,
            314 => ApiError::ExceedsMaxOrderSize,
            315 => ApiError::FarAwayLimitPrice,
            316 => ApiError::NoActiveOrder,
            317 => ApiError::PositionNoExist,
            318 => ApiError::ExceedsMaxAllowedOrders,
            319 => ApiError::ExceedsMaxPositionSize,
            320 => ApiError::ExceedsInitialMargin,
            321 => ApiError::ExceedsMaxAvailableBalance,
            401 => ApiError::AccountDoesNotExist,
            406 => ApiError::AccountIsNotActive,
            407 => ApiError::MarginUnitDoesNotExist,
            408 => ApiError::MarginUnitIsSuspended,
            409 => ApiError::InvalidUser,
            410 => ApiError::UserIsNotActive,
            411 => ApiError::UserNoDerivAccess,
            412 => ApiError::AccountNoDerivAccess,
            415 => ApiError::BelowMinOrderSize,
            501 => ApiError::ExceedMaximumEffectiveLeverage,
            604 => ApiError::InvalidCollateralPrice,
            605 => ApiError::InvalidMarginCalc,
            606 => ApiError::ExceedAllowedSlippage,
            30024 => ApiError::MaxAmountViolated,
            40001 => ApiError::BadRequest,
            40002 => ApiError::MethodNotFound,
            40003 => ApiError::InvalidRequest,
            40004 => ApiError::MissingOrInvalidArgument,
            40005 => ApiError::InvalidDate,
            40006 => ApiError::DuplicateRequest,
            40101 => ApiError::Unauthorized,
            40102 => ApiError::InvalidNonce,
            40103 => ApiError::IpIllegal,
            40104 => ApiError::UserTierInvalid,
            40107 => ApiError::ExceedMaxSubscriptions,
            40401 => ApiError::NotFound,
            40801 => ApiError::RequestTimeout,
            42901 => ApiError::TooManyRequests,
            43003 => ApiError::FillOrKill,
            43004 => ApiError::ImmediateOrCancel,
            43005 => ApiError::PostOnlyRej,
            43012 => ApiError::SelfTradePrevention,
            50001 => match err.message.as_str() {
                "If create-withdrawal call breaching credit line check" => ApiError::DwCreditLineNotMaintained,
                "Internal error" => ApiError::ErrInternal,
                _ => ApiError::UnmappedApiError { code: err.code, message: err.message },
            },
            _ => ApiError::UnmappedApiError { code: err.code, message: err.message },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_error_code() {
        let error_response = ErrorResponse {
            code: 0,
            message: "Success".to_string(),
        };
        
        let api_error: ApiError = error_response.into();
        match api_error {
            ApiError::Success => {},
            _ => panic!("Expected Success variant"),
        }
    }

    #[test]
    fn test_position_related_errors() {
        let test_cases = vec![
            (201, "No position", "NoPosition"),
            (202, "Account is suspended", "AccountIsSuspended"),
            (203, "Accounts do not match", "AccountsDoNotMatch"),
            (204, "Duplicate client order id", "DuplicateClientOrderId"),
        ];

        for (code, message, _expected_variant) in test_cases {
            let error_response = ErrorResponse {
                code,
                message: message.to_string(),
            };
            
            let api_error: ApiError = error_response.into();
            
            // Test that the error message contains expected content
            let error_string = format!("{}", api_error);
            assert!(error_string.len() > 0, "Error message should not be empty for code {}", code);
            
            // Test specific known cases
            match code {
                201 => {
                    if let ApiError::NoPosition = api_error {
                        // Expected
                    } else {
                        panic!("Expected NoPosition for code 201, got {:?}", api_error);
                    }
                },
                202 => {
                    if let ApiError::AccountIsSuspended = api_error {
                        // Expected  
                    } else {
                        panic!("Expected AccountIsSuspended for code 202, got {:?}", api_error);
                    }
                },
                _ => {} // Other cases are tested by building without panics
            }
        }
    }

    #[test]
    fn test_margin_related_errors() {
        let test_cases = vec![
            (301, "Account is in margin call"),
            (302, "Exceeds account risk limit"),
            (303, "Exceeds position risk limit"),
            (304, "Order will lead to immediate liquidation"),
        ];

        for (code, message) in test_cases {
            let error_response = ErrorResponse {
                code,
                message: message.to_string(),
            };
            
            let api_error: ApiError = error_response.into();
            let error_string = format!("{}", api_error);
            assert!(error_string.len() > 0, "Error message should not be empty for code {}", code);
        }
    }

    #[test]
    fn test_http_status_errors() {
        let test_cases = vec![
            (40001, "Bad request"),
            (40101, "Not authenticated, or key/signature incorrect"),
            (40401, "Not found"),
            (42901, "Requests have exceeded rate limits"),
        ];

        for (code, message) in test_cases {
            let error_response = ErrorResponse {
                code,
                message: message.to_string(),
            };
            
            let api_error: ApiError = error_response.into();
            let error_string = format!("{}", api_error);
            assert!(error_string.len() > 0, "Error message should not be empty for code {}", code);
        }
    }

    #[test]
    fn test_special_case_50001() {
        // Test the special case where code 50001 has two different messages
        let error_response_1 = ErrorResponse {
            code: 50001,
            message: "If create-withdrawal call breaching credit line check".to_string(),
        };
        
        let api_error_1: ApiError = error_response_1.into();
        if let ApiError::DwCreditLineNotMaintained = api_error_1 {
            // Expected
        } else {
            panic!("Expected DwCreditLineNotMaintained for specific message, got {:?}", api_error_1);
        }

        let error_response_2 = ErrorResponse {
            code: 50001,
            message: "Internal error".to_string(),
        };
        
        let api_error_2: ApiError = error_response_2.into();
        if let ApiError::ErrInternal = api_error_2 {
            // Expected
        } else {
            panic!("Expected ErrInternal for specific message, got {:?}", api_error_2);
        }

        // Test unknown message for 50001
        let error_response_3 = ErrorResponse {
            code: 50001,
            message: "Some unknown error".to_string(),
        };
        
        let api_error_3: ApiError = error_response_3.into();
        if let ApiError::UnmappedApiError { code, message } = api_error_3 {
            assert_eq!(code, 50001);
            assert_eq!(message, "Some unknown error");
        } else {
            panic!("Expected UnmappedApiError for unknown message, got {:?}", api_error_3);
        }
    }

    #[test]
    fn test_unmapped_error_code() {
        let error_response = ErrorResponse {
            code: 99999,
            message: "Unknown error".to_string(),
        };
        
        let api_error: ApiError = error_response.into();
        if let ApiError::UnmappedApiError { code, message } = api_error {
            assert_eq!(code, 99999);
            assert_eq!(message, "Unknown error");
        } else {
            panic!("Expected UnmappedApiError for unknown code, got {:?}", api_error);
        }
    }

    #[test]
    fn test_errors_enum_display() {
        let general_error = Errors::Error("Test error".to_string());
        let error_string = format!("{}", general_error);
        assert_eq!(error_string, "Error: Test error");

        let invalid_key_error = Errors::InvalidApiKey();
        let error_string = format!("{}", invalid_key_error);
        assert_eq!(error_string, "Invalid API key or signature");
    }

    #[test]
    fn test_comprehensive_error_code_coverage() {
        // Test a selection of error codes from each category to ensure they're all mapped
        let test_codes = vec![
            // Success
            0,
            // 2xx series
            201, 205, 213, 220, 225, 230,
            // 3xx series  
            301, 308, 315, 321,
            // 4xx series
            401, 409, 415,
            // 5xx series
            501, 604, 606,
            // Special codes
            30024, 40001, 40101, 42901, 43003, 43012,
        ];

        for code in test_codes {
            let error_response = ErrorResponse {
                code,
                message: format!("Test message for code {}", code),
            };
            
            let api_error: ApiError = error_response.into();
            
            // Ensure we don't get UnmappedApiError for known codes
            if let ApiError::UnmappedApiError { .. } = api_error {
                panic!("Code {} should be mapped but got UnmappedApiError", code);
            }
            
            // Ensure error message is not empty
            let error_string = format!("{}", api_error);
            assert!(error_string.len() > 0, "Error message should not be empty for code {}", code);
        }
    }
}