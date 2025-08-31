use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, PrivateRestClient as RestClient, RestResult};

const DEPOSIT_RISK_RECORDS_ENDPOINT: &str = "/openApi/wallets/v1/capital/deposit/riskRecords";

/// Request for deposit risk control records
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositRiskRecordsRequest {
    /// Execution window time, cannot be greater than 60000 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Current timestamp (required)
    pub timestamp: i64,
}

/// Deposit risk control record
#[derive(Debug, Clone, Deserialize)]
pub struct DepositRiskRecord {
    /// User ID
    pub uid: String,

    /// Currency name
    pub coin: String,

    /// Amount
    pub amount: String,

    /// Source address
    #[serde(rename = "sourceAddress")]
    pub source_address: String,

    /// Recharge address
    pub address: String,

    /// Creation time
    #[serde(rename = "insetTime")]
    pub insert_time: String,
}

/// Response for deposit risk control records
pub type GetDepositRiskRecordsResponse = Vec<DepositRiskRecord>;

impl RestClient {
    /// Deposit risk control records
    ///
    /// Used to query the recharge records in risk control for users and their sub-accounts.
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/wallet-api.html#Deposit%20risk%20control%20records)
    ///
    /// Rate limit: UID 2/s & IP rate limit group 2
    ///
    /// # Arguments
    /// * `request` - The deposit risk records request parameters
    ///
    /// # Returns
    /// A result containing the deposit risk records response or an error
    pub async fn get_deposit_risk_records(
        &self,
        request: &GetDepositRiskRecordsRequest,
    ) -> RestResult<GetDepositRiskRecordsResponse> {
        self.send_get_signed_request(
            DEPOSIT_RISK_RECORDS_ENDPOINT,
            request,
            EndpointType::AccountApiGroup2,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_risk_records_request_serialization() {
        let request = GetDepositRiskRecordsRequest {
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_deposit_risk_records_minimal_request() {
        let request = GetDepositRiskRecordsRequest {
            recv_window: None,
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1640995200000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_deposit_risk_record_deserialization() {
        let json = r#"[
            {
                "uid": "123456789",
                "coin": "USDT",
                "amount": "100.50",
                "sourceAddress": "0x1234567890abcdef1234567890abcdef12345678",
                "address": "0xabcdef1234567890abcdef1234567890abcdef12",
                "insetTime": "2023-08-15T10:30:00Z"
            },
            {
                "uid": "987654321",
                "coin": "BTC",
                "amount": "0.001",
                "sourceAddress": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                "address": "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
                "insetTime": "2023-08-15T11:45:00Z"
            }
        ]"#;

        let response: GetDepositRiskRecordsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        let first_record = &response[0];
        assert_eq!(first_record.uid, "123456789");
        assert_eq!(first_record.coin, "USDT");
        assert_eq!(first_record.amount, "100.50");
        assert_eq!(
            first_record.source_address,
            "0x1234567890abcdef1234567890abcdef12345678"
        );
        assert_eq!(
            first_record.address,
            "0xabcdef1234567890abcdef1234567890abcdef12"
        );
        assert_eq!(first_record.insert_time, "2023-08-15T10:30:00Z");

        let second_record = &response[1];
        assert_eq!(second_record.uid, "987654321");
        assert_eq!(second_record.coin, "BTC");
        assert_eq!(second_record.amount, "0.001");
        assert_eq!(
            second_record.source_address,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(
            second_record.address,
            "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"
        );
        assert_eq!(second_record.insert_time, "2023-08-15T11:45:00Z");
    }
}
