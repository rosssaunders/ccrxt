use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result, private_client::RestClient};

const DEPOSIT_ADDRESS_ENDPOINT: &str = "/api/v2/deposit-addresses";

/// Request for getting deposit addresses
#[derive(Debug, Clone, Serialize)]
pub struct GetDepositAddressRequest {
    /// Currency code
    pub currency: String,

    /// Chain name (optional, e.g., "eth", "bsc")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
}

/// Deposit address information
#[derive(Debug, Clone, Deserialize)]
pub struct DepositAddress {
    /// Deposit address
    pub address: String,

    /// Address memo/tag (for some currencies)
    pub memo: Option<String>,

    /// Chain name
    pub chain: String,

    /// Contract address (for tokens)
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
}

impl RestClient {
    /// Get deposit address for a currency
    ///
    /// [docs](https://docs.kucoin.com/#get-deposit-addresses-v2)
    pub async fn get_deposit_address(
        &self,
        request: GetDepositAddressRequest,
    ) -> Result<(DepositAddress, ResponseHeaders)> {
        let (response, headers): (RestResponse<DepositAddress>, ResponseHeaders) = self
            .get_with_request(DEPOSIT_ADDRESS_ENDPOINT, &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_address_request_creation() {
        let request = GetDepositAddressRequest {
            currency: "BTC".to_string(),
            chain: Some("btc".to_string()),
        };
        assert_eq!(request.currency, "BTC");
        assert_eq!(request.chain, Some("btc".to_string()));
    }
}
