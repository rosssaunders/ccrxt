use super::{
    RestClient, RestResult,
    models::{DeliveryLeverageResponse, SetDeliveryLeverageRequest},
};

const DELIVERY_POSITION_LEVERAGE_ENDPOINT: &str = "/delivery/{}/positions/{}/leverage";

impl RestClient {
    /// Set delivery position leverage
    ///
    /// This endpoint sets the leverage for a specific delivery contract position.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#update-position-leverage)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The leverage setting request parameters
    ///
    /// # Returns
    /// Updated leverage information
    pub async fn set_delivery_position_leverage(
        &self,
        request: SetDeliveryLeverageRequest,
    ) -> RestResult<DeliveryLeverageResponse> {
        let endpoint = DELIVERY_POSITION_LEVERAGE_ENDPOINT
            .replacen("{}", &request.settle, 1)
            .replacen("{}", &request.contract, 1);
        self.post(&endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_position_leverage_endpoint() {
        assert_eq!(
            DELIVERY_POSITION_LEVERAGE_ENDPOINT,
            "/delivery/{}/positions/{}/leverage"
        );
    }

    #[test]
    fn test_leverage_endpoint_construction() {
        let settle = "USDT";
        let contract = "BTC_USDT_20240315";
        let endpoint = DELIVERY_POSITION_LEVERAGE_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", contract, 1);
        assert_eq!(
            endpoint,
            "/delivery/USDT/positions/BTC_USDT_20240315/leverage"
        );
    }

    #[test]
    fn test_leverage_endpoint_different_params() {
        let test_cases = vec![
            (
                "BTC",
                "BTC_USDT_20240315",
                "/delivery/BTC/positions/BTC_USDT_20240315/leverage",
            ),
            (
                "USDT",
                "ETH_USDT_20240415",
                "/delivery/USDT/positions/ETH_USDT_20240415/leverage",
            ),
            (
                "ETH",
                "SOL_ETH_20240515",
                "/delivery/ETH/positions/SOL_ETH_20240515/leverage",
            ),
        ];

        for (settle, contract, expected) in test_cases {
            let endpoint = DELIVERY_POSITION_LEVERAGE_ENDPOINT
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
    fn test_leverage_endpoint_placeholders() {
        let placeholder_count = DELIVERY_POSITION_LEVERAGE_ENDPOINT.matches("{}").count();
        assert_eq!(
            placeholder_count, 2,
            "Endpoint should have exactly two placeholders"
        );
    }

    #[test]
    fn test_leverage_endpoint_structure() {
        assert!(DELIVERY_POSITION_LEVERAGE_ENDPOINT.starts_with("/delivery/"));
        assert!(DELIVERY_POSITION_LEVERAGE_ENDPOINT.contains("/positions/"));
        assert!(DELIVERY_POSITION_LEVERAGE_ENDPOINT.ends_with("/leverage"));
    }

    #[test]
    fn test_leverage_endpoint_parameter_replacement() {
        let settle = "BTC";
        let contract = "test_contract";

        let endpoint = DELIVERY_POSITION_LEVERAGE_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", contract, 1);

        assert!(
            !endpoint.contains("{}"),
            "All placeholders should be replaced"
        );
        assert!(
            endpoint.contains(settle),
            "Endpoint should contain settlement currency"
        );
        assert!(
            endpoint.contains(contract),
            "Endpoint should contain contract name"
        );
    }

    #[test]
    fn test_leverage_endpoint_path_components() {
        let endpoint = DELIVERY_POSITION_LEVERAGE_ENDPOINT;
        let parts: Vec<&str> = endpoint.split('/').collect();

        assert_eq!(parts[0], ""); // Leading slash
        assert_eq!(parts[1], "delivery");
        assert_eq!(parts[2], "{}"); // Settlement placeholder
        assert_eq!(parts[3], "positions");
        assert_eq!(parts[4], "{}"); // Contract placeholder
        assert_eq!(parts[5], "leverage");
    }

    #[test]
    fn test_leverage_endpoint_with_different_settlements() {
        let settlements = vec!["BTC", "USDT", "ETH", "SOL"];
        let contract = "TEST_CONTRACT";

        for settle in settlements {
            let endpoint = DELIVERY_POSITION_LEVERAGE_ENDPOINT
                .replacen("{}", settle, 1)
                .replacen("{}", contract, 1);

            assert!(endpoint.contains(settle));
            assert!(endpoint.contains(contract));
            assert_eq!(
                endpoint,
                format!("/delivery/{}/positions/{}/leverage", settle, contract)
            );
        }
    }
}
