use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request to set cross margin mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossModeRequest {
    /// Mode ("cross" for cross margin)
    pub mode: String,
}

impl RestClient {
    /// Switch to cross margin mode
    ///
    /// This endpoint switches the user's margin mode to cross margin.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#switch-between-cross-margin-and-isolated-margin>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    ///
    /// # Returns
    /// Empty response indicating success
    pub async fn switch_to_cross_margin(
        &self,
        settle: &str,
    ) -> crate::gateio::perpetual::Result<()> {
        let endpoint = format!("/futures/{}/cross_mode", settle);
        let request = CrossModeRequest {
            mode: "cross".to_string(),
        };
        self.post::<serde_json::Value>(&endpoint, &request).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_mode_request() {
        let request = CrossModeRequest {
            mode: "cross".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["mode"], "cross");
    }

    #[test]
    fn test_endpoint_formatting() {
        let settlements = vec!["USDT", "BTC", "ETH"];

        for settle in settlements {
            let endpoint = format!("/futures/{}/cross_mode", settle);
            assert!(endpoint.contains(settle));
            assert!(endpoint.starts_with("/futures"));
            assert!(endpoint.ends_with("/cross_mode"));
        }
    }

    #[test]
    fn test_cross_mode_serialization() {
        let request = CrossModeRequest {
            mode: "cross".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"mode\":\"cross\""));
    }

    #[test]
    fn test_different_settlements() {
        let settlements = vec![
            ("USDT", "/futures/USDT/cross_mode"),
            ("BTC", "/futures/BTC/cross_mode"),
            ("ETH", "/futures/ETH/cross_mode"),
        ];

        for (settle, expected) in settlements {
            let endpoint = format!("/futures/{}/cross_mode", settle);
            assert_eq!(endpoint, expected);
        }
    }
}
