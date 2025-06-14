use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Fee rate information for user's account
#[derive(Debug, Clone, Deserialize)]
pub struct FeeRate {
    /// 30-day spot trading volume tier
    pub spot_tier: String,
    /// 30-day derivatives trading volume tier
    pub deriv_tier: String,
    /// 30-day spot maker rate in basis points
    pub effective_spot_maker_rate_bps: String,
    /// 30-day spot taker rate in basis points
    pub effective_spot_taker_rate_bps: String,
    /// 30-day derivatives maker rate in basis points
    pub effective_deriv_maker_rate_bps: String,
    /// 30-day derivatives taker rate in basis points
    pub effective_deriv_taker_rate_bps: String,
}

impl RestClient {
    /// Get fee rates for user's account
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html#private-get-fee-rate>
    ///
    /// Rate limit: 2 requests per second
    ///
    /// # Returns
    /// Fee rate information
    pub async fn get_fee_rate(&self) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        let params = json!({});

        let signature = self.sign_request("private/get-fee-rate", id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": "private/get-fee-rate",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(&format!("{}/v1/private/get-fee-rate", self.base_url))
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
    fn test_fee_rate_structure() {
        let fee_rate_json = json!({
            "spot_tier": "3",
            "deriv_tier": "3",
            "effective_spot_maker_rate_bps": "6.5",
            "effective_spot_taker_rate_bps": "6.9",
            "effective_deriv_maker_rate_bps": "1.1",
            "effective_deriv_taker_rate_bps": "3"
        });

        let fee_rate: FeeRate = serde_json::from_value(fee_rate_json).unwrap();
        assert_eq!(fee_rate.spot_tier, "3");
        assert_eq!(fee_rate.deriv_tier, "3");
        assert_eq!(fee_rate.effective_spot_maker_rate_bps, "6.5");
        assert_eq!(fee_rate.effective_spot_taker_rate_bps, "6.9");
        assert_eq!(fee_rate.effective_deriv_maker_rate_bps, "1.1");
        assert_eq!(fee_rate.effective_deriv_taker_rate_bps, "3");
    }

    #[test]
    fn test_fee_rate_different_tiers() {
        let fee_rate_json = json!({
            "spot_tier": "0",
            "deriv_tier": "1",
            "effective_spot_maker_rate_bps": "10.0",
            "effective_spot_taker_rate_bps": "10.0",
            "effective_deriv_maker_rate_bps": "4.0",
            "effective_deriv_taker_rate_bps": "5.5"
        });

        let fee_rate: FeeRate = serde_json::from_value(fee_rate_json).unwrap();
        assert_eq!(fee_rate.spot_tier, "0");
        assert_eq!(fee_rate.deriv_tier, "1");
        assert_eq!(fee_rate.effective_spot_maker_rate_bps, "10.0");
        assert_eq!(fee_rate.effective_spot_taker_rate_bps, "10.0");
        assert_eq!(fee_rate.effective_deriv_maker_rate_bps, "4.0");
        assert_eq!(fee_rate.effective_deriv_taker_rate_bps, "5.5");
    }
}
