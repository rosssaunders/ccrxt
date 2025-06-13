use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::cryptocom::RestResult;
use super::client::RestClient;

/// Request parameters for create subaccount transfer
#[derive(Debug, Clone, Serialize)]
pub struct CreateSubaccountTransferRequest {
    /// Account UUID to be debited
    pub from: String,
    /// Account UUID to be credit
    pub to: String,
    /// Currency symbol
    pub currency: String,
    /// Amount to transfer - must a be positive number
    pub amount: String,
}

/// Response for create subaccount transfer endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubaccountTransferResponse {
    /// 0 for successful transfer (NO_ERROR) else the error code
    pub code: i32,
}

impl RestClient {
    /// Transfer funds between master and sub-accounts
    ///
    /// Transfers funds between master account and sub-accounts.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html#private-create-subaccount-transfer>
    ///
    /// Rate limit: No rate limit
    ///
    /// # Arguments
    /// * `from` - Account UUID to be debited
    /// * `to` - Account UUID to be credited
    /// * `currency` - Currency symbol
    /// * `amount` - Amount to transfer - must be a positive number
    ///
    /// # Returns
    /// Transfer result with status code
    pub async fn create_subaccount_transfer(
        &self,
        from: &str,
        to: &str,
        currency: &str,
        amount: &str
    ) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        
        let params = json!({
            "from": from,
            "to": to,
            "currency": currency,
            "amount": amount
        });
        
        let signature = self.sign_request("private/create-subaccount-transfer", id, &params, nonce)?;
        
        let request_body = json!({
            "id": id,
            "method": "private/create-subaccount-transfer",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self.client
            .post(&format!("{}/v1/private/create-subaccount-transfer", self.base_url))
            .json(&request_body)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    struct PlainTextSecret {
        secret: String,
    }
    
    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }
    
    impl PlainTextSecret {
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
        assert_eq!(json_value["from"], "12345678-0000-0000-0000-000000000001");
        assert_eq!(json_value["to"], "12345678-0000-0000-0000-000000000002");
        assert_eq!(json_value["currency"], "CRO");
        assert_eq!(json_value["amount"], "500.00");
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
            "code": 0
        });

        let response: CreateSubaccountTransferResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, 0);
    }

    #[test]
    fn test_create_subaccount_transfer_response_error() {
        let response_json = json!({
            "code": 10002
        });

        let response: CreateSubaccountTransferResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, 10002);
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
        assert_eq!(json_value["currency"], "BTC");
        assert_eq!(json_value["amount"], "0.00123456");

        let usdt_request = CreateSubaccountTransferRequest {
            from: "sub-account".to_string(),
            to: "master-account".to_string(),
            currency: "USDT".to_string(),
            amount: "250.5".to_string(),
        };

        let json_value = serde_json::to_value(usdt_request).unwrap();
        assert_eq!(json_value["currency"], "USDT");
        assert_eq!(json_value["amount"], "250.5");
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
        assert_eq!(json_value["from"], "");
    }
}