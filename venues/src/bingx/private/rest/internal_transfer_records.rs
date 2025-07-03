use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// Request to get internal transfer records
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferRecordsRequest {
    /// Transfer ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    /// Asset symbol (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// Start time in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// End time in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Page size (default: 10, max: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

/// Internal transfer record
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferRecord {
    /// Transfer ID
    pub transfer_id: String,
    /// Asset symbol
    pub asset: String,
    /// Transfer amount
    pub amount: Decimal,
    /// From account type
    pub from_account_type: String,
    /// To account type
    pub to_account_type: String,
    /// Transfer status
    pub status: String,
    /// Transfer timestamp
    pub timestamp: i64,
    /// Transaction fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Decimal>,
}

/// Response for internal transfer records
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferRecordsResponse {
    /// Success indicator
    pub success: bool,
    /// Transfer records data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InternalTransferRecordsData>,
}

/// Internal transfer records data
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransferRecordsData {
    /// List of transfer records
    pub records: Vec<InternalTransferRecord>,
    /// Total count
    pub total_count: i32,
    /// Current page
    pub page: i32,
    /// Page size
    pub size: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_internal_transfer_records_request_serialization() {
        let request = InternalTransferRecordsRequest {
            transfer_id: Some("TXN123456".to_string()),
            asset: Some("USDT".to_string()),
            start_time: Some(1640995200000),
            end_time: Some(1641081600000),
            page: Some(1),
            size: Some(20),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"transferId\":\"TXN123456\""));
        assert!(json.contains("\"asset\":\"USDT\""));
        assert!(json.contains("\"startTime\":1640995200000"));
        assert!(json.contains("\"endTime\":1641081600000"));
        assert!(json.contains("\"page\":1"));
        assert!(json.contains("\"size\":20"));
    }

    #[test]
    fn test_minimal_request() {
        let request = InternalTransferRecordsRequest {
            transfer_id: None,
            asset: None,
            start_time: None,
            end_time: None,
            page: None,
            size: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_internal_transfer_records_response_deserialization() {
        let json = r#"
        {
            "success": true,
            "data": {
                "records": [
                    {
                        "transferId": "TXN123456",
                        "asset": "USDT",
                        "amount": "100.50",
                        "fromAccountType": "SPOT",
                        "toAccountType": "FUTURES",
                        "status": "SUCCESS",
                        "timestamp": 1640995200000,
                        "fee": "0.1"
                    },
                    {
                        "transferId": "TXN123457",
                        "asset": "BTC",
                        "amount": "0.001",
                        "fromAccountType": "FUTURES",
                        "toAccountType": "SPOT",
                        "status": "SUCCESS",
                        "timestamp": 1640995300000
                    }
                ],
                "totalCount": 2,
                "page": 1,
                "size": 10
            }
        }
        "#;

        let response: InternalTransferRecordsResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        
        let data = response.data.unwrap();
        assert_eq!(data.records.len(), 2);
        assert_eq!(data.total_count, 2);
        assert_eq!(data.page, 1);
        assert_eq!(data.size, 10);

        let first_record = &data.records[0];
        assert_eq!(first_record.transfer_id, "TXN123456");
        assert_eq!(first_record.asset, "USDT");
        assert_eq!(first_record.amount, dec!(100.50));
        assert_eq!(first_record.from_account_type, "SPOT");
        assert_eq!(first_record.to_account_type, "FUTURES");
        assert_eq!(first_record.status, "SUCCESS");
        assert_eq!(first_record.timestamp, 1640995200000);
        assert_eq!(first_record.fee, Some(dec!(0.1)));

        let second_record = &data.records[1];
        assert_eq!(second_record.transfer_id, "TXN123457");
        assert_eq!(second_record.asset, "BTC");
        assert_eq!(second_record.amount, dec!(0.001));
        assert!(second_record.fee.is_none());
    }
}
