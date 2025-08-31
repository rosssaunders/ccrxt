use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const WITHDRAW_STATUS_ENDPOINT: &str = "/wallet/withdraw_status";

/// Request parameters for querying withdrawal status
#[derive(Debug, Clone, Serialize)]
pub struct WithdrawStatusRequest {
    /// Currency of the withdrawal
    pub currency: String,
}

/// Withdrawal status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawStatus {
    /// Currency
    pub currency: String,

    /// Withdrawal ID
    pub id: String,

    /// Withdrawal status
    pub status: String,

    /// Withdrawal amount
    pub amount: String,

    /// Destination address
    pub address: String,

    /// Transaction hash (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,

    /// Withdrawal fee
    pub fee: String,

    /// Withdrawal timestamp
    pub create_time: i64,
}

impl RestClient {
    /// Query Withdrawal Status
    ///
    /// Query the current status of withdrawal requests.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#query-withdrawal-status)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `req` - Request with currency to query withdrawal status for
    ///
    /// # Returns
    /// List of withdrawal status information
    pub async fn get_withdraw_status(
        &self,
        req: WithdrawStatusRequest,
    ) -> RestResult<Vec<WithdrawStatus>> {
        self.send_get_request(WITHDRAW_STATUS_ENDPOINT, Some(&req))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdraw_status_request_serialization() {
        let request = WithdrawStatusRequest {
            currency: "BTC".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
    }

    #[test]
    fn test_withdraw_status_request_different_currencies() {
        let currencies = vec!["BTC", "ETH", "USDT", "SOL", "BNB"];

        for currency in currencies {
            let request = WithdrawStatusRequest {
                currency: currency.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], currency);
        }
    }

    #[test]
    fn test_withdraw_status_response_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "id": "withdraw_123456",
            "status": "pending",
            "amount": "0.5",
            "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
            "txid": "tx_abcdef123456789",
            "fee": "0.0005",
            "create_time": 1640995200
        }"#;

        let status: WithdrawStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.currency, "BTC");
        assert_eq!(status.id, "withdraw_123456");
        assert_eq!(status.status, "pending");
        assert_eq!(status.amount, "0.5");
        assert_eq!(status.address, "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh");
        assert_eq!(status.txid.as_ref().unwrap(), "tx_abcdef123456789");
        assert_eq!(status.fee, "0.0005");
        assert_eq!(status.create_time, 1640995200);
    }

    #[test]
    fn test_withdraw_status_response_without_txid() {
        let json = r#"{
            "currency": "ETH",
            "id": "withdraw_789012",
            "status": "processing",
            "amount": "1.2",
            "address": "0x123456789abcdef123456789abcdef123456789a",
            "txid": null,
            "fee": "0.002",
            "create_time": 1640995800
        }"#;

        let status: WithdrawStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.currency, "ETH");
        assert_eq!(status.id, "withdraw_789012");
        assert_eq!(status.status, "processing");
        assert_eq!(status.amount, "1.2");
        assert_eq!(status.address, "0x123456789abcdef123456789abcdef123456789a");
        assert!(status.txid.is_none());
        assert_eq!(status.fee, "0.002");
        assert_eq!(status.create_time, 1640995800);
    }

    #[test]
    fn test_withdraw_status_types() {
        let statuses = vec!["pending", "processing", "confirmed", "failed", "cancelled"];

        for status_type in statuses {
            let json = format!(
                r#"{{
                "currency": "USDT",
                "id": "withdraw_test",
                "status": "{}",
                "amount": "100.0",
                "address": "TRX123456789abcdef123456789abcdef123456789",
                "fee": "1.0",
                "create_time": 1640995200
            }}"#,
                status_type
            );

            let status: WithdrawStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status.status, status_type);
        }
    }

    #[test]
    fn test_withdraw_status_endpoint_constant() {
        assert_eq!(WITHDRAW_STATUS_ENDPOINT, "/wallet/withdraw_status");
    }

    #[test]
    fn test_high_precision_amounts() {
        let json = r#"{
            "currency": "BTC",
            "id": "withdraw_precision_test",
            "status": "confirmed",
            "amount": "0.123456789012345678",
            "address": "bc1qtest",
            "fee": "0.000123456789012345",
            "create_time": 1640995200
        }"#;

        let status: WithdrawStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.amount, "0.123456789012345678");
        assert_eq!(status.fee, "0.000123456789012345");

        // Verify precision is maintained
        assert_eq!(status.amount.len(), 20); // 18 decimal places + "0."
        assert_eq!(status.fee.len(), 20); // 18 decimal places + "0."
    }

    #[test]
    fn test_serialization_round_trip() {
        let original = WithdrawStatus {
            currency: "BTC".to_string(),
            id: "withdraw_test".to_string(),
            status: "confirmed".to_string(),
            amount: "0.5".to_string(),
            address: "bc1qtest".to_string(),
            txid: Some("tx_test".to_string()),
            fee: "0.0005".to_string(),
            create_time: 1640995200,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: WithdrawStatus = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.status, original.status);
        assert_eq!(deserialized.amount, original.amount);
        assert_eq!(deserialized.address, original.address);
        assert_eq!(deserialized.txid, original.txid);
        assert_eq!(deserialized.fee, original.fee);
        assert_eq!(deserialized.create_time, original.create_time);
    }

    #[test]
    fn test_different_currency_addresses() {
        let test_cases = vec![
            ("BTC", "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"),
            ("ETH", "0x123456789abcdef123456789abcdef123456789a"),
            ("USDT", "TRX123456789abcdef123456789abcdef123456789"),
            ("LTC", "ltc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"),
            ("DOGE", "DH5yaieqoZN36fDVciNyRueRGvGLR3mr7L"),
        ];

        for (currency, address) in test_cases {
            let json = format!(
                r#"{{
                "currency": "{}",
                "id": "withdraw_{}",
                "status": "confirmed",
                "amount": "1.0",
                "address": "{}",
                "fee": "0.001",
                "create_time": 1640995200
            }}"#,
                currency,
                currency.to_lowercase(),
                address
            );

            let status: WithdrawStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status.currency, currency);
            assert_eq!(status.address, address);
        }
    }

    #[test]
    fn test_timestamp_validation() {
        let json = r#"{
            "currency": "BTC",
            "id": "withdraw_time_test",
            "status": "confirmed",
            "amount": "0.1",
            "address": "bc1qtest",
            "fee": "0.0001",
            "create_time": 1704067200
        }"#;

        let status: WithdrawStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.create_time, 1704067200);

        // Verify timestamp is reasonable (after 2020)
        assert!(status.create_time > 1577836800); // Jan 1, 2020
        // Verify timestamp is reasonable (before 2030)
        assert!(status.create_time < 1893456000); // Jan 1, 2030
    }

    #[test]
    fn test_minimal_withdraw_status() {
        let json = r#"{
            "currency": "BTC",
            "id": "minimal_test",
            "status": "pending",
            "amount": "0.001",
            "address": "bc1qtest",
            "fee": "0.0001",
            "create_time": 1640995200
        }"#;

        let status: WithdrawStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.currency, "BTC");
        assert_eq!(status.id, "minimal_test");
        assert_eq!(status.status, "pending");
        assert_eq!(status.amount, "0.001");
        assert_eq!(status.address, "bc1qtest");
        assert!(status.txid.is_none());
        assert_eq!(status.fee, "0.0001");
        assert_eq!(status.create_time, 1640995200);
    }
}
