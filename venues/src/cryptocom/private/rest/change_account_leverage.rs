use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::Serialize;
use serde_json::{Value, json};

/// Request parameters for changing account leverage
#[derive(Debug, Clone, Serialize)]
pub struct ChangeAccountLeverageRequest {
    /// Account ID to change the leverage. Must be currently the logged user's account
    pub account_id: String,
    /// Maximum leverage to be used for the account. Valid values are between 1-100 (inclusive)
    pub leverage: u8,
}

impl RestClient {
    /// Changes the maximum leverage used by the account
    ///
    /// Please note, each instrument has its own maximum leverage. Whichever leverage
    /// (account or instrument) is lower will be used.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 2 requests per second
    ///
    /// # Arguments
    /// * `request` - The change account leverage parameters
    ///
    /// # Returns
    /// Success confirmation (code 0)
    pub async fn change_account_leverage(&self, request: ChangeAccountLeverageRequest) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        let params = serde_json::to_value(&request).map_err(|e| crate::cryptocom::Errors::Error(format!("Serialization error: {}", e)))?;

        self.send_signed_request("private/change-account-leverage", params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;

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
    fn test_change_account_leverage_request_serialization() {
        let request = ChangeAccountLeverageRequest {
            account_id: "52e7c00f-1324-5a6z-bfgt-de445bde21a5".to_string(),
            leverage: 10,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("account_id").unwrap(),
            "52e7c00f-1324-5a6z-bfgt-de445bde21a5"
        );
        assert_eq!(serialized.get("leverage").unwrap(), 10);
    }

    #[test]
    fn test_change_account_leverage_request_minimum_leverage() {
        let request = ChangeAccountLeverageRequest {
            account_id: "52e7c00f-1324-5a6z-bfgt-de445bde21a5".to_string(),
            leverage: 1,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("leverage").unwrap(), 1);
    }

    #[test]
    fn test_change_account_leverage_request_maximum_leverage() {
        let request = ChangeAccountLeverageRequest {
            account_id: "52e7c00f-1324-5a6z-bfgt-de445bde21a5".to_string(),
            leverage: 100,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("leverage").unwrap(), 100);
    }
}
