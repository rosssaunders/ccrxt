use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const ACCOUNT_DEBIT_FEE_ENDPOINT: &str = "/account/debit_fee";

/// Request/response for GT fee deduction configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DebitFeeConfig {
    /// Whether GT fee deduction is enabled for the account.
    pub enabled: bool,
}

impl RestClient {
    /// Configure GT fee deduction for the current account.
    ///
    /// Enables or disables GT fee deduction for the current account.
    ///
    /// - [API docs](https://www.gate.io/docs/developers/apiv4/en/#configure-gt-fee-deduction)
    ///
    /// # Arguments
    /// * `config` - The GT fee deduction configuration to set.
    ///
    /// # Returns
    /// Returns `Ok(())` on success, or an error if the request fails.
    pub async fn set_debit_fee(&self, config: &DebitFeeConfig) -> RestResult<()> {
        // Gate.io returns 200 with empty body for success
        let _: serde_json::Value = self
            .send_post_request(ACCOUNT_DEBIT_FEE_ENDPOINT, Some(config))
            .await?;
        Ok(())
    }

    /// Query GT fee deduction configuration for the current account.
    ///
    /// Retrieves the current GT fee deduction configuration for the account.
    ///
    /// - [API docs](https://www.gate.io/docs/developers/apiv4/en/#query-gt-fee-deduction-configuration)
    ///
    /// # Returns
    /// Returns the current GT fee deduction configuration, or an error if the request fails.
    pub async fn get_debit_fee(&self) -> RestResult<DebitFeeConfig> {
        self.send_get_request(ACCOUNT_DEBIT_FEE_ENDPOINT, Option::<&()>::None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debit_fee_config_roundtrip() {
        let cfg = DebitFeeConfig { enabled: true };
        let json = serde_json::to_string(&cfg).unwrap();
        let back: DebitFeeConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg, back);
    }

    #[test]
    fn test_debit_fee_config_default() {
        let cfg = DebitFeeConfig::default();
        // Default should be false (Rust default for bool)
        assert!(!cfg.enabled);
    }
}
