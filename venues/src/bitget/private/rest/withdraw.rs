use crate::bitget::{BitgetRestClient, enums::*, error::BitgetError};
use reqwest::Method;
use rest::BitgetRequest;
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

impl WithdrawRequest {
    pub fn new(
        coin: impl Into<String>,
        transfer_type: WithdrawType,
        address: impl Into<String>,
        size: impl Into<String>,
    ) -> Self {
        Self {
            coin: coin.into(),
            transfer_type,
            address: address.into(),
            chain: None,
            inner_to_type: None,
            area_code: None,
            tag: None,
            size: size.into(),
            remark: None,
            client_oid: None,
            member_code: None,
            identity_type: None,
            company_name: None,
            first_name: None,
            last_name: None,
        }
    }

    pub fn chain(mut self, chain: impl Into<String>) -> Self {
        self.chain = Some(chain.into());
        self
    }

    pub fn inner_to_type(mut self, inner_to_type: InnerTransferType) -> Self {
        self.inner_to_type = Some(inner_to_type);
        self
    }

    pub fn area_code(mut self, area_code: impl Into<String>) -> Self {
        self.area_code = Some(area_code.into());
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    pub fn remark(mut self, remark: impl Into<String>) -> Self {
        self.remark = Some(remark.into());
        self
    }

    pub fn client_oid(mut self, client_oid: impl Into<String>) -> Self {
        self.client_oid = Some(client_oid.into());
        self
    }

    pub fn member_code(mut self, member_code: impl Into<String>) -> Self {
        self.member_code = Some(member_code.into());
        self
    }

    pub fn identity_type(mut self, identity_type: IdentityType) -> Self {
        self.identity_type = Some(identity_type);
        self
    }

    pub fn company_name(mut self, company_name: impl Into<String>) -> Self {
        self.company_name = Some(company_name.into());
        self
    }

    pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = Some(first_name.into());
        self
    }

    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }
}

impl BitgetRequest for WithdrawRequest {
    type Response = WithdrawResponse;

    fn path(&self) -> String {
        "/api/v2/spot/wallet/withdrawal".to_string()
    }

    fn method(&self) -> String {
        "POST".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

impl BitgetRestClient {
    /// Withdraw
    ///
    /// Coin withdrawals including on-chain withdrawals and internal transfers.
    pub async fn withdraw(
        &self,
        request: WithdrawRequest,
    ) -> Result<WithdrawResponse, BitgetError> {
        self.send_request(&request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdraw_request_serialization() {
        let request = WithdrawRequest::new(
            "USDT",
            WithdrawType::OnChain,
            "TJRyWwFs9wTFGZg3JbrVriFbNfCug5tDeC",
            "0.01",
        )
        .chain("trc20")
        .client_oid("my-withdraw-123");

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
        let _request = WithdrawRequest::new(
            "USDT",
            WithdrawType::OnChain,
            "TJRyWwFs9wTFGZg3JbrVriFbNfCug5tDeC",
            "0.01",
        )
        .chain("trc20");

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.withdraw(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
