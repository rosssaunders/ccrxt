//! Custody crypto withdrawal instructions endpoint for Bullish Exchange API

use serde::Deserialize;

use super::client::RestClient;
use crate::bullish::{
    DataOrPaginated, EndpointType, PaginatedResult, PaginationParams, RestResult,
};

/// Endpoint URL path for crypto withdrawal instructions
const CUSTODY_WITHDRAWAL_CRYPTO_ENDPOINT: &str = "/v1/wallets/withdrawal-instructions/crypto";

/// Withdrawal instruction for crypto
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustodyCryptoWithdrawalInstructions {
    /// Network of the native coin or token, e.g. BTC, ETH, EOS
    pub network: Option<String>,

    /// Asset symbol (non-multiplied), e.g. USDC, BTC
    pub symbol: Option<String>,

    /// Address on the given network
    pub address: Option<String>,

    /// Withdrawal fee charged (units of the asset)
    pub fee: Option<String>,

    /// Memo/reference for the transaction
    pub memo: Option<String>,

    /// Descriptive label of destination provided by user
    pub label: Option<String>,

    /// Destination id uniquely identifying a whitelisted address
    pub destination_id: Option<String>,
}

impl RestClient {
    /// Get withdrawal instructions for crypto symbol
    pub async fn get_custody_withdrawal_instructions_crypto(
        &mut self,
        symbol: &str,
        pagination: Option<PaginationParams>,
    ) -> RestResult<PaginatedResult<CustodyCryptoWithdrawalInstructions>> {
        let endpoint = format!("{}/{}", CUSTODY_WITHDRAWAL_CRYPTO_ENDPOINT, symbol);
        let params = pagination.unwrap_or_default();
        let wire: DataOrPaginated<CustodyCryptoWithdrawalInstructions> = self
            .send_get_authenticated_request(&endpoint, params, EndpointType::PrivateCustody)
            .await?;
        Ok(PaginatedResult::from(wire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_withdrawal_instruction_optional_fields() {
        let json = r#"{"network":"ETH","symbol":"USDC","address":"0xabc","fee":"0.001","memo":null,"label":"MyWallet","destinationId":"dest-1"}"#;
        let v: CustodyCryptoWithdrawalInstructions = serde_json::from_str(json).unwrap();
        assert_eq!(v.network.as_deref(), Some("ETH"));
        assert_eq!(v.symbol.as_deref(), Some("USDC"));
        assert_eq!(v.address.as_deref(), Some("0xabc"));
        assert_eq!(v.fee.as_deref(), Some("0.001"));
        assert_eq!(v.label.as_deref(), Some("MyWallet"));
        assert_eq!(v.destination_id.as_deref(), Some("dest-1"));
    }
}
