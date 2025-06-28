use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Request parameters for changing account leverage
#[derive(Debug, Clone, Serialize)]
pub struct ChangeAccountLeverageRequest {
    /// Account ID to change the leverage. Must be currently the logged user's account
    pub account_id: String,
    /// Maximum leverage to be used for the account. Valid values are between 1-100 (inclusive)
    pub leverage: u8,
}

/// Response for change account leverage endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeAccountLeverageResponse {
    /// Success code (typically 0)
    #[serde(default)]
    pub code: i32,
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
    pub async fn change_account_leverage(
        &self,
        request: ChangeAccountLeverageRequest,
    ) -> RestResult<ChangeAccountLeverageResponse> {
        self.send_signed_request("private/change-account-leverage", request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;

    use super::*;

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
