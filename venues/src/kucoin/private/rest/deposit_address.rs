use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

use super::RestClient;

/// Request for getting deposit addresses
#[derive(Debug, Clone, Serialize)]
pub struct GetDepositAddressRequest {
    /// Currency code
    pub currency: String,

    /// Chain name (optional, e.g., "eth", "bsc")
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
    /// Reference: https://docs.kucoin.com/#get-deposit-addresses-v2
    pub async fn get_deposit_address(
        &self,
        request: GetDepositAddressRequest,
    ) -> Result<(DepositAddress, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("currency".to_string(), request.currency);

        if let Some(chain) = request.chain {
            params.insert("chain".to_string(), chain);
        }

        let (response, headers): (RestResponse<DepositAddress>, ResponseHeaders) =
            self.get("/api/v2/deposit-addresses", Some(params)).await?;

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
