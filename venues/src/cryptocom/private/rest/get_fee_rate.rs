use serde::Deserialize;
use serde_json::{Value, json};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Fee rate information for user's account
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
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
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 2 requests per second
    ///
    /// # Returns
    /// Fee rate information
    #[allow(clippy::indexing_slicing)] // Safe: adding optional keys to JSON object
    pub async fn get_fee_rate(&self) -> RestResult<Value> {
        
        
        let params = json!({});

        self.send_signed_request("private/get-fee-rate", params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::json;

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
