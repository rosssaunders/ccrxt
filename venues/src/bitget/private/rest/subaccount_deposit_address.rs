use crate::bitget::{
    BitgetRestClient,
};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Request for getting subaccount deposit address
#[derive(Debug, Clone, Serialize)]
pub struct GetSubaccountDepositAddressRequest {
    /// Sub Account UID
    #[serde(rename = "subUid")]
    pub sub_uid: String,
    /// Coin name, e.g. USDT
    pub coin: String,
    /// Chain name, e.g. trc20 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// Bitcoin Lightning Network withdrawal amount (optional, limit: 0.000001 - 0.01)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

/// Response for getting subaccount deposit address
#[derive(Debug, Clone, Deserialize)]
pub struct GetSubaccountDepositAddressResponse {
    /// Deposit address
    pub address: String,
    /// Chain name
    pub chain: String,
    /// Token name
    pub coin: String,
    /// Tag
    pub tag: String,
    /// Blockchain address URL
    pub url: String,
}

impl GetSubaccountDepositAddressRequest {
    /// Create a new request builder
    pub fn builder() -> GetSubaccountDepositAddressRequestBuilder {
        GetSubaccountDepositAddressRequestBuilder::default()
    }
}

impl BitgetRequest for GetSubaccountDepositAddressRequest {
    type Response = GetSubaccountDepositAddressResponse;

    fn path(&self) -> String {
        "/api/v2/spot/wallet/subaccount-deposit-address".to_string()
    }

    fn method(&self) -> String {
        "GET".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

/// Builder for GetSubaccountDepositAddressRequest
#[derive(Debug, Default)]
pub struct GetSubaccountDepositAddressRequestBuilder {
    sub_uid: Option<String>,
    coin: Option<String>,
    chain: Option<String>,
    size: Option<String>,
}

impl GetSubaccountDepositAddressRequestBuilder {
    /// Set the sub-account UID
    pub fn sub_uid(mut self, sub_uid: impl Into<String>) -> Self {
        self.sub_uid = Some(sub_uid.into());
        self
    }

    /// Set the coin name
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }

    /// Set the chain name (optional)
    pub fn chain(mut self, chain: impl Into<String>) -> Self {
        self.chain = Some(chain.into());
        self
    }

    /// Set the size for Bitcoin Lightning Network (optional)
    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Build the request
    pub fn build(self) -> GetSubaccountDepositAddressRequest {
        GetSubaccountDepositAddressRequest {
            sub_uid: self.sub_uid.expect("sub_uid is required"),
            coin: self.coin.expect("coin is required"),
            chain: self.chain,
            size: self.size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder() {
        let request = GetSubaccountDepositAddressRequest::builder()
            .sub_uid("123456")
            .coin("USDT")
            .chain("ERC20")
            .build();

        assert_eq!(request.sub_uid, "123456");
        assert_eq!(request.coin, "USDT");
        assert_eq!(request.chain, Some("ERC20".to_string()));
        assert_eq!(request.size, None);
    }

    #[test]
    fn test_request_builder_minimal() {
        let request = GetSubaccountDepositAddressRequest::builder()
            .sub_uid("123456")
            .coin("BTC")
            .build();

        assert_eq!(request.sub_uid, "123456");
        assert_eq!(request.coin, "BTC");
        assert_eq!(request.chain, None);
        assert_eq!(request.size, None);
    }
}
