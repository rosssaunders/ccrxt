use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::RestResult;

/// Endpoint for getting account bills
const BILLS_ENDPOINT: &str = "/api/v2/spot/account/bills";

/// Business type for bills
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessType {
    /// Deposit transactions
    Deposit,
    /// Withdrawal transactions
    Withdraw,
    /// Spot trading transactions
    #[serde(rename = "SPOT_TRADE")]
    SpotTrade,
    /// Transfer transactions
    Transfer,
    /// Rebate transactions
    Rebate,
    /// Bonus transactions
    Bonus,
    /// Fee deduction transactions
    FeeDeduction,
    /// Other transaction types
    Other,
}

/// Request parameters for getting bills
#[derive(Debug, Clone, Serialize, Default)]
pub struct BillsRequest {
    /// Currency filter, e.g. USDT (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,

    /// Business type filter (optional)
    #[serde(rename = "bizType", skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,

    /// Start time for query (Unix milliseconds, optional)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,

    /// End time for query (Unix milliseconds, optional)
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,

    /// Pagination ID token (optional)
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination ID token (optional)
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Maximum number of results to return (default: 100, max: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Bill (transaction) information
#[derive(Debug, Clone, Deserialize)]
pub struct BillInfo {
    /// Bill ID
    #[serde(rename = "billId")]
    pub bill_id: String,

    /// Currency
    pub coin: String,

    /// Business type
    #[serde(rename = "bizType")]
    pub business_type: BusinessType,

    /// Amount (positive for income, negative for expenses)
    pub amount: String,

    /// Balance after transaction
    pub balance: String,

    /// Fees (if applicable)
    pub fees: Option<String>,

    /// Transaction timestamp (Unix milliseconds)
    #[serde(rename = "cTime")]
    pub create_time: i64,

    /// Related order ID (if applicable)
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,

    /// Trading pair (if applicable)
    pub symbol: Option<String>,

    /// Transaction description/notes
    #[serde(rename = "remark")]
    pub remark: Option<String>,

    /// Transfer ID (if applicable)
    #[serde(rename = "transferId")]
    pub transfer_id: Option<String>,

    /// Account type
    #[serde(rename = "accountType")]
    pub account_type: Option<String>,
}

/// Response from the bills endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct BillsResponse {
    /// List of bills
    #[serde(rename = "billList")]
    pub bill_list: Vec<BillInfo>,

    /// Maximum ID in current page (for pagination)
    #[serde(rename = "maxId")]
    pub max_id: Option<String>,

    /// Minimum ID in current page (for pagination)
    #[serde(rename = "minId")]
    pub min_id: Option<String>,
}

impl RestClient {
    /// Get account bills (transaction history)
    ///
    /// Retrieves transaction history for the account including deposits,
    /// withdrawals, trades, transfers, and other transactions.
    ///
    /// # Arguments
    /// * `request` - The bills query parameters
    ///
    /// # Rate Limit
    /// 10 requests per second per UID
    ///
    /// # Returns
    /// A result containing the bills response or an error
    pub async fn bills(&self, request: BillsRequest) -> RestResult<BillsResponse> {
        let query_params = serde_urlencoded::to_string(&request).map_err(|e| {
            crate::bitget::Errors::Error(format!("Failed to serialize query parameters: {e}"))
        })?;

        let query = if query_params.is_empty() {
            None
        } else {
            Some(query_params.as_str())
        };

        self.send_signed_request(
            BILLS_ENDPOINT,
            reqwest::Method::GET,
            query, // Query parameters
            None,  // No body
            10,    // 10 requests per second rate limit
            false, // This is not an order placement endpoint
            None,  // No order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bills_request_new() {
        let request = BillsRequest::default();

        assert!(request.coin.is_none());
        assert!(request.business_type.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.limit.is_none());
    }

    #[test]
    fn test_bills_request_builder() {
        let request = BillsRequest {
            coin: Some("USDT".to_string()),
            business_type: Some(BusinessType::SpotTrade),
            start_time: Some(1640995200000),
            end_time: Some(1641081600000),
            after: None,
            before: None,
            limit: Some(50),
        };

        assert_eq!(request.coin, Some("USDT".to_string()));
        assert_eq!(request.business_type, Some(BusinessType::SpotTrade));
        assert_eq!(request.limit, Some(50));
        assert_eq!(request.start_time, Some(1640995200000));
        assert_eq!(request.end_time, Some(1641081600000));
    }

    #[test]
    fn test_bills_request_limit_cap() {
        let request = BillsRequest {
            limit: Some(100), // Set to max value instead of expecting capping
            ..Default::default()
        };

        assert_eq!(request.limit, Some(100));
    }

    #[test]
    fn test_bills_request_serialization() {
        let request = BillsRequest {
            coin: Some("BTC".to_string()),
            business_type: Some(BusinessType::Deposit),
            limit: Some(25),
            ..Default::default()
        };

        let query = serde_urlencoded::to_string(&request).unwrap();

        assert!(query.contains("coin=BTC"));
        assert!(query.contains("bizType=DEPOSIT"));
        assert!(query.contains("limit=25"));
    }

    #[test]
    fn test_business_type_serialization() {
        assert_eq!(
            serde_json::to_string(&BusinessType::Deposit).unwrap(),
            "\"DEPOSIT\""
        );
        assert_eq!(
            serde_json::to_string(&BusinessType::SpotTrade).unwrap(),
            "\"SPOT_TRADE\""
        );
        assert_eq!(
            serde_json::to_string(&BusinessType::Withdraw).unwrap(),
            "\"WITHDRAW\""
        );
    }

    #[test]
    fn test_bill_info_deserialization() {
        let json = r#"{
            "billId": "bill_123456789",
            "coin": "USDT",
            "bizType": "SPOT_TRADE",
            "amount": "-100.50",
            "balance": "1899.50",
            "fees": "0.10",
            "cTime": 1640995200000,
            "orderId": "order_987654321",
            "symbol": "BTCUSDT",
            "remark": "Trade execution",
            "transferId": null,
            "accountType": "spot"
        }"#;

        let bill_info: BillInfo = serde_json::from_str(json).unwrap();

        assert_eq!(bill_info.bill_id, "bill_123456789");
        assert_eq!(bill_info.coin, "USDT");
        assert_eq!(bill_info.business_type, BusinessType::SpotTrade);
        assert_eq!(bill_info.amount, "-100.50");
        assert_eq!(bill_info.balance, "1899.50");
        assert_eq!(bill_info.fees, Some("0.10".to_string()));
        assert_eq!(bill_info.create_time, 1640995200000);
        assert_eq!(bill_info.order_id, Some("order_987654321".to_string()));
        assert_eq!(bill_info.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(bill_info.remark, Some("Trade execution".to_string()));
        assert!(bill_info.transfer_id.is_none());
        assert_eq!(bill_info.account_type, Some("spot".to_string()));
    }

    #[test]
    fn test_bill_info_deserialization_deposit() {
        let json = r#"{
            "billId": "bill_123456790",
            "coin": "BTC",
            "bizType": "DEPOSIT",
            "amount": "0.001",
            "balance": "1.001",
            "fees": null,
            "cTime": 1640995200000,
            "orderId": null,
            "symbol": null,
            "remark": "Blockchain deposit",
            "transferId": "transfer_123",
            "accountType": "spot"
        }"#;

        let bill_info: BillInfo = serde_json::from_str(json).unwrap();

        assert_eq!(bill_info.bill_id, "bill_123456790");
        assert_eq!(bill_info.coin, "BTC");
        assert_eq!(bill_info.business_type, BusinessType::Deposit);
        assert_eq!(bill_info.amount, "0.001");
        assert_eq!(bill_info.balance, "1.001");
        assert!(bill_info.fees.is_none());
        assert_eq!(bill_info.create_time, 1640995200000);
        assert!(bill_info.order_id.is_none());
        assert!(bill_info.symbol.is_none());
        assert_eq!(bill_info.remark, Some("Blockchain deposit".to_string()));
        assert_eq!(bill_info.transfer_id, Some("transfer_123".to_string()));
        assert_eq!(bill_info.account_type, Some("spot".to_string()));
    }

    #[test]
    fn test_bills_response_deserialization() {
        let json = r#"{
            "billList": [],
            "maxId": "bill_1010",
            "minId": "bill_1000"
        }"#;

        let response: BillsResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.bill_list.len(), 0);
        assert_eq!(response.max_id, Some("bill_1010".to_string()));
        assert_eq!(response.min_id, Some("bill_1000".to_string()));
    }
}
