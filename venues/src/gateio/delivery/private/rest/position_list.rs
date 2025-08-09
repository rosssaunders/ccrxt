use super::RestClient;
use crate::gateio::delivery::{
    RestResult,
    models::{DeliveryPosition, DeliveryPositionsRequest},
};

const DELIVERY_POSITIONS_ENDPOINT: &str = "/delivery/{}/positions";
const DELIVERY_POSITION_ENDPOINT: &str = "/delivery/{}/positions/{}";

impl RestClient {
    /// Get delivery positions
    ///
    /// This endpoint returns all delivery positions for the authenticated user.
    ///
    /// See: <https://www.gate.com/docs/developers/apiv4/#get-user-position-list-2>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery positions request parameters
    ///
    /// # Returns
    /// List of delivery positions
    pub async fn get_delivery_positions(
        &self,
        params: DeliveryPositionsRequest,
    ) -> RestResult<Vec<DeliveryPosition>> {
        let endpoint = DELIVERY_POSITIONS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a specific delivery position
    ///
    /// This endpoint returns details for a specific delivery position.
    ///
    /// See: <https://www.gate.com/docs/developers/apiv4/#get-user-position-list-2>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Contract name
    ///
    /// # Returns
    /// Specific delivery position details
    pub async fn get_delivery_position(
        &self,
        settle: &str,
        contract: &str,
    ) -> RestResult<DeliveryPosition> {
        let endpoint = DELIVERY_POSITION_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", contract, 1);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_positions_endpoint() {
        assert_eq!(DELIVERY_POSITIONS_ENDPOINT, "/delivery/{}/positions");
    }

    #[test]
    fn test_delivery_position_endpoint() {
        assert_eq!(DELIVERY_POSITION_ENDPOINT, "/delivery/{}/positions/{}");
    }

    #[test]
    fn test_get_delivery_positions_endpoint_construction() {
        let settle = "BTC";
        let endpoint = DELIVERY_POSITIONS_ENDPOINT.replace("{}", settle);
        assert_eq!(endpoint, "/delivery/BTC/positions");
    }

    #[test]
    fn test_get_delivery_position_endpoint_construction() {
        let settle = "USDT";
        let contract = "BTC_USDT_20240315";
        let endpoint = DELIVERY_POSITION_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", contract, 1);
        assert_eq!(endpoint, "/delivery/USDT/positions/BTC_USDT_20240315");
    }

    #[test]
    fn test_positions_endpoint_different_settlements() {
        let test_cases = vec![
            ("BTC", "/delivery/BTC/positions"),
            ("USDT", "/delivery/USDT/positions"),
            ("ETH", "/delivery/ETH/positions"),
        ];

        for (settle, expected) in test_cases {
            let endpoint = DELIVERY_POSITIONS_ENDPOINT.replace("{}", settle);
            assert_eq!(endpoint, expected, "Failed for settlement: {}", settle);
        }
    }

    #[test]
    fn test_position_endpoint_different_params() {
        let test_cases = vec![
            (
                "BTC",
                "BTC_USDT_20240315",
                "/delivery/BTC/positions/BTC_USDT_20240315",
            ),
            (
                "USDT",
                "ETH_USDT_20240415",
                "/delivery/USDT/positions/ETH_USDT_20240415",
            ),
            (
                "ETH",
                "SOL_ETH_20240515",
                "/delivery/ETH/positions/SOL_ETH_20240515",
            ),
        ];

        for (settle, contract, expected) in test_cases {
            let endpoint = DELIVERY_POSITION_ENDPOINT
                .replacen("{}", settle, 1)
                .replacen("{}", contract, 1);
            assert_eq!(
                endpoint, expected,
                "Failed for settle: {}, contract: {}",
                settle, contract
            );
        }
    }

    #[test]
    fn test_endpoints_have_correct_placeholders() {
        // Positions endpoint should have one placeholder
        let positions_placeholder_count = DELIVERY_POSITIONS_ENDPOINT.matches("{}").count();
        assert_eq!(positions_placeholder_count, 1);

        // Position endpoint should have two placeholders
        let position_placeholder_count = DELIVERY_POSITION_ENDPOINT.matches("{}").count();
        assert_eq!(position_placeholder_count, 2);
    }

    #[test]
    fn test_endpoint_paths_structure() {
        assert!(DELIVERY_POSITIONS_ENDPOINT.starts_with("/delivery/"));
        assert!(DELIVERY_POSITIONS_ENDPOINT.ends_with("/positions"));

        assert!(DELIVERY_POSITION_ENDPOINT.starts_with("/delivery/"));
        assert!(DELIVERY_POSITION_ENDPOINT.contains("/positions/"));
    }

    #[test]
    fn test_endpoint_parameter_replacement_completeness() {
        let settle = "BTC";
        let contract = "BTC_USDT_20240315";

        let positions_endpoint = DELIVERY_POSITIONS_ENDPOINT.replace("{}", settle);
        assert!(!positions_endpoint.contains("{}"));
        assert!(positions_endpoint.contains(settle));

        let position_endpoint = DELIVERY_POSITION_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", contract, 1);
        assert!(!position_endpoint.contains("{}"));
        assert!(position_endpoint.contains(settle));
        assert!(position_endpoint.contains(contract));
    }

    #[test]
    fn test_contract_naming_patterns() {
        let contracts = vec![
            "BTC_USDT_20240315",
            "ETH_USDT_20240415",
            "SOL_USDT_20240515",
            "DOGE_USDT_20240615",
        ];

        for contract in contracts {
            let endpoint = DELIVERY_POSITION_ENDPOINT
                .replacen("{}", "USDT", 1)
                .replacen("{}", contract, 1);
            assert!(endpoint.contains(contract));
            assert_eq!(endpoint, format!("/delivery/USDT/positions/{}", contract));
        }
    }
}
