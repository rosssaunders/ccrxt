use super::RestClient;
use crate::bitget::enums::*;
use crate::bitget::{Errors, RestResult};

use serde::{Deserialize, Serialize};

/// Withdraw
///
/// Coin withdrawals including on-chain withdrawals and internal transfers
/// (the address needs to be added in the address book on web).
///
/// Rate limit: 5 req/sec/UID

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawRequest {
    /// Coin name
    /// All coin names can be returned from Get Coin Info interface
    pub coin: String,

    /// The type of withdrawal
    #[serde(rename = "transferType")]
    pub transfer_type: WithdrawType,

    /// Withdrawal address
    /// When transferType is on_chain, it represents the chain address
    /// When transferType is internal_transfer, according the innerToType parameter,
    /// please input the UID, email or the mobile
    pub address: String,

    /// Chain network e.g. erc20, trc20, etc.
    /// This field must be passed when the transferType is on-chain.
    /// You can get the chain names via Get Coin Info interface
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,

    /// Type of address for internal withdrawals
    /// email: Email address
    /// mobile: Mobile phone number  
    /// uid: UID
    /// The default value is uid
    #[serde(rename = "innerToType", skip_serializing_if = "Option::is_none")]
    pub inner_to_type: Option<InnerTransferType>,

    /// This field is required when the value of the collection address type is mobile
    #[serde(rename = "areaCode", skip_serializing_if = "Option::is_none")]
    pub area_code: Option<String>,

    /// Address tag
    /// Some special coins need this field, e.g. EOS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Withdrawal amount
    /// The decimal places of withdrawal amount will be returned by the Get Coin Info interface
    pub size: String,

    /// Note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    /// Client Unique Customized Id
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,

    /// Support: bithumb, korbit, coinone
    #[serde(rename = "memberCode", skip_serializing_if = "Option::is_none")]
    pub member_code: Option<String>,

    /// Normal user: user, company: company
    #[serde(rename = "identityType", skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<IdentityType>,

    /// Company name
    #[serde(rename = "companyName", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,

    /// First Name
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Last name
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    pub data: WithdrawResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawResult {
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Custom order ID
    #[serde(rename = "clientOid")]
    pub client_oid: String,
}

impl RestClient {
    /// Withdraw
    ///
    /// Coin withdrawals including on-chain withdrawals and internal transfers.
    ///
    /// [API Documentation](https://www.bitget.com/api-doc/spot/withdraw/Withdraw)
    ///
    /// Rate limit: 5 req/sec/UID
    ///
    /// Returns a `RestResult<WithdrawResponse>` containing the withdrawal result or an error.
    pub async fn withdraw(&self, request: WithdrawRequest) -> RestResult<WithdrawResponse> {
        self.send_signed_request(
            "/api/v2/spot/wallet/withdrawal",
            reqwest::Method::POST,
            None,
            Some(
                &serde_json::to_string(&request)
                    .map_err(|e| Errors::Error(format!("Serialization error: {e}")))?,
            ),
            5,
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
    fn test_withdraw_request_serialization() {
        let request = WithdrawRequest {
            coin: "USDT".to_string(),
            transfer_type: WithdrawType::OnChain,
            address: "TJRyWwFs9wTFGZg3JbrVriFbNfCug5tDeC".to_string(),
            size: "0.01".to_string(),
            chain: Some("trc20".to_string()),
            client_oid: Some("my-withdraw-123".to_string()),
            inner_to_type: None,
            area_code: None,
            tag: None,
            remark: None,
            member_code: None,
            identity_type: None,
            company_name: None,
            first_name: None,
            last_name: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized request: {}", serialized);

        assert!(serialized.contains("\"coin\":\"USDT\""));
        assert!(serialized.contains("\"size\":\"0.01\""));
        assert!(serialized.contains("\"chain\":\"trc20\""));
    }

    #[test]
    fn test_withdraw_response_deserialization() {
        let json = r#"
        {
            "code": "00000",
            "msg": "success",
            "requestTime": 1695808949356,
            "data": {
                "orderId": "123",
                "clientOid": "my-withdraw-123"
            }
        }"#;

        let response: WithdrawResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert_eq!(response.data.order_id, "123");
        assert_eq!(response.data.client_oid, "my-withdraw-123");
    }

    #[tokio::test]
    async fn test_withdraw_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = WithdrawRequest {
            coin: "USDT".to_string(),
            transfer_type: WithdrawType::OnChain,
            address: "TJRyWwFs9wTFGZg3JbrVriFbNfCug5tDeC".to_string(),
            size: "0.01".to_string(),
            chain: Some("trc20".to_string()),
            inner_to_type: None,
            area_code: None,
            tag: None,
            remark: None,
            client_oid: None,
            member_code: None,
            identity_type: None,
            company_name: None,
            first_name: None,
            last_name: None,
        };

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.withdraw(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
