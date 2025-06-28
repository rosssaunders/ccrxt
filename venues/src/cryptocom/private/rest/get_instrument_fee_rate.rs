use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::RestResult;

/// Request parameters for getting instrument fee rate
#[derive(Debug, Clone, Serialize)]
pub struct GetInstrumentFeeRateRequest {
    /// Instrument name e.g. BTC_USD, BTCUSD-PERP
    pub instrument_name: String,
}

/// Instrument fee rate information
#[derive(Debug, Clone, Deserialize)]
pub struct InstrumentFeeRate {
    /// Instrument name e.g. BTC_USD
    pub instrument_name: String,
    /// Maker rate in basis points
    pub effective_maker_rate_bps: String,
    /// Taker rate in basis points
    pub effective_taker_rate_bps: String,
}

/// Response for get instrument fee rate endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstrumentFeeRateResponse {
    /// Fee rate information
    pub result: InstrumentFeeRate,
}

impl RestClient {
    /// Get the instrument fee rate
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: 2 requests per second
    ///
    /// # Arguments
    /// * `request` - The get instrument fee rate parameters
    ///
    /// # Returns
    /// Instrument fee rate information
    pub async fn get_instrument_fee_rate(
        &self,
        request: GetInstrumentFeeRateRequest,
    ) -> RestResult<GetInstrumentFeeRateResponse> {
        self.send_signed_request("private/get-instrument-fee-rate", request)
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
    fn test_get_instrument_fee_rate_request_spot() {
        let request = GetInstrumentFeeRateRequest {
            instrument_name: "BTC_USD".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTC_USD");
    }

    #[test]
    fn test_get_instrument_fee_rate_request_perp() {
        let request = GetInstrumentFeeRateRequest {
            instrument_name: "BTCUSD-PERP".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instrument_name").unwrap(), "BTCUSD-PERP");
    }

    #[test]
    fn test_instrument_fee_rate_structure() {
        let fee_rate_json = json!({
            "instrument_name": "BTC_USD",
            "effective_maker_rate_bps": "6.5",
            "effective_taker_rate_bps": "6.9"
        });

        let fee_rate: InstrumentFeeRate = serde_json::from_value(fee_rate_json).unwrap();
        assert_eq!(fee_rate.instrument_name, "BTC_USD");
        assert_eq!(fee_rate.effective_maker_rate_bps, "6.5");
        assert_eq!(fee_rate.effective_taker_rate_bps, "6.9");
    }

    #[test]
    fn test_instrument_fee_rate_structure_perp() {
        let fee_rate_json = json!({
            "instrument_name": "BTCUSD-PERP",
            "effective_maker_rate_bps": "1.1",
            "effective_taker_rate_bps": "3.0"
        });

        let fee_rate: InstrumentFeeRate = serde_json::from_value(fee_rate_json).unwrap();
        assert_eq!(fee_rate.instrument_name, "BTCUSD-PERP");
        assert_eq!(fee_rate.effective_maker_rate_bps, "1.1");
        assert_eq!(fee_rate.effective_taker_rate_bps, "3.0");
    }
}
