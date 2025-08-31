use serde::{Deserialize, Serialize};

use crate::bitmart::{RestResult, rate_limit::EndpointType, spot::private_client::RestClient};

const WITHDRAW_ADDRESS_LIST_ENDPOINT: &str = "/account/v1/withdraw/address/list";

/// Request parameters for getting withdraw address list (no parameters required)
#[derive(Debug, Serialize, Default)]
pub struct GetWithdrawAddressListRequest {}

/// Withdraw address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawAddress {
    /// Withdraw Address
    pub address: String,
    /// Memo
    pub memo: String,
    /// Remark
    pub remark: String,
    /// Address verify status
    /// - `0` = Unverified
    /// - `1` = Verified
    #[serde(rename = "verifyStatus")]
    pub verify_status: i32,
    /// Address Type
    /// - `0` = Standard Address
    /// - `1` = Universal Address
    /// - `2` = EVM Address
    #[serde(rename = "addressType")]
    pub address_type: i32,
    /// Network. The value is present only when the address type is a Standard address or Universal Address
    pub network: Option<String>,
    /// The value is present only when the address type is a Standard address
    pub currency: Option<String>,
}

/// Response for withdraw address list endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWithdrawAddressListResponse {
    /// Array of withdraw address data
    pub list: Vec<WithdrawAddress>,
}

impl RestClient {
    /// Withdraw Address
    ///
    /// Gets the user's withdraw address list
    ///
    /// [docs](https://developer-pro.bitmart.com/en/spot/#withdraw-address-keyed)
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters (empty struct)
    ///
    /// # Returns
    /// Withdraw address list information
    pub async fn get_withdraw_address_list(
        &self,
        request: GetWithdrawAddressListRequest,
    ) -> RestResult<GetWithdrawAddressListResponse> {
        self.send_get_signed_request(
            WITHDRAW_ADDRESS_LIST_ENDPOINT,
            &request,
            EndpointType::FundingAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_withdraw_address_list_request_default() {
        let request = GetWithdrawAddressListRequest::default();
        // This is an empty struct, just verify it can be created
        let _serialized = serde_json::to_string(&request).unwrap();
    }

    #[test]
    fn test_withdraw_address_structure() {
        let address = WithdrawAddress {
            address: "0x1121".to_string(),
            memo: "12".to_string(),
            remark: "12".to_string(),
            verify_status: 0,
            address_type: 0,
            network: Some("ETH".to_string()),
            currency: Some("ETH".to_string()),
        };

        assert_eq!(address.address, "0x1121");
        assert_eq!(address.memo, "12");
        assert_eq!(address.remark, "12");
        assert_eq!(address.verify_status, 0);
        assert_eq!(address.address_type, 0);
        assert_eq!(address.network, Some("ETH".to_string()));
        assert_eq!(address.currency, Some("ETH".to_string()));
    }

    #[test]
    fn test_withdraw_address_universal() {
        let address = WithdrawAddress {
            address: "0xUniversalAddress".to_string(),
            memo: "".to_string(),
            remark: "Universal Address".to_string(),
            verify_status: 1,
            address_type: 1, // Universal Address
            network: Some("EVM".to_string()),
            currency: None, // None for universal addresses
        };

        assert_eq!(address.address, "0xUniversalAddress");
        assert_eq!(address.verify_status, 1);
        assert_eq!(address.address_type, 1);
        assert_eq!(address.network, Some("EVM".to_string()));
        assert_eq!(address.currency, None);
    }

    #[test]
    fn test_withdraw_address_evm() {
        let address = WithdrawAddress {
            address: "0xEVMAddress".to_string(),
            memo: "".to_string(),
            remark: "EVM Address".to_string(),
            verify_status: 1,
            address_type: 2, // EVM Address
            network: None,   // None for EVM addresses
            currency: None,  // None for EVM addresses
        };

        assert_eq!(address.address, "0xEVMAddress");
        assert_eq!(address.verify_status, 1);
        assert_eq!(address.address_type, 2);
        assert_eq!(address.network, None);
        assert_eq!(address.currency, None);
    }

    #[test]
    fn test_withdraw_address_serialization_roundtrip() {
        let address = WithdrawAddress {
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            memo: "".to_string(),
            remark: "BTC Address".to_string(),
            verify_status: 1,
            address_type: 0,
            network: Some("BTC".to_string()),
            currency: Some("BTC".to_string()),
        };

        let serialized = serde_json::to_string(&address).unwrap();
        let deserialized: WithdrawAddress = serde_json::from_str(&serialized).unwrap();

        assert_eq!(address.address, deserialized.address);
        assert_eq!(address.memo, deserialized.memo);
        assert_eq!(address.remark, deserialized.remark);
        assert_eq!(address.verify_status, deserialized.verify_status);
        assert_eq!(address.address_type, deserialized.address_type);
        assert_eq!(address.network, deserialized.network);
        assert_eq!(address.currency, deserialized.currency);
    }

    #[test]
    fn test_get_withdraw_address_list_response_structure() {
        let response = GetWithdrawAddressListResponse {
            list: vec![
                WithdrawAddress {
                    address: "0x1121".to_string(),
                    memo: "12".to_string(),
                    remark: "12".to_string(),
                    verify_status: 0,
                    address_type: 0,
                    network: Some("ETH".to_string()),
                    currency: Some("ETH".to_string()),
                },
                WithdrawAddress {
                    address: "0xUniversalAddress".to_string(),
                    memo: "".to_string(),
                    remark: "Universal".to_string(),
                    verify_status: 1,
                    address_type: 1,
                    network: Some("EVM".to_string()),
                    currency: None,
                },
            ],
        };

        assert_eq!(response.list.len(), 2);
        assert_eq!(response.list[0].address, "0x1121");
        assert_eq!(response.list[1].address, "0xUniversalAddress");
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "list": [
                {
                    "currency": "ETH",
                    "network": "ETH",
                    "address": "0x1121",
                    "memo": "12",
                    "remark": "12",
                    "addressType": 0,
                    "verifyStatus": 0
                }
            ]
        }"#;

        let response: GetWithdrawAddressListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.list.len(), 1);
        assert_eq!(response.list[0].address, "0x1121");
        assert_eq!(response.list[0].memo, "12");
        assert_eq!(response.list[0].remark, "12");
        assert_eq!(response.list[0].verify_status, 0);
        assert_eq!(response.list[0].address_type, 0);
        assert_eq!(response.list[0].network, Some("ETH".to_string()));
        assert_eq!(response.list[0].currency, Some("ETH".to_string()));
    }
}
