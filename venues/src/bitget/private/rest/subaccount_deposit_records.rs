use crate::bitget::{
    BitgetRestClient,
};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Request for getting subaccount deposit records
#[derive(Debug, Clone, Serialize)]
pub struct GetSubaccountDepositRecordsRequest {
    /// Sub Account UID
    #[serde(rename = "subUid")]
    pub sub_uid: String,
    /// Coin name, e.g. USDT (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Record start time (Unix millisecond timestamp) (optional)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Record end time (Unix millisecond timestamp) (optional)
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Request content before this ID (older data) (optional)
    #[serde(rename = "idLessThan", skip_serializing_if = "Option::is_none")]
    pub id_less_than: Option<String>,
    /// Number of entries per page (default: 20, max: 100) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Subaccount deposit record
#[derive(Debug, Clone, Deserialize)]
pub struct SubaccountDepositRecord {
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Trade ID (on-chain hash or internal trade ID)
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    /// Token name
    pub coin: String,
    /// Customized order ID
    #[serde(rename = "clientOid")]
    pub client_oid: Option<String>,
    /// Quantity
    pub size: String,
    /// Deposit status (pending, fail, success)
    pub status: String,
    /// Deposit initiator address
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    /// Coin receiver address
    #[serde(rename = "toAddress")]
    pub to_address: String,
    /// Deposit network
    pub chain: String,
    /// Number of confirmed blocks
    pub confirm: Option<String>,
    /// Deposit type (on_chain, internal_transfer)
    pub dest: String,
    /// Tag
    pub tag: String,
    /// Creation time in ms
    #[serde(rename = "cTime")]
    pub c_time: String,
    /// Update time in ms
    #[serde(rename = "uTime")]
    pub u_time: String,
}

/// Response for getting subaccount deposit records
#[derive(Debug, Clone, Deserialize)]
pub struct GetSubaccountDepositRecordsResponse {
    /// List of deposit records
    pub data: Vec<SubaccountDepositRecord>,
}

impl GetSubaccountDepositRecordsRequest {
    /// Create a new request builder
    pub fn builder() -> GetSubaccountDepositRecordsRequestBuilder {
        GetSubaccountDepositRecordsRequestBuilder::default()
    }
}

impl BitgetRequest for GetSubaccountDepositRecordsRequest {
    type Response = GetSubaccountDepositRecordsResponse;

    fn path(&self) -> String {
        "/api/v2/spot/wallet/subaccount-deposit-records".to_string()
    }

    fn method(&self) -> String {
        "GET".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

/// Builder for GetSubaccountDepositRecordsRequest
#[derive(Debug, Default)]
pub struct GetSubaccountDepositRecordsRequestBuilder {
    sub_uid: Option<String>,
    coin: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    id_less_than: Option<String>,
    limit: Option<String>,
}

impl GetSubaccountDepositRecordsRequestBuilder {
    /// Set the sub-account UID
    pub fn sub_uid(mut self, sub_uid: impl Into<String>) -> Self {
        self.sub_uid = Some(sub_uid.into());
        self
    }

    /// Set the coin name (optional)
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }

    /// Set the start time (optional)
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }

    /// Set the end time (optional)
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// Set the ID less than parameter (optional)
    pub fn id_less_than(mut self, id_less_than: impl Into<String>) -> Self {
        self.id_less_than = Some(id_less_than.into());
        self
    }

    /// Set the limit (optional)
    pub fn limit(mut self, limit: impl Into<String>) -> Self {
        self.limit = Some(limit.into());
        self
    }

    /// Build the request
    pub fn build(self) -> GetSubaccountDepositRecordsRequest {
        GetSubaccountDepositRecordsRequest {
            sub_uid: self.sub_uid.expect("sub_uid is required"),
            coin: self.coin,
            start_time: self.start_time,
            end_time: self.end_time,
            id_less_than: self.id_less_than,
            limit: self.limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder() {
        let request = GetSubaccountDepositRecordsRequest::builder()
            .sub_uid("12121212")
            .coin("USDT")
            .id_less_than("1111120137173336063")
            .limit("5")
            .build();

        assert_eq!(request.sub_uid, "12121212");
        assert_eq!(request.coin, Some("USDT".to_string()));
        assert_eq!(request.id_less_than, Some("1111120137173336063".to_string()));
        assert_eq!(request.limit, Some("5".to_string()));
    }

    #[test]
    fn test_request_builder_required_only() {
        let request = GetSubaccountDepositRecordsRequest::builder()
            .sub_uid("12121212")
            .build();

        assert_eq!(request.sub_uid, "12121212");
        assert_eq!(request.coin, None);
        assert_eq!(request.start_time, None);
        assert_eq!(request.end_time, None);
        assert_eq!(request.id_less_than, None);
        assert_eq!(request.limit, None);
    }
}
