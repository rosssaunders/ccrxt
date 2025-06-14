use super::client::RestClient;
use crate::cryptocom::{RestResult, StpInst, StpScope};
use serde::Serialize;
use serde_json::{json, Value};

/// Request parameters for changing account settings
#[derive(Debug, Clone, Serialize)]
pub struct ChangeAccountSettingsRequest {
    /// Self-trade prevention scope: M (Matches Master or Sub a/c), S (Matches Sub a/c only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_scope: Option<StpScope>,
    /// Self-trade prevention instruction: M (Cancel Maker), T (Cancel Taker), B (Cancel Both)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_inst: Option<StpInst>,
    /// STP ID value: 0 to 32767
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_id: Option<String>,
    /// Maximum leverage user intends to set for the account. Valid values are between 1-50 (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<u8>,
}

impl RestClient {
    /// Change the account STP settings
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html#private-change-account-settings>
    ///
    /// Rate limit: 2 requests per second
    ///
    /// # Arguments
    /// * `request` - The change account settings parameters
    ///
    /// # Returns
    /// Success confirmation (code 0)
    pub async fn change_account_settings(
        &self,
        request: ChangeAccountSettingsRequest,
    ) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        let params = serde_json::to_value(&request)
            .map_err(|e| crate::cryptocom::Errors::Error(format!("Serialization error: {}", e)))?;

        let signature = self.sign_request("private/change-account-settings", id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": "private/change-account-settings",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(format!(
                "{}/v1/private/change-account-settings",
                self.base_url
            ))
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
    fn test_change_account_settings_request_full() {
        let request = ChangeAccountSettingsRequest {
            stp_scope: Some(StpScope::SubAccountOnly),
            stp_inst: Some(StpInst::CancelMaker),
            stp_id: Some("100".to_string()),
            leverage: Some(20),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("stp_scope").unwrap(), "S");
        assert_eq!(serialized.get("stp_inst").unwrap(), "M");
        assert_eq!(serialized.get("stp_id").unwrap(), "100");
        assert_eq!(serialized.get("leverage").unwrap(), 20);
    }

    #[test]
    fn test_change_account_settings_request_stp_only() {
        let request = ChangeAccountSettingsRequest {
            stp_scope: Some(StpScope::MasterOrSubAccount),
            stp_inst: Some(StpInst::CancelBoth),
            stp_id: Some("0".to_string()),
            leverage: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("stp_scope").unwrap(), "M");
        assert_eq!(serialized.get("stp_inst").unwrap(), "B");
        assert_eq!(serialized.get("stp_id").unwrap(), "0");
        assert!(!serialized.as_object().unwrap().contains_key("leverage"));
    }

    #[test]
    fn test_change_account_settings_request_leverage_only() {
        let request = ChangeAccountSettingsRequest {
            stp_scope: None,
            stp_inst: None,
            stp_id: None,
            leverage: Some(50),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("leverage").unwrap(), 50);
        assert!(!serialized.as_object().unwrap().contains_key("stp_scope"));
        assert!(!serialized.as_object().unwrap().contains_key("stp_inst"));
        assert!(!serialized.as_object().unwrap().contains_key("stp_id"));
    }

    #[test]
    fn test_change_account_settings_request_empty() {
        let request = ChangeAccountSettingsRequest {
            stp_scope: None,
            stp_inst: None,
            stp_id: None,
            leverage: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert!(!serialized.as_object().unwrap().contains_key("stp_scope"));
        assert!(!serialized.as_object().unwrap().contains_key("stp_inst"));
        assert!(!serialized.as_object().unwrap().contains_key("stp_id"));
        assert!(!serialized.as_object().unwrap().contains_key("leverage"));
    }
}
