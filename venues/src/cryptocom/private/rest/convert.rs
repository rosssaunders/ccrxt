use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Request parameters for convert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertRequest {
    /// Instrument name to convert from: ETH.staked or CDCETH
    pub from_instrument_name: String,
    /// Instrument name to convert to: CDCETH if from_instrument_name is ETH.staked, ETH.staked if from_instrument_name is CDCETH
    pub to_instrument_name: String,
    /// Expected conversion rate, received from public/staking/get-conversion-rate
    pub expected_rate: String,
    /// Quantity to be converted in from_instrument_name
    pub from_quantity: String,
    /// Maximum slippage allowed in basis point
    pub slippage_tolerance_bps: String,
}

/// Convert response information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertResponse {
    /// Instrument name to convert from, e.g. ETH.staked
    pub from_instrument_name: String,
    /// Instrument name to convert to, e.g. CDCETH
    pub to_instrument_name: String,
    /// Expected conversion rate
    pub expected_rate: String,
    /// Quantity to be converted in from_instrument_name
    pub from_quantity: String,
    /// Maximum slippage allowed in basis point
    pub slippage_tolerance_bps: String,
    /// Convert request id
    pub convert_id: String,
    /// Reason for the status, e.g. "NO_ERROR"
    pub reason: String,
}

impl RestClient {
    /// Create a request to convert between staked token with liquid staking token
    ///
    /// Creates a conversion request between staked tokens and liquid staking tokens.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/index.html#private-staking-convert>
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Arguments
    /// * `from_instrument_name` - Instrument name to convert from: "ETH.staked" or "CDCETH"
    /// * `to_instrument_name` - Instrument name to convert to: "CDCETH" if from is "ETH.staked", "ETH.staked" if from is "CDCETH"
    /// * `expected_rate` - Expected conversion rate, received from public/staking/get-conversion-rate
    /// * `from_quantity` - Quantity to be converted in from_instrument_name
    /// * `slippage_tolerance_bps` - Maximum slippage allowed in basis point
    ///
    /// # Returns
    /// Convert request information including convert ID, rates, and reason
    pub async fn convert(
        &self,
        from_instrument_name: &str,
        to_instrument_name: &str,
        expected_rate: &str,
        from_quantity: &str,
        slippage_tolerance_bps: &str,
    ) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;

        let params = json!({
            "from_instrument_name": from_instrument_name,
            "to_instrument_name": to_instrument_name,
            "expected_rate": expected_rate,
            "from_quantity": from_quantity,
            "slippage_tolerance_bps": slippage_tolerance_bps
        });

        let signature = self.sign_request("private/staking/convert", id, &params, nonce)?;

        let request_body = json!({
            "id": id,
            "method": "private/staking/convert",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(&format!("{}/v1/private/staking/convert", self.base_url))
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
    fn test_convert_request_structure() {
        let request = ConvertRequest {
            from_instrument_name: "ETH.staked".to_string(),
            to_instrument_name: "CDCETH".to_string(),
            expected_rate: "1.0203".to_string(),
            from_quantity: "3.14159265".to_string(),
            slippage_tolerance_bps: "3".to_string(),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("from_instrument_name").unwrap(), "ETH.staked");
        assert_eq!(json_value.get("to_instrument_name").unwrap(), "CDCETH");
        assert_eq!(json_value.get("expected_rate").unwrap(), "1.0203");
        assert_eq!(json_value.get("from_quantity").unwrap(), "3.14159265");
        assert_eq!(json_value.get("slippage_tolerance_bps").unwrap(), "3");
    }

    #[test]
    fn test_convert_request_reverse_direction() {
        let request = ConvertRequest {
            from_instrument_name: "CDCETH".to_string(),
            to_instrument_name: "ETH.staked".to_string(),
            expected_rate: "0.9801".to_string(),
            from_quantity: "2.0".to_string(),
            slippage_tolerance_bps: "5".to_string(),
        };

        let json_value = serde_json::to_value(request).unwrap();
        assert_eq!(json_value.get("from_instrument_name").unwrap(), "CDCETH");
        assert_eq!(json_value.get("to_instrument_name").unwrap(), "ETH.staked");
        assert_eq!(json_value.get("expected_rate").unwrap(), "0.9801");
        assert_eq!(json_value.get("from_quantity").unwrap(), "2.0");
        assert_eq!(json_value.get("slippage_tolerance_bps").unwrap(), "5");
    }

    #[test]
    fn test_convert_request_serialization() {
        let request = ConvertRequest {
            from_instrument_name: "ETH.staked".to_string(),
            to_instrument_name: "CDCETH".to_string(),
            expected_rate: "1.0203".to_string(),
            from_quantity: "1.5".to_string(),
            slippage_tolerance_bps: "10".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ConvertRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.from_instrument_name, "ETH.staked");
        assert_eq!(deserialized.to_instrument_name, "CDCETH");
        assert_eq!(deserialized.expected_rate, "1.0203");
        assert_eq!(deserialized.from_quantity, "1.5");
        assert_eq!(deserialized.slippage_tolerance_bps, "10");
    }

    #[test]
    fn test_convert_response_structure() {
        let response_json = json!({
            "from_instrument_name": "ETH.staked",
            "to_instrument_name": "CDCETH",
            "expected_rate": "1.0203",
            "from_quantity": "3.14159265",
            "slippage_tolerance_bps": "3",
            "convert_id": "1",
            "reason": "NO_ERROR"
        });

        let response: ConvertResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.from_instrument_name, "ETH.staked");
        assert_eq!(response.to_instrument_name, "CDCETH");
        assert_eq!(response.expected_rate, "1.0203");
        assert_eq!(response.from_quantity, "3.14159265");
        assert_eq!(response.slippage_tolerance_bps, "3");
        assert_eq!(response.convert_id, "1");
        assert_eq!(response.reason, "NO_ERROR");
    }

    #[test]
    fn test_convert_response_different_convert_ids() {
        let convert_ids = vec!["1", "123", "456789"];

        for convert_id in convert_ids {
            let response_json = json!({
                "from_instrument_name": "CDCETH",
                "to_instrument_name": "ETH.staked",
                "expected_rate": "0.9801",
                "from_quantity": "5.0",
                "slippage_tolerance_bps": "5",
                "convert_id": convert_id,
                "reason": "NO_ERROR"
            });

            let response: ConvertResponse = serde_json::from_value(response_json).unwrap();
            assert_eq!(response.convert_id, convert_id);
        }
    }

    #[test]
    fn test_convert_different_slippage_tolerances() {
        let slippages = vec!["1", "3", "5", "10", "50"];

        for slippage in slippages {
            let request = ConvertRequest {
                from_instrument_name: "ETH.staked".to_string(),
                to_instrument_name: "CDCETH".to_string(),
                expected_rate: "1.0203".to_string(),
                from_quantity: "1.0".to_string(),
                slippage_tolerance_bps: slippage.to_string(),
            };

            let json_value = serde_json::to_value(request).unwrap();
            assert_eq!(json_value.get("slippage_tolerance_bps").unwrap(), slippage);
        }
    }
}
