use serde::Serialize;

use super::RestClient;

const RESET_MMP_ENDPOINT: &str = "/options/mmp/reset";

/// Request to reset MMP
#[derive(Debug, Clone, Serialize)]
pub struct ResetMMPRequest {
    /// Underlying asset
    pub underlying: String,
}

impl RestClient {
    /// Reset MMP
    ///
    /// This endpoint resets the Market Maker Protection state.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#reset-mmp>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `underlying` - Underlying asset to reset MMP for
    ///
    /// # Returns
    /// Empty response indicating success
    pub async fn reset_mmp(&self, underlying: &str) -> crate::gateio::options::Result<()> {
        let request = ResetMMPRequest {
            underlying: underlying.to_string(),
        };
        self.post(RESET_MMP_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_mmp_request_serialization() {
        let request = ResetMMPRequest {
            underlying: "BTC_USDT".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1); // Only underlying field
    }

    #[test]
    fn test_reset_mmp_request_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT", "ADA_USDT"];

        for underlying in underlyings {
            let request = ResetMMPRequest {
                underlying: underlying.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["underlying"], underlying);
        }
    }

    #[test]
    fn test_reset_mmp_request_special_chars() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BTC-PERP", "ETH-PERP"];

        for underlying in underlyings {
            let request = ResetMMPRequest {
                underlying: underlying.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["underlying"], underlying);
        }
    }

    #[test]
    fn test_reset_mmp_request_empty_underlying() {
        let request = ResetMMPRequest {
            underlying: "".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["underlying"], "");
    }

    #[test]
    fn test_reset_mmp_request_json_string() {
        let request = ResetMMPRequest {
            underlying: "BTC_USDT".to_string(),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        assert_eq!(json_str, r#"{"underlying":"BTC_USDT"}"#);
    }
}
