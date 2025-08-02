use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::RestResult;

const SUBACCOUNT_DEPOSIT_ADDRESS_ENDPOINT: &str = "/api/v2/spot/wallet/subaccount-deposit-address";

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

impl RestClient {
    /// Get Subaccount Deposit Address
    pub async fn get_subaccount_deposit_address(
        &self,
        request: GetSubaccountDepositAddressRequest,
    ) -> RestResult<GetSubaccountDepositAddressResponse> {
        self.send_get_signed_request(SUBACCOUNT_DEPOSIT_ADDRESS_ENDPOINT, request,
            10,
            false,
            None,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder() {
        let request = GetSubaccountDepositAddressRequest {
            sub_uid: "123456".to_string(),
            coin: "USDT".to_string(),
            chain: Some("ERC20".to_string()),
            size: None,
        };

        assert_eq!(request.sub_uid, "123456");
        assert_eq!(request.coin, "USDT");
        assert_eq!(request.chain, Some("ERC20".to_string()));
        assert_eq!(request.size, None);
    }

    #[test]
    fn test_request_builder_minimal() {
        let request = GetSubaccountDepositAddressRequest {
            sub_uid: "123456".to_string(),
            coin: "BTC".to_string(),
            chain: None,
            size: None,
        };

        assert_eq!(request.sub_uid, "123456");
        assert_eq!(request.coin, "BTC");
        assert_eq!(request.chain, None);
        assert_eq!(request.size, None);
    }
}
