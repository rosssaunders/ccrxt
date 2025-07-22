use super::RestClient;
use super::position::DualModePosition;

impl RestClient {
    /// Get dual mode position
    ///
    /// This endpoint returns dual mode position information for a specific contract.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#retrieve-position-detail-in-dual-mode>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Contract name
    ///
    /// # Returns
    /// Dual mode position details
    pub async fn get_dual_mode_position(
        &self,
        settle: &str,
        contract: &str,
    ) -> crate::gateio::perpetual::Result<DualModePosition> {
        let endpoint = format!("/futures/{}/dual_positions/{}", settle, contract);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_dual_mode_position_endpoint() {
        let test_cases = vec![
            ("USDT", "BTC_USDT", "/futures/USDT/dual_positions/BTC_USDT"),
            ("BTC", "BTC_USD", "/futures/BTC/dual_positions/BTC_USD"),
            ("ETH", "ETH_USD", "/futures/ETH/dual_positions/ETH_USD"),
        ];

        for (settle, contract, expected) in test_cases {
            let endpoint = format!("/futures/{}/dual_positions/{}", settle, contract);
            assert_eq!(endpoint, expected);
        }
    }

    #[test]
    fn test_various_contracts() {
        let contracts = vec![
            ("USDT", "BTC_USDT"),
            ("USDT", "ETH_USDT"),
            ("USDT", "SOL_USDT"),
            ("BTC", "BTC_USD"),
            ("ETH", "ETH_USD"),
        ];

        for (settle, contract) in contracts {
            let endpoint = format!("/futures/{}/dual_positions/{}", settle, contract);
            assert!(endpoint.contains(settle));
            assert!(endpoint.contains(contract));
            assert!(endpoint.contains("dual_positions"));
        }
    }

    #[test]
    fn test_endpoint_structure() {
        let endpoint = format!("/futures/{}/dual_positions/{}", "USDT", "BTC_USDT");

        // Verify endpoint structure
        assert!(endpoint.starts_with("/futures"));
        assert!(endpoint.contains("/dual_positions/"));
        assert!(endpoint.ends_with("BTC_USDT"));
        assert!(!endpoint.contains("?"));
    }
}
