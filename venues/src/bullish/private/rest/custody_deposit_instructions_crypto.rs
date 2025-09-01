//! Custody crypto deposit instructions endpoint for Bullish Exchange API

use serde::Deserialize;

use crate::bullish::{
    DataOrPaginated, EndpointType, PaginatedResult, PaginationParams,
    PrivateRestClient as RestClient, RestResult,
};

/// Endpoint URL path for crypto deposit instructions
const CUSTODY_DEPOSIT_CRYPTO_ENDPOINT: &str = "/v1/wallets/deposit-instructions/crypto";

/// Deposit instruction for crypto
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustodyCryptoDepositInstructions {
    /// Network of the native coin or token, e.g. BTC, ETH, EOS
    pub network: String,

    /// Asset symbol (non-multiplied), e.g. USDC, BTC
    pub symbol: String,

    /// Optional memo or destination tag
    pub memo: Option<String>,

    /// Address on the given network
    pub address: String,
}

impl RestClient {
    /// Get deposit instructions for crypto symbol
    pub async fn get_custody_deposit_instructions_crypto(
        &mut self,
        symbol: &str,
        pagination: Option<PaginationParams>,
    ) -> RestResult<PaginatedResult<CustodyCryptoDepositInstructions>> {
        let endpoint = format!("{}/{}", CUSTODY_DEPOSIT_CRYPTO_ENDPOINT, symbol);
        // API supports pagination flags; when None, send empty tuple
        let params = pagination.unwrap_or_default();
        let wire: DataOrPaginated<CustodyCryptoDepositInstructions> = self
            .send_get_authenticated_request(&endpoint, params, EndpointType::PrivateCustody)
            .await?;
        Ok(PaginatedResult::from(wire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_deposit_instruction_fields() {
        let json = r#"{"network":"ETH","symbol":"USDC","memo":null,"address":"0xabc"}"#;
        let v: CustodyCryptoDepositInstructions = serde_json::from_str(json).unwrap();
        assert_eq!(v.network, "ETH");
        assert_eq!(v.symbol, "USDC");
        assert_eq!(v.address, "0xabc");
    }
}
