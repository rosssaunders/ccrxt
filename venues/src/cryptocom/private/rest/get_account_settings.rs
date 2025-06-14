use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Account settings information
#[derive(Debug, Clone, Deserialize)]
pub struct AccountSettings {
    /// Maximum leverage user set on the account
    pub leverage: u8,
    /// STP ID value: 0 to 32767
    pub stp_id: u16,
    /// Self-trade prevention scope: M (Matches Master or Sub a/c), S (Matches Sub a/c only)
    pub stp_scope: String,
    /// Self-trade prevention instruction: M (Cancel Maker), T (Cancel Taker), B (Cancel Both)
    pub stp_inst: String,
}

/// Response for getting account settings
#[derive(Debug, Clone, Deserialize)]
pub struct GetAccountSettingsResponse {
    /// Array of account settings (typically one element)
    #[serde(rename = "result")]
    pub data: Vec<AccountSettings>,
}

impl RestClient {
    /// Get the STP account settings
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html#private-get-account-settings>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Returns
    /// Account settings information
    pub async fn get_account_settings(&self) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        let params = json!({});

        let signature = self.sign_request("private/get-account-settings", id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": "private/get-account-settings",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(&format!(
                "{}/v1/private/get-account-settings",
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
    fn test_account_settings_structure() {
        let settings_json = json!({
            "leverage": 20,
            "stp_id": 100,
            "stp_scope": "S",
            "stp_inst": "M"
        });

        let settings: AccountSettings = serde_json::from_value(settings_json).unwrap();
        assert_eq!(settings.leverage, 20);
        assert_eq!(settings.stp_id, 100);
        assert_eq!(settings.stp_scope, "S");
        assert_eq!(settings.stp_inst, "M");
    }

    #[test]
    fn test_get_account_settings_response_structure() {
        let response_json = json!({
            "result": [
                {
                    "leverage": 20,
                    "stp_id": 100,
                    "stp_scope": "S",
                    "stp_inst": "M"
                }
            ]
        });

        let response: GetAccountSettingsResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].leverage, 20);
        assert_eq!(response.data[0].stp_id, 100);
        assert_eq!(response.data[0].stp_scope, "S");
        assert_eq!(response.data[0].stp_inst, "M");
    }

    #[test]
    fn test_account_settings_different_values() {
        let settings_json = json!({
            "leverage": 50,
            "stp_id": 0,
            "stp_scope": "M",
            "stp_inst": "B"
        });

        let settings: AccountSettings = serde_json::from_value(settings_json).unwrap();
        assert_eq!(settings.leverage, 50);
        assert_eq!(settings.stp_id, 0);
        assert_eq!(settings.stp_scope, "M");
        assert_eq!(settings.stp_inst, "B");
    }
}
