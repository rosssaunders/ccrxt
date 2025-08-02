use super::{RestClient, mmp_settings::MMPSettings};

const OPTIONS_MMP_ENDPOINT: &str = "/options/mmp";

impl RestClient {
    /// Get MMP settings
    ///
    /// This endpoint returns Market Maker Protection settings for options trading.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-mmp-settings>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `underlying` - Underlying asset
    ///
    /// # Returns
    /// Current MMP settings
    pub async fn get_mmp_settings(
        &self,
        underlying: &str,
    ) -> crate::gateio::options::Result<MMPSettings> {
        let endpoint = format!("{}?underlying={}", OPTIONS_MMP_ENDPOINT, underlying);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_mmp_settings_endpoint() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT"];

        for underlying in underlyings {
            let endpoint = format!("{}?underlying={}", OPTIONS_MMP_ENDPOINT, underlying);
            assert!(endpoint.starts_with("/options/mmp?underlying="));
            assert!(endpoint.contains(underlying));
        }
    }

    #[test]
    fn test_get_mmp_settings_endpoint_format() {
        let underlying = "BTC_USDT";
        let endpoint = format!("{}?underlying={}", OPTIONS_MMP_ENDPOINT, underlying);
        assert_eq!(endpoint, "/options/mmp?underlying=BTC_USDT");
    }

    #[test]
    fn test_get_mmp_settings_endpoint_special_chars() {
        // Test with underlyings that might have special characters
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BTC-PERP", "ETH-PERP"];

        for underlying in underlyings {
            let endpoint = format!("{}?underlying={}", OPTIONS_MMP_ENDPOINT, underlying);
            assert!(endpoint.contains(underlying));
            assert_eq!(endpoint, format!("/options/mmp?underlying={}", underlying));
        }
    }
}
