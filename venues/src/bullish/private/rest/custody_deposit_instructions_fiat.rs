//! Custody fiat deposit instructions endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use crate::bullish::{
    DataOrPaginated, EndpointType, PaginatedResult, PaginationParams,
    PrivateRestClient as RestClient, RestResult,
};

/// Endpoint URL path for fiat deposit instructions
const CUSTODY_DEPOSIT_FIAT_ENDPOINT: &str = "/v1/wallets/deposit-instructions/fiat";

/// Fiat network identifiers for bank transfers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CustodyFiatNetwork {
    Swift,

    Aba,

    Sepa,
}

/// Deposit instruction for fiat
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustodyFiatDepositInstructions {
    /// The network that the account belongs to and the transaction will be performed on SWIFT, ABA or SEPA
    pub network: Option<CustodyFiatNetwork>,

    /// The currency associated with the account, e.g. USD, EUR
    pub symbol: Option<String>,

    /// The Bullish account number
    pub account_number: Option<String>,

    /// Official Bullish account holder name
    pub name: Option<String>,

    /// Bullish entity's physical address for the bank account
    pub physical_address: Option<String>,
}

impl RestClient {
    /// Get deposit instructions for fiat symbol
    pub async fn get_custody_deposit_instructions_fiat(
        &mut self,
        symbol: &str,
        pagination: Option<PaginationParams>,
    ) -> RestResult<PaginatedResult<CustodyFiatDepositInstructions>> {
        let endpoint = format!("{}/{}", CUSTODY_DEPOSIT_FIAT_ENDPOINT, symbol);

        let params = pagination.unwrap_or_default();

        let wire: DataOrPaginated<CustodyFiatDepositInstructions> = self
            .send_get_authenticated_request(&endpoint, params, EndpointType::PrivateCustody)
            .await?;

        Ok(PaginatedResult::from(wire))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fiat_deposit_instruction_optional_fields() {
        let json = r#"{"network":"SWIFT","symbol":"USD","accountNumber":"123","name":"Bullish","physicalAddress":"Addr"}"#;
        let v: CustodyFiatDepositInstructions = serde_json::from_str(json).unwrap();
        assert_eq!(v.network, Some(CustodyFiatNetwork::Swift));
        assert_eq!(v.symbol.as_deref(), Some("USD"));
        assert_eq!(v.account_number.as_deref(), Some("123"));
        assert_eq!(v.name.as_deref(), Some("Bullish"));
        assert_eq!(v.physical_address.as_deref(), Some("Addr"));
    }
}
