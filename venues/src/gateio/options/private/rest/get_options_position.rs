use super::RestClient;
use super::position::OptionsPosition;

const OPTIONS_POSITIONS_ENDPOINT: &str = "/options/positions";

impl RestClient {
    /// Get a specific options position
    ///
    /// This endpoint returns details for a specific options position.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#get-single-position>
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `contract` - Contract name
    ///
    /// # Returns
    /// Specific options position details
    pub async fn get_options_position(
        &self,
        contract: &str,
    ) -> crate::gateio::options::Result<OptionsPosition> {
        let endpoint = format!("{}/{}", OPTIONS_POSITIONS_ENDPOINT, contract);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_options_position_endpoint() {
        let contracts = vec![
            "BTC-20240101-50000-C",
            "BTC-20240101-50000-P",
            "ETH-20240201-3000-C",
            "ETH-20240201-3000-P",
        ];

        for contract in contracts {
            let endpoint = format!("{}/{}", OPTIONS_POSITIONS_ENDPOINT, contract);
            assert!(endpoint.starts_with("/options/positions/"));
            assert!(endpoint.ends_with(contract));
        }
    }

    #[test]
    fn test_get_options_position_endpoint_format() {
        let contract = "BTC-20240101-50000-C";
        let endpoint = format!("/options/positions/{}", contract);
        assert_eq!(endpoint, "/options/positions/BTC-20240101-50000-C");
    }

    #[test]
    fn test_get_options_position_different_contracts() {
        let contracts = vec![
            "BTC-20240101-50000-C",
            "ETH-20240215-3000-P",
            "BNB-20240301-400-C",
            "SOL-20240315-150-P",
            "ADA-20240401-1-C",
        ];

        for contract in contracts {
            let endpoint = format!("{}/{}", OPTIONS_POSITIONS_ENDPOINT, contract);
            assert_eq!(endpoint, format!("/options/positions/{}", contract));
        }
    }

    #[test]
    fn test_get_options_position_special_dates() {
        let contracts = vec![
            "BTC-20231231-50000-C", // End of year
            "BTC-20240101-50000-C", // Start of year
            "BTC-20240229-50000-C", // Leap year date
            "BTC-20241225-50000-C", // Christmas
        ];

        for contract in contracts {
            let endpoint = format!("{}/{}", OPTIONS_POSITIONS_ENDPOINT, contract);
            assert!(endpoint.contains(contract));
        }
    }

    #[test]
    fn test_get_options_position_different_strikes() {
        let contracts = vec![
            "BTC-20240101-10000-C",  // Low strike
            "BTC-20240101-50000-C",  // Mid strike
            "BTC-20240101-100000-C", // High strike
            "ETH-20240101-1000-P",   // Low strike put
            "ETH-20240101-5000-P",   // High strike put
        ];

        for contract in contracts {
            let endpoint = format!("{}/{}", OPTIONS_POSITIONS_ENDPOINT, contract);
            assert!(endpoint.contains(contract));
        }
    }
}
