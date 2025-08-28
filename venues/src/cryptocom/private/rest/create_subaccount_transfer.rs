use serde::{Deserialize, Serialize};

use crate::cryptocom::{
    ApiResult, PrivateRestClient as RestClient, RestResult, enums::ResponseCode,
};

/// Endpoint path for the create-subaccount-transfer API
const CREATE_SUBACCOUNT_TRANSFER_ENDPOINT: &str = "exchange/v1/private/create-subaccount-transfer";

/// Request parameters for creating a subaccount transfer.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSubaccountTransferRequest {
    /// Account UUID to be debited.
    pub from: String,

    /// Account UUID to be credited.
    pub to: String,

    /// Currency symbol.
    pub currency: String,

    /// Amount to transfer - must be a positive number.
    pub amount: String,
}

/// Result data for creating a subaccount transfer.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSubaccountTransferResult {
    /// Status code: 0 for successful transfer (NO_ERROR), else the error code.
    pub code: ResponseCode,
}

/// Response wrapper for endpoint
pub type CreateSubaccountTransferResponse = ApiResult<CreateSubaccountTransferResult>;

impl RestClient {
    /// Transfer funds between master and sub-accounts.
    ///
    /// Transfers funds between master account and sub-accounts.
    ///
    /// [docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#private-create-subaccount-transfer)
    ///
    /// Rate limit: No rate limit
    ///
    /// # Arguments
    /// * `params` - Parameters for the subaccount transfer.
    ///
    /// # Returns
    /// Transfer result with status code.
    pub async fn create_subaccount_transfer(
        &self,
        params: CreateSubaccountTransferRequest,
    ) -> RestResult<CreateSubaccountTransferResponse> {
        let params = serde_json::to_value(&params)
            .map_err(|e| crate::cryptocom::Errors::Error(format!("Serialization error: {e}")))?;

        self.send_signed_request(CREATE_SUBACCOUNT_TRANSFER_ENDPOINT, params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    use super::*;
    use crate::cryptocom::enums::ResponseCode;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        #[allow(dead_code)]
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_create_subaccount_transfer_request_structure() {
        let request = CreateSubaccountTransferRequest {
            from: "12345678-0000-0000-0000-000000000001".to_string(),
            to: "12345678-0000-0000-0000-000000000002".to_string(),
            currency: "CRO".to_string(),
            amount: "500.00".to_string(),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(
            json_value.get("from").unwrap(),
            "12345678-0000-0000-0000-000000000001"
        );
        assert_eq!(
            json_value.get("to").unwrap(),
            "12345678-0000-0000-0000-000000000002"
        );
        assert_eq!(json_value.get("currency").unwrap(), "CRO");
        assert_eq!(json_value.get("amount").unwrap(), "500.00");
    }

    #[test]
    fn test_create_subaccount_transfer_request_serialization() {
        let request = CreateSubaccountTransferRequest {
            from: "master-uuid".to_string(),
            to: "sub-uuid".to_string(),
            currency: "USD".to_string(),
            amount: "1000.0000".to_string(),
        };

        let json_value = serde_json::to_value(&request).unwrap();
        let expected = json!({
            "from": "master-uuid",
            "to": "sub-uuid",
            "currency": "USD",
            "amount": "1000.0000"
        });

        assert_eq!(json_value, expected);
    }

    #[test]
    fn test_create_subaccount_transfer_response_structure() {
        let response_json = json!({
            "code": 0,
            "id": 1,
            "result": {
                "code": 0
            }
        });

        let response: CreateSubaccountTransferResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.code, ResponseCode::NoError);
    }

    #[test]
    fn test_create_subaccount_transfer_response_error() {
        let response_json = json!({
            "code": 0,
            "id": 1,
            "result": {
                "code": 10002
            }
        });

        let response: CreateSubaccountTransferResponse =
            serde_json::from_value(response_json).unwrap();
        if let ResponseCode::Error(code) = response.result.code {
            assert_eq!(code, 10002);
        } else {
            assert!(false, "Expected Error variant");
        }
    }

    #[test]
    fn test_create_subaccount_transfer_request_different_currencies() {
        let btc_request = CreateSubaccountTransferRequest {
            from: "master-account".to_string(),
            to: "sub-account".to_string(),
            currency: "BTC".to_string(),
            amount: "0.00123456".to_string(),
        };

        let json_value = serde_json::to_value(btc_request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "BTC");
        assert_eq!(json_value.get("amount").unwrap(), "0.00123456");

        let usdt_request = CreateSubaccountTransferRequest {
            from: "sub-account".to_string(),
            to: "master-account".to_string(),
            currency: "USDT".to_string(),
            amount: "250.5".to_string(),
        };

        let json_value = serde_json::to_value(usdt_request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "USDT");
        assert_eq!(json_value.get("amount").unwrap(), "250.5");
    }

    #[test]
    fn test_create_subaccount_transfer_request_validation() {
        let request = CreateSubaccountTransferRequest {
            from: "".to_string(),
            to: "valid-uuid".to_string(),
            currency: "USD".to_string(),
            amount: "100.00".to_string(),
        };

        // Should serialize even with empty from field
        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("from").unwrap(), "");
    }
}
